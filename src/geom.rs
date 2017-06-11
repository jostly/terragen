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

    let planet = terr.to_planet();

    // Count how many triangles we will need

    let mut num_faces = 0;

    for tile in planet.tiles.iter() {
        num_faces += tile.border.len();
    }

    let num_vertices = num_faces + planet.tiles.len();

    let mut mesh_faces = Vec::with_capacity(num_faces);
    let mut mesh_vertices = Vec::with_capacity(num_vertices);
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

    let mut pentagons = 0;
    let mut hexagons = 0;
    let mut heptagons = 0;
    let mut othergons = 0;

    sw.restart();

    let mut vertex_index = 0;
    for tile in planet.tiles.iter() {

        let normal = Vector3::from(&normalize(planet.vertices[tile.midpoint as usize].clone()));

        let mut n = 0;
        for vi in tile.border.iter() {
            mesh_vertices.push(Point3::from(&planet.vertices[*vi as usize]));
            mesh_normals.push(normal.clone());
            mesh_texcoords.push(Point2::new(0.0, 0.0));
            n += 1;
        }

        mesh_vertices.push(Point3::from(&planet.vertices[tile.midpoint as usize]));
        mesh_normals.push(normal);
        mesh_texcoords.push(Point2::new(0.0, 0.0));

        let center = vertex_index + n;
        for j in 0..n {
            let p1 = j;
            let p2 = (j + 1) % n;
            mesh_faces.push(Point3::new(center, vertex_index + p1, vertex_index + p2));
            if generate_wireframe {
                wireframes.push(vertex_index + p1);
                wireframes.push(vertex_index + p2);
            }
        }

        match n {
            5 => pentagons += 1,
            6 => hexagons += 1,
            7 => heptagons += 1,
            _ => othergons += 1,
        }

        vertex_index += n + 1;
    }

    let wireframes = encode_wireframes(&wireframes);

    println!("  Generated mesh in {} ms", sw.elapsed_ms()); // (2944 ms)

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

    let total_faces = pentagons + hexagons + heptagons;
    println!("  Number of tiles: {}", total_faces);
    println!("    Pentagons: {}", pentagons);
    println!("    Hexagons : {}", hexagons);
    println!("    Heptagons: {}", heptagons);
    if othergons > 0 {
        println!("  Also found {} tiles of other sizes", othergons);
    }

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
