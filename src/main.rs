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

fn main() {

    env_logger::init().unwrap();

    let mut ico = Terrain::new();
    for _ in 0..6 {
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

    let genfuns: [fn(&Terrain) -> Mesh; 2] = [generate_dual, generate_regular];
    let mut genfunidx = 0;
    let mut regenerate_mesh = false;

    while window.render_with_camera(&mut arc_ball) {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                    println!("Subdividing a level {} terrain", ico.current_level());
                    let sw = Stopwatch::start_new();
                    ico.subdivide();
                    println!("Subdivision took {}ms", sw.elapsed_ms());
                    // (1744 ms, lvl 6), (7401 ms, lvl 7)
                    regenerate_mesh = true;
                    event.inhibited = true
                }
                WindowEvent::Key(Key::D, _, Action::Release, _) => {
                    if genfunidx == 1 {
                        genfunidx = 0;
                        regenerate_mesh = true;
                    }
                    event.inhibited = true
                }
                WindowEvent::Key(Key::R, _, Action::Release, _) => {
                    if genfunidx == 0 {
                        genfunidx = 1;
                        regenerate_mesh = true;
                    }
                    event.inhibited = true
                }
                _ => {}
            }
        }
        if regenerate_mesh {
            window.remove(&mut c);
            let mut sw = Stopwatch::start_new();
            let mesh = genfuns[genfunidx](&ico);
            println!("Generating mesh took {}ms", sw.elapsed_ms());
            // (3568 ms, lvl 6)
            sw.restart();
            c = add_mesh(&mut grp, mesh);
            println!("Adding mesh took {}ms", sw.elapsed_ms());
            regenerate_mesh = false;
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
