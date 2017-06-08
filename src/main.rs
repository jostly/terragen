extern crate kiss3d;
extern crate glfw;
extern crate nalgebra as na;
extern crate rand;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate stopwatch;
extern crate gl;

mod terrain;
mod math;
mod geom;
mod render;

use na::{Vector3, UnitQuaternion, Point2, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;
use kiss3d::scene::SceneNode;
use kiss3d::resource::{Mesh, Material};

use glfw::{Action, Key, WindowEvent};

use stopwatch::Stopwatch;

use terrain::Terrain;
use geom::*;
use render::WireframeMaterial;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use std::sync::mpsc::channel;

fn main() {

    env_logger::init().unwrap();

    let (tx, rx) = channel();

    let mut terrain = Some(Terrain::new());
    //for _ in 0..6 {
    //if let Some(ref mut ico) = terrain {
    //ico.subdivide();
    //}
    //}

    let mut window = Window::new("Terragen");

    let eye = Point3::new(0.0, 2.0, 5.0);
    let at = Point3::origin();
    let mut arc_ball = ArcBall::new(eye, at);

    window.set_light(Light::StickToCamera);

    let wireframe_material = Rc::new(RefCell::new(Box::new(WireframeMaterial::new()) as
                                                  Box<Material + 'static>));

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.001);

    let mut grp = window.add_group();
    let mut terrain_node: Option<SceneNode> = None;

    let mut generator = Generator::Dual;
    let mut regenerate_mesh = true;
    let mut use_wireframe = true;
    let mut rotate = true;

    while window.render_with_camera(&mut arc_ball) {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                    if let Some(ref mut ico) = terrain {
                        println!("Subdividing a level {} terrain", ico.current_level());
                        let sw = Stopwatch::start_new();
                        ico.subdivide();
                        ico.stat();
                        println!("Subdivision took {}ms", sw.elapsed_ms());
                        // (1744 ms, lvl 6), (7401 ms, lvl 7)
                        regenerate_mesh = true;
                        event.inhibited = true
                    }
                }
                WindowEvent::Key(Key::S, _, Action::Release, _) => {
                    if let Some(ref mut ico) = terrain {
                        println!("Relaxing, total movement: {}", ico.relax(0.5));
                        regenerate_mesh = true;
                        event.inhibited = true
                    }
                }
                WindowEvent::Key(Key::D, _, Action::Release, _) => {
                    if generator == Generator::Regular {
                        generator = Generator::Dual;
                    } else {
                        generator = Generator::Regular;
                    }
                    regenerate_mesh = true;
                    event.inhibited = true
                }
                WindowEvent::Key(Key::R, _, Action::Release, _) => {
                    rotate = !rotate;
                    event.inhibited = true
                }
                WindowEvent::Key(Key::W, _, Action::Release, _) => {
                    use_wireframe = !use_wireframe;
                    regenerate_mesh = true;
                    event.inhibited = true;
                }
                _ => {}
            }
        }
        if regenerate_mesh {
            if let Some(ico) = terrain {
                generate(generator, ico, use_wireframe, &tx);
                terrain = None;
            }
            regenerate_mesh = false;
        }
        match rx.try_recv() {
            Ok(Message::Complete(vertices, faces, normals, texcoords, wireframe, terr)) => {
                if let Some(mut c) = terrain_node {
                    window.remove(&mut c);
                }
                terrain_node = Some(add_mesh(&mut grp,
                                             vertices,
                                             faces,
                                             normals,
                                             texcoords,
                                             wireframe,
                                             wireframe_material.clone()));
                terrain = Some(terr);
            }
            _ => {}
        }
        if rotate {
            grp.prepend_to_local_rotation(&rot);
        }
    }

}

fn add_mesh(parent: &mut SceneNode,
            vertices: Vec<Point3<f32>>,
            faces: Vec<Point3<u32>>,
            normals: Option<Vec<Vector3<f32>>>,
            texcoords: Option<Vec<Point2<f32>>>,
            wireframe: Option<Vec<Point3<u32>>>,
            wireframe_material: Rc<RefCell<Box<Material + 'static>>>)
            -> SceneNode {
    let mut grp = parent.add_group();

    if let Some(line_faces) = wireframe {
        let mesh = Mesh::new(vertices.clone(), line_faces, None, None, false);
        let mesh = Rc::new(RefCell::new(mesh));
        let scale = 1.001;
        let mut c = grp.add_mesh(mesh, Vector3::new(scale, scale, scale));

        c.set_color(0.0, 0.0, 0.0);
        c.set_lines_width(2.0);
        c.set_material(wireframe_material);
    }

    let mesh = Mesh::new(vertices, faces, normals, texcoords, false);
    let mesh = Rc::new(RefCell::new(mesh));

    let mut c = grp.add_mesh(mesh.clone(), Vector3::new(1.0, 1.0, 1.0));

    c.set_color(1.0, 1.0, 1.0);
    c.set_texture_from_file(&Path::new("media/height_ramp.png"), "colour_ramp");
    c.enable_backface_culling(false);


    grp
}
