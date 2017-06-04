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
mod geom;

use na::{Vector3, UnitQuaternion, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;
use kiss3d::scene::SceneNode;
use kiss3d::resource::Mesh;

use glfw::{Action, Key, WindowEvent};

use stopwatch::Stopwatch;

use terrain::Terrain;
use geom::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use std::sync::mpsc::channel;

fn main() {

    env_logger::init().unwrap();

    let (tx, rx) = channel();

    let mut terrain = Some(Terrain::new());
    for _ in 0..6 {
        if let Some(ref mut ico) = terrain {
            ico.subdivide();
        }
    }

    let mut window = Window::new("Terragen");

    let eye = Point3::new(0.0, 2.0, 5.0);
    let at = Point3::origin();
    let mut arc_ball = ArcBall::new(eye, at);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.001);

    let mut grp = window.add_group();
    let mut terrain_node: Option<SceneNode> = None;

    let mut generator = Generator::Dual;
    let mut regenerate_mesh = true;

    while window.render_with_camera(&mut arc_ball) {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                    if let Some(ref mut ico) = terrain {
                        println!("Subdividing a level {} terrain", ico.current_level());
                        let sw = Stopwatch::start_new();
                        ico.subdivide();
                        println!("Subdivision took {}ms", sw.elapsed_ms());
                        // (1744 ms, lvl 6), (7401 ms, lvl 7)
                        regenerate_mesh = true;
                        event.inhibited = true
                    }
                }
                WindowEvent::Key(Key::D, _, Action::Release, _) => {
                    if generator == Generator::Regular {
                        generator = Generator::Dual;
                        regenerate_mesh = true;
                    }
                    event.inhibited = true
                }
                WindowEvent::Key(Key::R, _, Action::Release, _) => {
                    if generator == Generator::Dual {
                        generator = Generator::Regular;
                        regenerate_mesh = true;
                    }
                    event.inhibited = true
                }
                _ => {}
            }
        }
        if regenerate_mesh {
            if let Some(ico) = terrain {
                generate(generator, ico, &tx);
                terrain = None;
            }
            regenerate_mesh = false;
        }
        match rx.try_recv() {
            Ok(Message::Complete(vertices, faces, normals, texcoords, terr)) => {
                if let Some(mut c) = terrain_node {
                    window.remove(&mut c);
                }
                terrain_node =
                    Some(add_mesh(&mut grp,
                                  Mesh::new(vertices, faces, normals, texcoords, false)));
                terrain = Some(terr);
            }
            _ => {}
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
