use std::ptr;
use gl;
use gl::types::*;
use na::{Point3, Vector3, Matrix3, Matrix4, Isometry3};
use kiss3d::resource::Material;
use kiss3d::scene::ObjectData;
use kiss3d::light::Light;
use kiss3d::camera::Camera;
use kiss3d::resource::{Mesh, Shader, ShaderAttribute, ShaderUniform};

macro_rules! verify(
    ($e: expr) => {
        unsafe {
            let res = $e;
            assert_eq!(gl::GetError(), 0);
            res
        }
    }
);

/// The default material used to draw objects.
pub struct WireframeMaterial {
    shader: Shader,
    pos: ShaderAttribute<Point3<f32>>,
    color: ShaderUniform<Point3<f32>>,
    transform: ShaderUniform<Matrix4<f32>>,
    scale: ShaderUniform<Matrix3<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
}

impl WireframeMaterial {
    pub fn new() -> WireframeMaterial {
        // load the shader
        let mut shader = Shader::new_from_str(WIREFRAME_VERTEX_SRC, WIREFRAME_FRAGMENT_SRC);

        shader.use_program();

        // get the variables locations
        WireframeMaterial {
            pos: shader.get_attrib("position").unwrap(),
            color: shader.get_uniform("color").unwrap(),
            transform: shader.get_uniform("transform").unwrap(),
            scale: shader.get_uniform("scale").unwrap(),
            view: shader.get_uniform("view").unwrap(),
            shader: shader,
        }
    }

    fn activate(&mut self) {
        self.shader.use_program();
        self.pos.enable();
    }

    fn deactivate(&mut self) {
        self.pos.disable();
    }
}

impl Material for WireframeMaterial {
    fn render(&mut self,
              pass: usize,
              transform: &Isometry3<f32>,
              scale: &Vector3<f32>,
              camera: &mut Camera,
              _: &Light,
              data: &ObjectData,
              mesh: &mut Mesh) {
        self.activate();


        /*
         *
         * Setup camera and light.
         *
         */
        camera.upload(pass, &mut self.view);

        /*
         *
         * Setup object-related stuffs.
         *
         */
        let formated_transform = transform.to_homogeneous();
        let formated_scale = Matrix3::from_diagonal(&Vector3::new(scale.x, scale.y, scale.z));

        unsafe {
            self.transform.upload(&formated_transform);
            self.scale.upload(&formated_scale);
            self.color.upload(data.color());

            mesh.bind_coords(&mut self.pos);
            mesh.bind_faces();

            if data.lines_width() != 0.0 {
                gl::Disable(gl::CULL_FACE);
                //verify!(gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE));
                gl::LineWidth(data.lines_width());
                let mut pts = mesh.num_pts();
                if pts % 2 == 1 {
                    pts -= 1;
                }
                gl::DrawElements(gl::LINES, pts as GLint, gl::UNSIGNED_INT, ptr::null());
                gl::LineWidth(1.0);
            }
        }

        mesh.unbind();
        self.deactivate();
    }
}

/// Vertex shader of the default object material.
pub static WIREFRAME_VERTEX_SRC: &'static str = A_VERY_LONG_STRING;
/// Fragment shader of the default object material.
pub static WIREFRAME_FRAGMENT_SRC: &'static str = ANOTHER_VERY_LONG_STRING;

const A_VERY_LONG_STRING: &'static str = "#version 120
attribute vec3 position;
uniform mat4 view;
uniform mat4 transform;
uniform mat3 scale;
void main() {
    gl_Position = view * transform * mat4(scale) * vec4(position, 1.0);
}
";

const ANOTHER_VERY_LONG_STRING: &'static str = "#version 120
uniform vec3      color;
void main() {
    gl_FragColor = vec4(color, 1.0);
}
";
