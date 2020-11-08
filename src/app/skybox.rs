use cgmath::prelude::*;
use std::ffi::{CStr, CString};

use utils::*;

use renderer::{Drawable, Material, Texture, CubeMap};
use renderer::{AttributeType, BufferLayout, IndexBuffer, Shader, Uniform, VertexArray, VertexBuffer};


static SKYBOX_VERTICES: [f32; 108] = [
    // positions       // normals        // texture coords
    -1.0f32,  1.0f32, -1.0f32,
    -1.0f32, -1.0f32, -1.0f32,
     1.0f32, -1.0f32, -1.0f32,
     1.0f32, -1.0f32, -1.0f32,
     1.0f32,  1.0f32, -1.0f32,
    -1.0f32,  1.0f32, -1.0f32,

    -1.0f32, -1.0f32,  1.0f32,
    -1.0f32, -1.0f32, -1.0f32,
    -1.0f32,  1.0f32, -1.0f32,
    -1.0f32,  1.0f32, -1.0f32,
    -1.0f32,  1.0f32,  1.0f32,
    -1.0f32, -1.0f32,  1.0f32,

     1.0f32, -1.0f32, -1.0f32,
     1.0f32, -1.0f32,  1.0f32,
     1.0f32,  1.0f32,  1.0f32,
     1.0f32,  1.0f32,  1.0f32,
     1.0f32,  1.0f32, -1.0f32,
     1.0f32, -1.0f32, -1.0f32,

    -1.0f32, -1.0f32,  1.0f32,
    -1.0f32,  1.0f32,  1.0f32,
     1.0f32,  1.0f32,  1.0f32,
     1.0f32,  1.0f32,  1.0f32,
     1.0f32, -1.0f32,  1.0f32,
    -1.0f32, -1.0f32,  1.0f32,

    -1.0f32,  1.0f32, -1.0f32,
     1.0f32,  1.0f32, -1.0f32,
     1.0f32,  1.0f32,  1.0f32,
     1.0f32,  1.0f32,  1.0f32,
    -1.0f32,  1.0f32,  1.0f32,
    -1.0f32,  1.0f32, -1.0f32,

    -1.0f32, -1.0f32, -1.0f32,
    -1.0f32, -1.0f32,  1.0f32,
     1.0f32, -1.0f32, -1.0f32,
     1.0f32, -1.0f32, -1.0f32,
    -1.0f32, -1.0f32,  1.0f32,
     1.0f32, -1.0f32,  1.0f32,
];

static SKYBOX_INDICES: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];

pub struct Skybox {
  vertex_array: Ref<VertexArray>,
  material: Ref<Material>,
}

impl Skybox {
  pub fn new(texture: &str) -> Skybox {
    let layout = BufferLayout::new(vec![AttributeType::Float3]);
    let vert_buff = VertexBuffer::create(SKYBOX_VERTICES.to_vec(), layout);
    let ind_buff = IndexBuffer::create(SKYBOX_INDICES.to_vec());
    let vertex_array = VertexArray::new(vec![vert_buff], ind_buff);
    let mut material = Material::new();
    material.unknown_uniform("skybox", Uniform::CubeMap(CubeMap::from_file(texture)));
    Skybox {
      vertex_array: Ref::from(vertex_array),
      material: Ref::from(material),
    }
  }
}

impl Drawable for Skybox {
  fn vertex_array(&self) -> &VertexArray {
    &self.vertex_array
  }
  fn material(&self) -> &Material {
    &self.material
  }

  fn shader_name(&self) -> String {
    "skybox".to_string()
  }
}