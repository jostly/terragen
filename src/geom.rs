use math::{Vec3, normalize};
use terrain::Terrain;
use na::{Vector3, Point3, Point2};
use stopwatch::Stopwatch;

use std::sync::mpsc::Sender;
use std::thread;

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

pub enum Message {
    Complete(Vec<Point3<f32>>,
             Vec<Point3<u32>>,
             Option<Vec<Vector3<f32>>>,
             Option<Vec<Point2<f32>>>,
             Option<Vec<Point3<u32>>>,
             Terrain),
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Generator {
    Regular,
    Dual,
}

pub fn generate(generator: Generator,
                terrain: Terrain,
                generate_wireframe: bool,
                tx: &Sender<Message>) {
    let channel = tx.clone();
    thread::spawn(move || {
        let sw = Stopwatch::start_new();
        let mess = match generator {
            Generator::Regular => generate_regular(terrain, generate_wireframe),
            Generator::Dual => generate_dual(terrain, generate_wireframe),
        };
        println!("Generating mesh took {} ms", sw.elapsed_ms());
        // (3568 ms, lvl 6)

        channel.send(mess).unwrap();
    });
}

fn generate_regular(ico: Terrain, generate_wireframe: bool) -> Message {
    let num_faces = ico.faces.len();
    let num_vertices = num_faces * 3;
    let (min_elev, max_elev) = ico.calculate_elevations();
    let elev_scale = max_elev - min_elev;
    let mut vertices = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut texcoords = Vec::with_capacity(num_vertices);
    let mut faces = Vec::with_capacity(num_faces);
    let mut wireframes = if generate_wireframe {
        Vec::with_capacity(num_faces * 2)
    } else {
        Vec::new()
    };

    {
        let ico_faces = &ico.faces;
        let ico_vertices = &ico.nodes;

        let mut vert_index = 0u32;
        for f in ico_faces.iter() {
            for idx in [f.points.x, f.points.y, f.points.z].iter() {
                let ref vert = ico_vertices[*idx as usize];
                let elevation = (vert.elevation - min_elev) / elev_scale;
                //let vertex_scale = (elevation.powi(2) - 0.5) * 0.02;
                let vertex = &vert.point; // * (1.0 + vertex_scale);

                vertices.push(Point3::from(vertex));
                let normal = normalize(ico.face_midpoint(f));
                normals.push(Vector3::from(&normal));
                let uv = Point2::new(1.0 - elevation.powf(1.5), 0.0);
                texcoords.push(uv);
            }
            faces.push(Point3::new(vert_index, vert_index + 1, vert_index + 2));
            if generate_wireframe {
                wireframes.push(Point3::new(vert_index, vert_index + 1, vert_index + 1));
                wireframes.push(Point3::new(vert_index + 2, vert_index + 2, vert_index));
            }
            vert_index += 3;
        }
    }

    Message::Complete(vertices,
                      faces,
                      Some(normals),
                      Some(texcoords),
                      if generate_wireframe {
                          Some(wireframes)
                      } else {
                          None
                      },
                      ico)
}
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

fn generate_dual(terr: Terrain, generate_wireframe: bool) -> Message {
    println!("  Generator started...");
    let mut sw = Stopwatch::start_new();

    let num_nodes = terr.nodes.len();
    let num_edges = terr.edges.len();
    let num_faces = terr.faces.len();

    let mut mesh_faces = Vec::with_capacity(num_nodes * 6 - 12);
    let mut mesh_vertices = Vec::with_capacity(num_nodes * 7 - 12);
    let mut mesh_normals = Vec::with_capacity(mesh_vertices.capacity());
    let mut mesh_texcoords = Vec::with_capacity(mesh_vertices.capacity());
    let mut wireframes = Vec::new();

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
            let fp = &face.points;
            node_to_faces[fp.x as usize].push(i);
            node_to_faces[fp.y as usize].push(i);
            node_to_faces[fp.z as usize].push(i);
        }
        node_to_faces
    };

    println!("  Built node -> face index @ {} ms", sw.elapsed_ms()); // (142 ms)

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

    println!("  Built node -> edge index @ {} ms", sw.elapsed_ms()); // (289 ms)

    // Build map of edge index -> faces
    let edge_to_faces = {
        let mut edge_to_faces = Vec::with_capacity(num_edges);
        for _ in 0..num_edges {
            edge_to_faces.push(Vec::with_capacity(2));
        }

        for (i, face) in terr.faces.iter().enumerate() {
            let fv = &face.edges;
            edge_to_faces[fv.x as usize].push(i);
            edge_to_faces[fv.y as usize].push(i);
            edge_to_faces[fv.z as usize].push(i);
        }
        edge_to_faces
    };

    println!("  Built edge -> face index @ {} ms", sw.elapsed_ms()); // (483 ms)

    let face_midpoints = {
        let mut face_midpoints = Vec::with_capacity(num_faces);
        for face in terr.faces.iter() {
            face_midpoints.push(terr.face_midpoint(face));
        }
        face_midpoints
    };

    println!("  Built midpoint registry @ {} ms", sw.elapsed_ms()); // (554 ms)

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
        let colour = 1.0 - elevation.powf(1.5);
        let uv = Point2::new(colour.min(1.0).max(0.0), 0.0);
        mesh_texcoords.push(uv.clone());

        let normal = &node.point;
        let mut n = 0;
        let mut midpoint = Vec3::new(0.0f32, 0.0, 0.0);

        st_a.stop();
        // SEGMENT B  (2192 ms)
        st_b.start();

        loop {
            let face_mid = &face_midpoints[this_face_idx];
            // SEGMENT B:0  (36 ms)
            st_b_0.start();
            midpoint += face_mid;
            st_b_0.stop();
            // SEGMENT B:1  (1099 ms)
            st_b_1.start();
            mesh_texcoords.push(uv.clone());
            mesh_vertices.push(Point3::from(face_mid));
            mesh_normals.push(Vector3::from(normal));
            n += 1;

            st_b_1.stop();
            // SEGMENT B:2  (98 ms)
            st_b_2.start();

            let face = &terr.faces[this_face_idx];

            let other_point = if face.points.x == i as u32 {
                face.points.z
            } else if face.points.y == i as u32 {
                face.points.x
            } else {
                face.points.y
            };

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
            if generate_wireframe {
                wireframes.push(curr_vertex + p1);
                wireframes.push(curr_vertex + p2);
            }
        }

        st_c.stop();
    }
    let wireframes = encode_wireframes(&wireframes);

    println!("  Generated mesh in {} ms", sw.elapsed_ms()); // (2944 ms)
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
    let r = Message::Complete(mesh_vertices,
                              mesh_faces,
                              Some(mesh_normals),
                              Some(mesh_texcoords),
                              wireframes,
                              terr);

    println!("  Creating mesh object in {} ms", sw.elapsed_ms());

    r
}

fn encode_wireframes(wireframes: &Vec<u32>) -> Option<Vec<Point3<u32>>> {
    let num_points = wireframes.len();
    if num_points == 0 {
        None
    } else {
        let mut encoded = Vec::with_capacity(num_points / 3 + 1);
        let mut i = 0;
        while i < num_points - 3 {
            let tri = Point3::new(wireframes[i], wireframes[i + 1], wireframes[i + 2]);
            encoded.push(tri);
            i += 3;
        }
        match num_points - i {
            2 => {
                let tri = Point3::new(wireframes[i], wireframes[i + 1], 0);
                encoded.push(tri);
            }
            1 => {
                let tri = Point3::new(wireframes[i], 0, 0);
                encoded.push(tri);
            }
            _ => {}
        }
        Some(encoded)
    }
}
