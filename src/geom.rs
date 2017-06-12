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
            Generator::Regular => generate_regular(terrain),
            Generator::Dual => generate_dual(terrain, generate_wireframe),
        };
        info!("Generating mesh took {} ms", sw.elapsed_ms());
        // (3568 ms, lvl 6)

        channel.send(mess).unwrap();
    });
}

fn elevation_to_uv(elevation: f32, min_elev: f32, max_elev: f32) -> Point2<f32> {
    let d = max_elev - min_elev;
    let scaled_elev = if d.abs() > 0.01 {
        (elevation - min_elev) / (max_elev - min_elev)
    } else {
        0.5
    };
    Point2::new(1.0 - scaled_elev.powf(1.5), 0.0)
}

fn generate_regular(ico: Terrain) -> Message {
    let num_faces = ico.faces.len();
    let num_vertices = num_faces * 3;
    let (min_elev, max_elev) = ico.calculate_elevations();
    let mut vertices = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut texcoords = Vec::with_capacity(num_vertices);
    let mut faces = Vec::with_capacity(num_faces);

    {
        let ico_faces = &ico.faces;
        let ico_vertices = &ico.nodes;

        let mut vert_index = 0u32;
        for f in ico_faces.iter() {
            let mut average_elevation = 0.0;
            for idx in [f.points.x, f.points.y, f.points.z].iter() {
                let ref vert = ico_vertices[*idx as usize];
                average_elevation += vert.elevation;
                //let vertex_scale = (elevation.powi(2) - 0.5) * 0.02;
                let vertex = &vert.point; // * (1.0 + vertex_scale);

                vertices.push(Point3::from(vertex));
                let normal = normalize(ico.face_midpoint(f));
                normals.push(Vector3::from(&normal));
            }
            let uv = elevation_to_uv(average_elevation / 3.0, min_elev, max_elev);
            for _ in 0..3 {
                texcoords.push(uv);
            }

            faces.push(Point3::new(vert_index, vert_index + 1, vert_index + 2));
            vert_index += 3;
        }
    }

    Message::Complete(vertices, faces, Some(normals), Some(texcoords), None, ico)
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
    debug!("  Generator started...");
    let mut sw = Stopwatch::start_new();

    let planet = terr.to_planet();

    // Count how many triangles we will need

    let mut num_faces = 0;

    for tile in planet.tiles.iter() {
        num_faces += tile.border.len();
    }

    let mut num_vertices = num_faces + planet.tiles.len();
    if generate_wireframe {
        num_vertices += num_faces;
        num_faces += num_faces * 2;
    }

    let mut mesh_faces = Vec::with_capacity(num_faces);
    let mut mesh_vertices = Vec::with_capacity(num_vertices);
    let mut mesh_normals = Vec::with_capacity(mesh_vertices.capacity());
    let mut mesh_texcoords = Vec::with_capacity(mesh_vertices.capacity());

    debug!("    Capacity mesh_faces:     {} / {}",
           mesh_faces.len(),
           mesh_faces.capacity());
    debug!("    Capacity mesh_vertices:  {} / {}",
           mesh_vertices.len(),
           mesh_vertices.capacity());
    debug!("    Capacity mesh_normals:   {} / {}",
           mesh_normals.len(),
           mesh_normals.capacity());
    debug!("    Capacity mesh_texcoords: {} / {}",
           mesh_texcoords.len(),
           mesh_texcoords.capacity());

    debug!("  Generated vectors @ {} ms", sw.elapsed_ms());

    let (min_elev, max_elev) = terr.calculate_elevations();
    let elev_scale = max_elev - min_elev;

    debug!("  Calculated min/max elev @ {} ms", sw.elapsed_ms()); // (7 ms)

    let mut pentagons = 0;
    let mut hexagons = 0;
    let mut heptagons = 0;
    let mut othergons = 0;

    sw.restart();

    let mut vertex_index = 0;
    for tile in planet.tiles.iter() {

        let normal = Vector3::from(&normalize(planet.vertices[tile.midpoint as usize].clone()));

        let elevation = (tile.elevation - min_elev) / elev_scale;
        let colour = 1.0 - elevation.powf(1.5);
        let uv = Point2::new(colour.min(1.0).max(0.0), 0.10);
        let uv_outer = if generate_wireframe {
            Point2::new(colour.min(1.0).max(0.0), 0.5)
        } else {
            uv.clone()
        };

        // Center
        mesh_vertices.push(Point3::from(&planet.vertices[tile.midpoint as usize]));
        mesh_normals.push(normal);
        let center_uv = Point2::new(colour.min(1.0).max(0.0), 0.0);
        mesh_texcoords.push(center_uv);

        let mut n = 0;
        for vi in tile.border.iter() {
            mesh_vertices.push(Point3::from(&planet.vertices[*vi as usize]));
            mesh_normals.push(normal.clone());
            mesh_texcoords.push(uv_outer.clone());
            n += 1;
        }

        if generate_wireframe {
            let mp = &planet.vertices[tile.midpoint as usize];
            for vi in tile.border.iter() {
                let delta = (&planet.vertices[*vi as usize] - mp) * 0.90 + mp;
                mesh_vertices.push(Point3::from(&delta));
                mesh_normals.push(normal.clone());
                mesh_texcoords.push(uv.clone());
            }
        }


        let center = vertex_index;
        for j in 0..n {
            let p1 = vertex_index + 1 + j;
            let p2 = vertex_index + 1 + (j + 1) % n;
            if generate_wireframe {
                let p1_inner = p1 + n;
                let p2_inner = p2 + n;

                mesh_faces.push(Point3::new(center, p1_inner, p2_inner));
                mesh_faces.push(Point3::new(p1_inner, p1, p2_inner));
                mesh_faces.push(Point3::new(p1, p2, p2_inner));
            } else {
                mesh_faces.push(Point3::new(center, p1, p2));
            }
        }

        match n {
            5 => pentagons += 1,
            6 => hexagons += 1,
            7 => heptagons += 1,
            _ => othergons += 1,
        }

        vertex_index += n + 1;
        if generate_wireframe {
            vertex_index += n;
        }
    }

    let wireframes = None;

    debug!("  Generated mesh in {} ms", sw.elapsed_ms()); // (2944 ms)

    debug!("    Capacity mesh_faces:     {} / {}",
           mesh_faces.len(),
           mesh_faces.capacity());
    debug!("    Capacity mesh_vertices:  {} / {}",
           mesh_vertices.len(),
           mesh_vertices.capacity());
    debug!("    Capacity mesh_normals:   {} / {}",
           mesh_normals.len(),
           mesh_normals.capacity());
    debug!("    Capacity mesh_texcoords: {} / {}",
           mesh_texcoords.len(),
           mesh_texcoords.capacity());

    let total_faces = pentagons + hexagons + heptagons;
    debug!("  Number of tiles: {}", total_faces);
    debug!("    Pentagons: {}", pentagons);
    debug!("    Hexagons : {}", hexagons);
    debug!("    Heptagons: {}", heptagons);
    if othergons > 0 {
        debug!("  Also found {} tiles of other sizes", othergons);
    }
    debug!("  Earth analogy: average tile is {} km^2",
           510100000 / total_faces);

    sw.restart();

    let r = Message::Complete(mesh_vertices,
                              mesh_faces,
                              Some(mesh_normals),
                              Some(mesh_texcoords),
                              wireframes,
                              terr);

    debug!("  Creating mesh object in {} ms", sw.elapsed_ms());

    r
}
