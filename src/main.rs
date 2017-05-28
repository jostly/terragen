extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;
#[macro_use]
extern crate log;
extern crate env_logger;

mod terrain;

use na::{Vector3, UnitQuaternion, Point3, Point2};
use na::normalize;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;
use kiss3d::scene::SceneNode;
use kiss3d::resource::Mesh;

use terrain::Terrain;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

fn main() {

    env_logger::init().unwrap();

    let mut ico = Terrain::new();
    for _ in 0..2 {
        ico.subdivide();
    }

    let mut window = Window::new("Kiss3d: cube");

    let eye              = Point3::new(0.0, 2.0, 5.0);
    let at               = Point3::origin();
    let mut arc_ball     = ArcBall::new(eye, at);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.007);

    let mut c = generate_regular(&mut window, &ico);

    while window.render_with_camera(&mut arc_ball) {
        c.prepend_to_local_rotation(&rot);
    }
}

fn generate_regular(window: &mut Window, ico: &Terrain) -> SceneNode {
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

    let mesh = Mesh::new(vertices, faces, Some(normals), Some(texcoords), false);
    let mut c = window.add_mesh(Rc::new(RefCell::new(mesh)), Vector3::new(1.0, 1.0, 1.0));

    c.set_color(1.0, 1.0, 1.0);
    c.set_texture_from_file(&Path::new("media/colour_ramp.png"), "colour_ramp");

    c
}

