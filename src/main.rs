extern crate kiss3d;
extern crate nalgebra as na;

mod icosahedron;

use na::{Vector3, UnitQuaternion, Point3};
use na::normalize;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;
use kiss3d::scene::SceneNode;
use kiss3d::resource::Mesh;

use icosahedron::Icosahedron;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    let mut ico = Icosahedron::new();
    for _ in 0..1 {
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

fn generate_regular(window: &mut Window, ico: &Icosahedron) -> SceneNode {
    let ico_faces = &ico.faces;
    let ico_vertices = &ico.vertices;
    let mut vertices = Vec::with_capacity(ico_faces.len()*3);
    let mut normals = Vec::with_capacity(vertices.capacity());
    let mut faces = Vec::with_capacity(ico_faces.len());

    let mut vert_index = 0u32;
    for f in ico_faces.iter() {
        let normal = ico.normal(&f.points);
        for idx in f.points.iter() {
            let vert = ico_vertices[*idx as usize];
            vertices.push(vert.clone());
            normals.push(normal.clone());
        }
        faces.push(Point3::new(vert_index, vert_index+1, vert_index+2));
        vert_index += 3;
    }

    let mesh = Mesh::new(vertices, faces, Some(normals), None, false);
    let mut c = window.add_mesh(Rc::new(RefCell::new(mesh)), Vector3::new(1.0, 1.0, 1.0));

    c.set_color(1.0, 0.5, 0.2);

    c
}

fn generate_dual(window: &mut Window, ico: &Icosahedron) -> SceneNode {
    let ico_faces = &ico.faces;
    let ico_vertices = &ico.vertices;

    let mut vertices = Vec::with_capacity(ico_faces.len()*3);
    let mut normals = Vec::with_capacity(vertices.capacity());
    let mut faces = Vec::with_capacity(ico_faces.len());

    let mut vert_index = 0u32;
    for (i, v) in ico_vertices.iter().enumerate() {
        let normal = normalize(&v.coords);
        let face_indices = ico.find_faces(i as u32);
        let num_vertices = face_indices.len() as u32;
        let mut centroid = Vector3::new(0.0f32, 0.0, 0.0);
        for idx in face_indices.iter() {
            let face = &ico_faces[*idx];
            let p0 = &ico_vertices[face.points[0] as usize];
            let p1 = &ico_vertices[face.points[1] as usize];
            let p2 = &ico_vertices[face.points[2] as usize];

            let midpoint = Point3::from_coordinates((p0.coords + p1.coords + p2.coords) / 3.0);
            centroid += midpoint.coords;
            vertices.push(midpoint);
            normals.push(normal.clone());
        }
        vertices.push(Point3::from_coordinates(centroid / num_vertices as f32));

        faces.push(Point3::new(vert_index, vert_index+1, vert_index+2));
        vert_index += 3;
    }

    let mesh = Mesh::new(vertices, faces, Some(normals), None, false);
    let mut c = window.add_mesh(Rc::new(RefCell::new(mesh)), Vector3::new(1.0, 1.0, 1.0));

    c.set_color(1.0, 0.5, 0.2);

    c
}

