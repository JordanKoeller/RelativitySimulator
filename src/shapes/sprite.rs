use specs::prelude::*;
use cgmath::prelude::*;
use renderer::{DrawableState, Drawable, Material, Texture, AttributeType, BufferLayout, IndexBuffer, VertexArray, VertexBuffer};
use ecs::MyBuilder;

pub static QUAD_VERTICES: [f32; 20] = [
  0.5f32,  0.5f32, 0.0f32,    1.0f32, 1.0f32, // top right
  0.5f32, -0.5f32, 0.0f32,    1.0f32, 0.0f32, // bottom right
 -0.5f32, -0.5f32, 0.0f32,    0.0f32, 0.0f32, // bottom left
 -0.5f32,  0.5f32, 0.0f32,    0.0f32, 1.0f32  // top left 
];

pub static QUAD_INDICES: [u32; 6] = [0, 1, 2, 1, 2, 3];



pub struct Sprite {
  material: Material,
}

impl Sprite {
  pub fn new(texture: &str) -> Self {
    let mut mat = Material::new();
    mat.diffuse_texture(Texture::from_file(texture));
    Self {
      material: mat
    }
  }
}

impl Drawable for Sprite {
  fn shader_name(&self) -> String {
    "default_texture".to_string()
  }
  fn vertex_array(&self) -> VertexArray {
    let layout = BufferLayout::new(vec![AttributeType::Float3, AttributeType::Float2]);
    let vertex_buff = VertexBuffer::create(QUAD_VERTICES.to_vec(), layout);
    let ind_buff = IndexBuffer::create(QUAD_INDICES.to_vec());
    VertexArray::new(vec![vertex_buff], ind_buff)

  }

  fn material(&self) -> Material {
    self.material.clone()
  }
}