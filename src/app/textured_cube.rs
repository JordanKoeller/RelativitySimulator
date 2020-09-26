use cgmath::prelude::*;
use std::ffi::{CStr, CString};

use utils::*;

use renderer::{Drawable, Material, Texture};
use renderer::{AttributeType, BufferLayout, IndexBuffer, Shader, Uniform, VertexArray, VertexBuffer};

const TEXTURE_CUBE_VERTICES: [f32; 180] = [
    // positions       // normals        // texture coords
    -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, -0.5,
    0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5,
    0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5, 0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0,
    0.0, -0.5, 0.5, -0.5, 1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0,
    -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5,
    -0.5, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5,
    1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0,
    1.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5,
    0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0,
];

const TEXTURE_CUBE_INDICES: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];

pub struct TexturedCube {
  vertex_array: VertexArray,
  uniforms: Vec<(CString, Uniform)>,
  textures: Vec<(CString, Texture)>,
}

impl TexturedCube {
  pub fn new(position: Vec3F, texture: &str) -> TexturedCube {
    let transform = translate(position);
    let layout = BufferLayout::new(vec![AttributeType::Float3, AttributeType::Float2]);
    let vert_buff = VertexBuffer::create(TEXTURE_CUBE_VERTICES.to_vec(), layout);
    let ind_buff = IndexBuffer::create(TEXTURE_CUBE_INDICES.to_vec());
    let vertex_array = VertexArray::new(vec![vert_buff], ind_buff);
    let uniforms = vec![
      (CString::new("model").unwrap(), Uniform::Mat4(transform)),
    ];
    let textures = vec![
      (CString::new("diffuseMap").unwrap(), Texture::from_file(texture))
    ];
    TexturedCube { vertex_array, uniforms, textures }
  }
}

impl Drawable for TexturedCube {
  fn vertex_array(&self) -> &VertexArray {
    &self.vertex_array
  }
  fn material(&self) -> &Material {
    &self.uniforms
  }

  fn shader_name(&self) -> &str {
    "default_texture"
  }

  fn textures(&self) -> Option<&Vec<(CString, Texture)>> {
    Some(&self.textures)
  }
}
