use lazy_static::lazy_static;
use std::sync::Mutex;

use std::ffi::CStr;
use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_void;

use cgmath::prelude::*;
use cgmath::{Matrix4, Vector2, Vector3};

use gl;

use drawable::mesh::{Mesh, Texture};
use drawable::Drawable;
use drawable::Model;
use renderer::shader::Shader;
use renderer::shader_manager::ShaderManager;
use common::load_texture;

struct VaoVbo {
    vao: u32,
    vbo: u32,
    ebo: u32,
}

const BOX_VERTICES: [f32; 288] = [
        // positions          // normals           // texture coords
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
];

lazy_static! {
    static ref CUBE_MESH: VaoVbo = init_cube();
}

pub struct Cube {
    texture: Texture,
    model_matrix: Matrix4<f32>,
}

impl Cube {
    pub fn new(tex: Texture, mat: Matrix4<f32>) -> Cube {
        Cube {
            texture: tex,
            model_matrix: mat,
        }
    }

    pub fn with_texture(texture_path: &str, mat: Matrix4<f32>) -> Cube {
        unsafe {
            let t_id =  load_texture(texture_path);
            let texture = Texture {
                id: t_id,
                type_: "ambient".to_string(),
                path: texture_path.to_string()
            };
            Cube::new(texture, mat)
        }
    }
}

impl Drawable for Cube {
    fn shader_name(&self) -> String {
        "cube".to_string()
    }

    fn draw(&self, shader: &ShaderManager) {
        let s = shader.get_shader(self.shader_name());
        s.set_mat4(c_str!("model"), &self.model_matrix);
        let norm_matrix = self
            .model_matrix
            .inverse_transform()
            .expect("Could not invert model matrix")
            .transpose();
        s.set_mat4(c_str!("normalMatrix"), &norm_matrix);
        s.set_int(c_str!("texture_diffuse1"), 0);
        // let sampler = CString::new("texture_diffuse1").unwrap();
        unsafe {
            // gl::Uniform1i(gl::GetUniformLocation(s.id, sampler.as_ptr()), 0 as i32);
            // and finally bind the texture
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
            // draw mesh
            gl::BindVertexArray(CUBE_MESH.vao);
            gl::DrawArrays(gl::TRIANGLES,0, 36);
            gl::BindVertexArray(0);
            // always good practice to set everything back to defaults once configured.
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }
}

fn init_cube() -> VaoVbo {
    println!("INIT CUBE");
    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

    let size = (size_of::<f32>() * BOX_VERTICES.len()) as isize;
    let data = &BOX_VERTICES[0] as *const f32 as *const c_void;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        // gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);
        gl::BindVertexArray(vao);

        // Positions
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()) as i32,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(0);

        // normals
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()) as i32,
            (3 * size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        // texture uv
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()) as i32,
            (6 * size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);

        gl::BindVertexArray(0);
    }

    let vao_d = vao;
    let vbo_d = vbo;
    let ebo_d = ebo;

    VaoVbo {
        vao: vao_d,
        vbo: vbo_d,
        ebo: ebo_d,
    }
}
