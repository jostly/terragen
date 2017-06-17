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
use kiss3d::text::Font;

use glfw::{Action, Key, WindowEvent};

use stopwatch::Stopwatch;

use terrain::Terrain;
use terrain::planet::Planet;
use geom::*;
use render::WireframeMaterial;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use std::sync::mpsc::channel;
use std::f32;

fn main() {

    env_logger::init().unwrap();

    let (tx, rx) = channel();

    let terr = Terrain::new();
    let mut terrain: Option<Terrain> = Some(terr);
    let mut planet: Option<Planet> = None;

    let mut window = Window::new_with_size("Terragen", 900, 900);

    let font = Font::new(&Path::new("media/1942_report/1942.ttf"), 50);

    let eye = Point3::new(0.0, 2.0, 50.0);
    let at = Point3::origin();
    let mut arc_ball = ArcBall::new(eye, at);

    window.set_light(Light::StickToCamera);


    let wireframe_material = Rc::new(RefCell::new(Box::new(WireframeMaterial::new()) as
                                                  Box<Material + 'static>));

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.001);

    let mut grp = window.add_group();
    let mut terrain_node: Option<SceneNode> = None;

    let generators = [Generator::Dual, Generator::Plates];
    let mut generator_index = 0;
    let mut regenerate_mesh = true;
    let mut use_wireframe = true;
    let mut rotate = false;
    let mut current_level = 0;
    let mut num_tiles = 0;

    while window.render_with_camera(&mut arc_ball) {
        let text_point = Point2::new(50.0, 50.0);

        if let Some(ref terr) = terrain {
            current_level = terr.current_level();
        }
        if let Some(ref pla) = planet {
            num_tiles = pla.num_tiles;
        }
        window.draw_text(&format!("Level: {}\nTiles: {}", current_level, num_tiles),
                         &text_point,
                         &font,
                         &Point3::new(1.0, 1.0, 1.0));

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                    if let Some(ref mut ico) = terrain {
                        info!("Subdividing a level {} terrain", ico.current_level());
                        let sw = Stopwatch::start_new();
                        ico.subdivide();
                        planet = None;
                        info!("Subdivision took {} ms", sw.elapsed_ms());
                        // (1744 ms, lvl 6), (7401 ms, lvl 7)
                        regenerate_mesh = true;
                        event.inhibited = true
                    }
                }
                WindowEvent::Key(Key::A, _, Action::Release, _) => {
                    if let Some(ref mut ico) = terrain {
                        let topology_distortion_rate = 0.04; // 0 - 0.15
                        let mut total_distortion =
                            (ico.num_edges() as f32 * topology_distortion_rate).ceil() as u32;
                        let mut iterations = 6;
                        while iterations > 0 {
                            let iteration_distortion = total_distortion / iterations;
                            total_distortion -= iteration_distortion;
                            ico.distort(iteration_distortion);
                            ico.relax(0.);
                            iterations -= 1;
                        }
                        planet = None;
                        regenerate_mesh = true;
                        event.inhibited = true;
                    }
                }
                WindowEvent::Key(Key::S, _, Action::Release, _) => {
                    if let Some(ref mut ico) = terrain {
                        let max_relax = 300;
                        let mut last_move = f32::MAX;
                        let mut i = 1;
                        let num_nodes = ico.num_nodes() as f32;
                        let average_node_radius = (f32::consts::PI * 4.0 / num_nodes).sqrt();
                        let min_shift_delta = average_node_radius * num_nodes / 500000.0;
                        while i <= max_relax {
                            let rel = ico.relax(0.5);
                            debug!("Relaxation iteration {}: {}", i, rel);
                            let diff = (last_move - rel).abs();
                            if diff < min_shift_delta {
                                debug!("Relaxation converging with diff {}", diff);
                                break;
                            }
                            last_move = rel;
                            i += 1;
                        }
                        planet = None;
                        regenerate_mesh = true;
                        event.inhibited = true
                    }
                }
                WindowEvent::Key(Key::D, _, Action::Release, _) => {
                    generator_index = (generator_index + 1) % generators.len();
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
                WindowEvent::Key(Key::G, _, Action::Release, _) => {
                    if let Some(ref mut pla) = planet {
                        pla.grow_plates();
                        regenerate_mesh = true;
                        event.inhibited = true;
                    }
                }
                WindowEvent::Key(Key::M, _, Action::Release, _) => {
                    if let Some(ref mut pla) = planet {
                        pla.merge_plates();
                        regenerate_mesh = true;
                        event.inhibited = true;
                    }
                }
                _ => {}
            }
        }
        if regenerate_mesh {
            if let Some(ico) = terrain {
                let mut p = planet.unwrap_or_else(|| ico.to_planet());
                generate(generators[generator_index], ico, p, use_wireframe, &tx);
                terrain = None;
                planet = None;
            }
            regenerate_mesh = false;
        }
        match rx.try_recv() {
            Ok(Message::Complete(vertices, faces, normals, texcoords, terr, pla)) => {
                if let Some(mut c) = terrain_node {
                    window.remove(&mut c);
                }
                let (wirecoords, wirefaces) = generate_plate_vectors(&pla);
                terrain_node = Some(add_mesh(generators[generator_index],
                                             &mut grp,
                                             vertices,
                                             faces,
                                             normals,
                                             texcoords,
                                             Some((wirecoords, wirefaces)),
                                             wireframe_material.clone()));
                terrain = Some(terr);
                planet = Some(pla);
            }
            _ => {}
        }
        if rotate {
            grp.prepend_to_local_rotation(&rot);
        }
    }

}

fn add_mesh(generator: Generator,
            parent: &mut SceneNode,
            vertices: Vec<Point3<f32>>,
            faces: Vec<Point3<u32>>,
            normals: Option<Vec<Vector3<f32>>>,
            texcoords: Option<Vec<Point2<f32>>>,
            wireframes: Option<(Vec<Point3<f32>>, Vec<Point3<u32>>)>,
            wireframe_material: Rc<RefCell<Box<Material + 'static>>>)
            -> SceneNode {
    let mut grp = parent.add_group();
    if let Some((line_verts, line_faces)) = wireframes {
        let mesh = Mesh::new(line_verts, line_faces, None, None, false);
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
    if generator == Generator::Plates {
        c.set_texture_from_file(&Path::new("media/groups.png"), "colour_ramp");
    } else {
        c.set_texture_from_file(&Path::new("media/height_ramp.png"), "colour_ramp");
    }
    c.enable_backface_culling(true);

    grp
}
