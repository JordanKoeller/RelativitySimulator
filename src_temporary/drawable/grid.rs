// use cgmath::{Matrix4, Vector3};
use cgmath::{Matrix4, Vector3};

use renderer::shader_manager::ShaderManager;
use std::ffi::CStr;

use gl;

use drawable::Drawable;
use drawable::Model;

pub struct Grid {
    spacing: i32,
    extent: i32,
    model: Model,
}

impl Grid {
    pub fn new(spacing: i32, extent: i32, model_matrix: Matrix4<f32>) -> Grid {
        Grid {
            spacing: spacing,
            extent: extent,
            model: Model::new("resources/objects/gridblock/Grass_Block.obj", model_matrix),
        }
    }
}

impl Drawable for Grid {
    fn shader_name(&self) -> String {
        "world".to_string()
    }
    fn draw(&self, shader: &ShaderManager) {
        let s = shader.get_shader(self.shader_name());
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        for i in 0..self.extent-1 {
            for j in 0..self.extent-1 {
                for k in 0..self.extent-1 {
                    let new_matrix = Matrix4::from_translation(
                        Vector3::<f32>::new((i*self.spacing) as f32, (j * self.spacing) as f32, (k * self.spacing) as f32)
                    ) * self.model.model_matrix;
                    s.set_mat4(c_str!("model"), &new_matrix);
                    for mesh in &self.model.meshes {
                      unsafe {
                        mesh.draw(s);
                      }
                    }
                }
            }
        }
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::Disable(gl::BLEND);
        }
      }
}
