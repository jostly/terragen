extern crate kiss3d;
extern crate glfw;
extern crate nalgebra as na;
extern crate rand;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate stopwatch;

mod terrain;
mod math;

use na::{Vector3, UnitQuaternion, Point3, Point2};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;
use kiss3d::scene::SceneNode;
use kiss3d::resource::Mesh;

use glfw::{Action, Key, WindowEvent};

use stopwatch::Stopwatch;

use terrain::Terrain;
use math::Vec3;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

fn main() {

    env_logger::init().unwrap();

    let mut ico = Terrain::new();
    for _ in 0..7 {
        ico.subdivide();
    }

    let mut window = Window::new("Terragen");

    let eye = Point3::new(0.0, 2.0, 5.0);
    let at = Point3::origin();
    let mut arc_ball = ArcBall::new(eye, at);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.001);

    let mut grp = window.add_group();
    let mut c = add_mesh(&mut grp, generate_dual(&ico));

    while window.render_with_camera(&mut arc_ball) {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                    println!("Subdividing a level {} terrain", ico.current_level());
                    window.remove(&mut c);
                    let mut sw = Stopwatch::start_new();
                    ico.subdivide();
                    println!("Subdivision took {}ms", sw.elapsed_ms());
                    // (2099 ms, lvl 6), (8685 ms, lvl 7)
                    sw.restart();
                    let mesh = generate_dual(&ico);
                    println!("Generating mesh took {}ms", sw.elapsed_ms());
                    sw.restart();
                    c = add_mesh(&mut grp, mesh);
                    println!("Adding mesh took {}ms", sw.elapsed_ms());
                    event.inhibited = true
                }
                _ => {}
            }
        }
        grp.prepend_to_local_rotation(&rot);
    }
}

fn add_mesh(parent: &mut SceneNode, mesh: Mesh) -> SceneNode {
    let mut c = parent.add_mesh(Rc::new(RefCell::new(mesh)), Vector3::new(1.0, 1.0, 1.0));

    c.set_color(1.0, 1.0, 1.0);
    c.set_texture_from_file(&Path::new("media/height_ramp.png"), "colour_ramp");

    c
}

impl<'a> From<&'a Vec3<f32>> for Vector3<f32> {
    fn from(v: &'a Vec3<f32>) -> Self {
        Vector3::new(v.x, v.y, v.z)
    }
}

impl<'a> From<&'a Vec3<f32>> for Point3<f32> {
    fn from(v: &'a Vec3<f32>) -> Self {
        Point3::new(v.x, v.y, v.z)
    }
}

/*
fn generate_regular(ico: &Terrain) -> Mesh {
    let ico_faces = &ico.faces;
    let ico_vertices = &ico.nodes;
    let num_vertices = ico_faces.len() * 3;
    let (min_elev, max_elev) = ico.calculate_elevations();
    let elev_scale = max_elev - min_elev;
    let mut vertices = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut texcoords = Vec::with_capacity(num_vertices);
    let mut faces = Vec::with_capacity(ico_faces.len());

    let mut vert_index = 0u32;
    for f in ico_faces.iter() {
        for idx in f.points.iter() {
            let ref vert = ico_vertices[*idx as usize];
            let elevation = (vert.elevation - min_elev) / elev_scale;
            let vertex_scale = (elevation.powi(2) - 0.5) * 0.02;
            let vertex = Point3::from_coordinates(vert.point.coords * (1.0 + vertex_scale));

            vertices.push(vertex);
            //let normal = normalize(&vert.point.coords);
            let normal = ico.normal(&f.points);
            normals.push(normal);
            let uv = Point2::new(1.0 - elevation, 0.0);
            texcoords.push(uv);
        }
        faces.push(Point3::new(vert_index, vert_index + 1, vert_index + 2));
        vert_index += 3;
    }

    Mesh::new(vertices, faces, Some(normals), Some(texcoords), false)
}
*/
/*

  Generated vectors @ 0 ms
  Calculated min/max elev @ 8 ms
  Built node -> face index @ 214 ms
  Built node -> edge index @ 441 ms
  Built edge -> face index @ 702 ms
  Built midpoint registry @ 1547 ms
  Generated mesh in 4602 ms
    Segment A: 187 ms
    Segment B: 3796 ms
      Segment B:0: 31 ms
      Segment B:1: 416 ms
      Segment B:2: 547 ms
      Segment B:3: 1307 ms
      Segment B:4: 212 ms
    Segment C: 578 ms
    Capacity mesh_faces:     983040 / 983052
    Capacity mesh_vertices:  1146882 / 1146894
    Capacity mesh_normals:   1146882 / 1146894
    Capacity mesh_texcoords: 1146882 / 1146894
  Creating mesh object in 0 ms

 */

