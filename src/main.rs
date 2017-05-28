extern crate kiss3d;
extern crate glfw;
extern crate nalgebra as na;
extern crate rand;
#[macro_use]
extern crate log;
extern crate env_logger;

mod terrain;

use na::{Vector3, UnitQuaternion, Point3, Point2};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;
use kiss3d::scene::SceneNode;
use kiss3d::resource::Mesh;

use glfw::{Action, Key, WindowEvent};

use terrain::Terrain;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

fn main() {

    env_logger::init().unwrap();

    let mut ico = Terrain::new();

    let mut window = Window::new("Terragen");

    let eye              = Point3::new(0.0, 2.0, 5.0);
    let at               = Point3::origin();
    let mut arc_ball     = ArcBall::new(eye, at);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.007);

    let mut grp = window.add_group();
    let mut c = add_mesh(&mut grp, generate_dual(&ico));

    while window.render_with_camera(&mut arc_ball) {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                    info!("Subdividing");
                    window.remove(&mut c);
                    ico.subdivide();
                    c = add_mesh(&mut grp, generate_dual(&ico));
                    event.inhibited = true
                },
                _ => { }
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

fn generate_regular(ico: &Terrain) -> Mesh {
    let ico_faces = &ico.faces;
    let ico_vertices = &ico.nodes;
    let num_vertices = ico_faces.len()*3;
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
            let vertex_scale = (elevation.powi(2) - 0.5)*0.02;
            let vertex = Point3::from_coordinates(vert.point.coords * (1.0 + vertex_scale));

            vertices.push(vertex);
            //let normal = normalize(&vert.point.coords);
            let normal = ico.normal(&f.points);
            normals.push(normal);
            let uv = Point2::new(1.0 - elevation, 0.0);
            texcoords.push(uv);
        }
        faces.push(Point3::new(vert_index, vert_index+1, vert_index+2));
        vert_index += 3;
    }

    Mesh::new(vertices, faces, Some(normals), Some(texcoords), false)
}

fn generate_dual(terr: &Terrain) -> Mesh {

    let num_nodes = terr.nodes.len();
    let num_edges = terr.edges.len();
    let num_faces = terr.faces.len();

    let mut mesh_faces = Vec::with_capacity(num_nodes * 6);
    let mut mesh_vertices = Vec::with_capacity(mesh_faces.len() * 7);
    let mut mesh_normals = Vec::with_capacity(mesh_vertices.len());
    let mut mesh_texcoords = Vec::with_capacity(mesh_vertices.len());

    let (min_elev, max_elev) = terr.calculate_elevations();
    let elev_scale = max_elev - min_elev;

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

    // Build map of node index -> edges
    let node_to_edges = {
        let mut node_to_edges = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            node_to_edges.push(Vec::with_capacity(6));
        }

        for (i, edge) in terr.edges.iter().enumerate() {
            for v in edge.iter() {
                node_to_edges[*v as usize].push(i);
            }
        }
        node_to_edges
    };

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

    for (i, node) in terr.nodes.iter().enumerate() {
        // Find the faces that contain this node
        let ref faces = node_to_faces[i];

        // Start with first face
        assert!(faces.len() > 0);

        let mut this_face_idx = faces[0];
        let curr_vertex = mesh_vertices.len() as u32;

        let elevation = (node.elevation - min_elev) / elev_scale;
        let uv = Point2::new(1.0 - elevation.powf(1.5), 0.0);
        mesh_texcoords.push(uv.clone());

        let normal = &node.point.coords;
        let mut n = 0;
        let mut midpoint = Vector3::new(0.0f32, 0.0, 0.0);

        loop {
            let face_mid = terr.face_midpoint(this_face_idx as u32);
            midpoint += face_mid.coords;
            mesh_vertices.push(face_mid);
            mesh_normals.push(normal.clone());
            mesh_texcoords.push(uv.clone());
            n += 1;

            let face = &terr.faces[this_face_idx];

            let other_point_idx =
                if face.points[0] == i as u32 { 2 }
                else if face.points[1] == i as u32 { 0 }
                else { 1 };

            let other_point = face.points[other_point_idx];
            // Find the edge
            let (e0, e1) = if i as u32 <= other_point { (i as u32, other_point) } else { (other_point, i as u32) };

            let mut edge_idx = usize::max_value();

            for e in node_to_edges[i].iter() {
                let te = terr.edges[*e];
                if te[0] == e0 && te[1] == e1 {
                    edge_idx = *e;
                    break;
                }
            }

            assert!(edge_idx != usize::max_value());
            // Find the other face with that edge

            let ef = &edge_to_faces[edge_idx];
            let other_face_idx = if ef[0] == this_face_idx { ef[1] } else { ef[0] };

            if other_face_idx == faces[0] {
                break;
            }
            this_face_idx = other_face_idx;
        }

        mesh_vertices.push(Point3::from_coordinates(midpoint / n as f32));
        mesh_normals.push(normal.clone());

        let center = curr_vertex + n;
        for j in 0..n {
            let p1 = j;
            let p2 = (j + 1) % n;
            mesh_faces.push(Point3::new(center, curr_vertex + p1, curr_vertex + p2));
        }
    }

    Mesh::new(mesh_vertices, mesh_faces, Some(mesh_normals), Some(mesh_texcoords), false)
}
