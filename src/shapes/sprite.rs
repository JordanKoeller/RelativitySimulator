use specs::prelude::*;
use cgmath::prelude::*;
use renderer::{DrawableState, Drawable, Material, Texture, TextureLike, AttributeType, BufferLayout, IndexBuffer, VertexArray, VertexBuffer};
use ecs::MyBuilder;

pub static QUAD_VERTICES: [f32; 20] = [
  0.5f32,  0.5f32, 0.0f32,    1.0f32, 1.0f32, // top right
  0.5f32, -0.5f32, 0.0f32,    1.0f32, 0.0f32, // bottom right
 -0.5f32, -0.5f32, 0.0f32,    0.0f32, 0.0f32, // bottom left
 -0.5f32,  0.5f32, 0.0f32,    0.0f32, 1.0f32  // top left 
];

pub static QUAD_INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];



pub struct Sprite {
  material: Material,
  vertex_array: VertexArray,
}

impl Sprite {
  pub fn new(texture: &str) -> Self {
    let mut mat = Material::new();
    let mut tex = Texture::from_file(texture);
    tex.refresh();
    let vertex_buff = Sprite::rescale_vertices((tex.height as f32) / (tex.width as f32));
    let ind_buff = IndexBuffer::create(QUAD_INDICES.to_vec());
    let vertex_array = VertexArray::new(vec![vertex_buff], ind_buff);
    mat.diffuse_texture(tex);
    Self {
      material: mat,
      vertex_array: vertex_array,
    }
  }
  
  fn rescale_vertices(aspect_ratio: f32) -> VertexBuffer {
    let layout = BufferLayout::new(vec![AttributeType::Float3, AttributeType::Float2]);
    let mut vertices = QUAD_VERTICES.to_vec();
    if aspect_ratio < 1f32 {
      vertices[1] *= aspect_ratio;
      vertices[6] *= aspect_ratio;
      vertices[11] *= aspect_ratio;
      vertices[16] *= aspect_ratio;
    } else {
      vertices[0] /= aspect_ratio;
      vertices[5] /= aspect_ratio;
      vertices[10] /= aspect_ratio;
      vertices[15] /= aspect_ratio;
    }
    VertexBuffer::create(vertices, layout)
  } 
}

impl Drawable for Sprite {
  fn shader_name(&self) -> String {
    "default_texture".to_string()
  }
  fn vertex_array(&self) -> VertexArray {
    self.vertex_array.clone()
  }

  fn material(&self) -> Material {
    self.material.clone()
  }
}