fn generate_dual(terr: &Terrain) -> Mesh {
    println!("  Generator started...");
    let mut sw = Stopwatch::start_new();

    let num_nodes = terr.nodes.len();
    let num_edges = terr.edges.len();
    let num_faces = terr.faces.len();

    let mut mesh_faces = Vec::with_capacity(num_nodes * 6 - 12);
    let mut mesh_vertices = Vec::with_capacity(num_nodes * 7 - 12);
    let mut mesh_normals = Vec::with_capacity(mesh_vertices.capacity());
    let mut mesh_texcoords = Vec::with_capacity(mesh_vertices.capacity());

    println!("    Capacity mesh_faces:     {} / {}",
             mesh_faces.len(),
             mesh_faces.capacity());
    println!("    Capacity mesh_vertices:  {} / {}",
             mesh_vertices.len(),
             mesh_vertices.capacity());
    println!("    Capacity mesh_normals:   {} / {}",
             mesh_normals.len(),
             mesh_normals.capacity());
    println!("    Capacity mesh_texcoords: {} / {}",
             mesh_texcoords.len(),
             mesh_texcoords.capacity());

    println!("  Generated vectors @ {} ms", sw.elapsed_ms());

    let (min_elev, max_elev) = terr.calculate_elevations();
    let elev_scale = max_elev - min_elev;

    println!("  Calculated min/max elev @ {} ms", sw.elapsed_ms()); // (7 ms)

    // Build map of node index -> faces
    let node_to_faces = {
        let mut node_to_faces = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            node_to_faces.push(Vec::with_capacity(6));
        }

        for (i, face) in terr.faces.iter().enumerate() {
            for v in face.points.iter() {
                node_to_faces[*v as usize].push(i);
            }
        }
        node_to_faces
    };

    println!("  Built node -> face index @ {} ms", sw.elapsed_ms()); // (209 ms)

    // Build map of node index -> edges
    let node_to_edges = {
        let mut node_to_edges = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            node_to_edges.push(Vec::with_capacity(6));
        }

        for (i, edge) in terr.edges.iter().enumerate() {
            node_to_edges[edge.a as usize].push(i);
            node_to_edges[edge.b as usize].push(i);
        }
        node_to_edges
    };

    println!("  Built node -> edge index @ {} ms", sw.elapsed_ms()); // (360 ms)

    // Build map of edge index -> faces
    let edge_to_faces = {
        let mut edge_to_faces = Vec::with_capacity(num_edges);
        for _ in 0..num_edges {
            edge_to_faces.push(Vec::with_capacity(2));
        }

        for (i, face) in terr.faces.iter().enumerate() {
            for v in face.edges.iter() {
                edge_to_faces[*v as usize].push(i);
            }
        }
        edge_to_faces
    };

    println!("  Built edge -> face index @ {} ms", sw.elapsed_ms()); // (624 ms)

    let face_midpoints = {
        let mut face_midpoints = Vec::with_capacity(num_faces);
        for face in terr.faces.iter() {
            face_midpoints.push(terr.face_midpoint(face));
        }
        face_midpoints
    };

    println!("  Built midpoint registry @ {} ms", sw.elapsed_ms()); // (862 ms)

    let mut st_a = Stopwatch::new();
    let mut st_b = Stopwatch::new();
    let mut st_b_0 = Stopwatch::new();
    let mut st_b_1 = Stopwatch::new();
    let mut st_b_2 = Stopwatch::new();
    let mut st_b_3 = Stopwatch::new();
    let mut st_b_4 = Stopwatch::new();
    let mut st_c = Stopwatch::new();

    sw.restart();

    for (i, node) in terr.nodes.iter().enumerate() {

        // SEGMENT A  (126 ms)
        st_a.start();

        // Find the faces that contain this node
        let ref faces = node_to_faces[i];

        // Start with first face
        assert!(faces.len() > 0);

        let mut this_face_idx = faces[0];
        let curr_vertex = mesh_vertices.len() as u32;

        let elevation = (node.elevation - min_elev) / elev_scale;
        let uv = Point2::new(1.0 - elevation.powf(1.5), 0.0);
        mesh_texcoords.push(uv.clone());

        let normal = &node.point;
        let mut n = 0;
        let mut midpoint = Vec3::new(0.0f32, 0.0, 0.0);

        st_a.stop();
        // SEGMENT B  (2604 ms)
        st_b.start();

        loop {
            let face_mid = face_midpoints[this_face_idx];
            // SEGMENT B:0  (36 ms)
            st_b_0.start();
            midpoint += face_mid;
            st_b_0.stop();
            // SEGMENT B:1  (1099 ms)
            st_b_1.start();
            mesh_texcoords.push(uv.clone());
            mesh_vertices.push(Point3::from(&face_mid));
            mesh_normals.push(Vector3::from(normal));
            n += 1;

            st_b_1.stop();
            // SEGMENT B:2  (533 ms)
            st_b_2.start();

            let face = &terr.faces[this_face_idx];

            let other_point_idx = if face.points[0] == i as u32 {
                2
            } else if face.points[1] == i as u32 {
                0
            } else {
                1
            };

            let other_point = face.points[other_point_idx];
            // Find the edge
            let (e0, e1) = if i as u32 <= other_point {
                (i as u32, other_point)
            } else {
                (other_point, i as u32)
            };

            let mut edge_idx = usize::max_value();
            st_b_2.stop();
            // SEGMENT B:3  (405 ms)
            st_b_3.start();

            for e in node_to_edges[i].iter() {
                let ref te = terr.edges[*e];
                if te.a == e0 && te.b == e1 {
                    edge_idx = *e;
                    break;
                }
            }

            st_b_3.stop();
            // SEGMENT B:4  (210 ms)
            st_b_4.start();

            assert!(edge_idx != usize::max_value());
            // Find the other face with that edge

            let ef = &edge_to_faces[edge_idx];
            let other_face_idx = if ef[0] == this_face_idx { ef[1] } else { ef[0] };

            if other_face_idx == faces[0] {
                break;
            }
            this_face_idx = other_face_idx;
            st_b_4.stop();
        }
        st_b_4.stop();

        st_b.stop();
        // SEGMENT C  (587 ms)
        st_c.start();

        midpoint /= n as f32;

        mesh_vertices.push(Point3::from(&midpoint));
        mesh_normals.push(Vector3::from(normal));

        let center = curr_vertex + n;
        for j in 0..n {
            let p1 = j;
            let p2 = (j + 1) % n;
            mesh_faces.push(Point3::new(center, curr_vertex + p1, curr_vertex + p2));
        }

        st_c.stop();
    }

    println!("  Generated mesh in {} ms", sw.elapsed_ms()); // (3353 ms)
    println!("    Segment A: {} ms", st_a.elapsed_ms());
    println!("    Segment B: {} ms", st_b.elapsed_ms());
    println!("      Segment B:0: {} ms", st_b_0.elapsed_ms());
    println!("      Segment B:1: {} ms", st_b_1.elapsed_ms());
    println!("      Segment B:2: {} ms", st_b_2.elapsed_ms());
    println!("      Segment B:3: {} ms", st_b_3.elapsed_ms());
    println!("      Segment B:4: {} ms", st_b_4.elapsed_ms());
    println!("    Segment C: {} ms", st_c.elapsed_ms());

    println!("    Capacity mesh_faces:     {} / {}",
             mesh_faces.len(),
             mesh_faces.capacity());
    println!("    Capacity mesh_vertices:  {} / {}",
             mesh_vertices.len(),
             mesh_vertices.capacity());
    println!("    Capacity mesh_normals:   {} / {}",
             mesh_normals.len(),
             mesh_normals.capacity());
    println!("    Capacity mesh_texcoords: {} / {}",
             mesh_texcoords.len(),
             mesh_texcoords.capacity());

    sw.restart();
    let r = Mesh::new(mesh_vertices,
                      mesh_faces,
                      Some(mesh_normals),
                      Some(mesh_texcoords),
                      false);

    println!("  Creating mesh object in {} ms", sw.elapsed_ms());

    r
}
