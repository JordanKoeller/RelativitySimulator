use std::ops::Deref;
use specs::prelude::*;
use specs::{Component, VecStorage};
use cgmath::prelude::{InnerSpace};
use renderer::{Drawable, Texture};
use renderer::{AttributeType, BufferLayout, IndexBuffer, VertexArray, DataBuffer};

use physics::{Collision, CollisionSummary};

use utils::{Vec3F, Vec4F, swizzle_down, swizzle_up, Mat3F};
use physics::TransformComponent;

use ecs::Material;


pub struct Block {
  filename: String
}

impl Drawable for Block {
  fn vertex_array(&self) -> VertexArray {
    let layout = BufferLayout::new(vec![AttributeType::Float3, AttributeType::Float3, AttributeType::Float2]);
    let vert_buff = DataBuffer::static_buffer(&TEXTURE_CUBE_VERTICES, layout);
    let ind_buff = IndexBuffer::create(TEXTURE_CUBE_INDICES.to_vec());
    VertexArray::new(vert_buff, ind_buff)
  }
  fn material(&self) -> Material {
    let mut material = Material::new();
    material.diffuse_texture(Texture::from_file(&self.filename));
    material
  }

  fn shader_name(&self) -> String {
    "instanced".to_string()
  }

  fn instance_attributes(&self) -> Option<Vec<(String, AttributeType)>> {
    // None
    Some(vec![
      ("model".to_string(), AttributeType::Mat4),
      ("diffuse_texture".to_string(), AttributeType::Int),
    ])
  }
}

impl Block {
  pub fn new(texture_file: &str) -> Self {
    Self {
      filename: texture_file.to_string()
    }
  }
}

// Counter-clockwise is front-facing

pub static TEXTURE_CUBE_VERTICES: [f32; 288] = [
    // positions                 // normals                // texture coords
   -0.5f32,  -0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.3333333f32,  0.25f32,
    0.5f32,  -0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.6666666f32,  0.25f32,
    0.5f32,   0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.6666666f32,  0.50f32, // FRONT
    0.5f32,   0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.6666666f32,  0.50f32,
   -0.5f32,   0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.3333333f32,  0.50f32,
   -0.5f32,  -0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.3333333f32,  0.25f32,

   -0.5f32,  -0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.3333333f32,  1.0f32,
   0.5f32,   0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.6666666f32,  0.75f32, // BACK
   0.5f32,  -0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.6666666f32,  1.0f32,
    0.5f32,   0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.6666666f32,  0.75f32,
    -0.5f32,  -0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.3333333f32,  1.0f32,
    -0.5f32,   0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.3333333f32,  0.75f32,

   -0.5f32,   0.5f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.33333333f32,  0.5f32, // B
   -0.5f32,  -0.5f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0000000f32,  0.75f32, // C RIGHT
   -0.5f32,   0.5f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.33333333f32,  0.75f32, // A
   -0.5f32,  -0.5f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0000000f32,  0.75f32, // C
   -0.5f32,   0.5f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.33333333f32,  0.5f32, // B
   -0.5f32,  -0.5f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0000000f32,  0.5f32, // D

    0.5f32,   0.5f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.33333333f32,  0.5f32, // B
    0.5f32,   0.5f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.33333333f32,  0.75f32, // A
    0.5f32,  -0.5f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0000000f32,  0.75f32, // C LEFT
    0.5f32,  -0.5f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0000000f32,  0.75f32, // C
    0.5f32,  -0.5f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0000000f32,  0.5f32, // D
    0.5f32,   0.5f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.33333333f32,  0.5f32, // B

   -0.5f32,  -0.5f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.3333333f32,  0.25f32,
   0.5f32,  -0.5f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.6666666f32,  0.0f32, // BOTTOM
    0.5f32,  -0.5f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.6666666f32,  0.25f32,
    0.5f32,  -0.5f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.6666666f32,  0.0f32,
    -0.5f32,  -0.5f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.3333333f32,  0.25f32,
   -0.5f32,  -0.5f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.3333333f32,  0.0f32,

   -0.5f32,   0.5f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.3333333f32,  0.75f32,
    0.5f32,   0.5f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.6666666f32,  0.75f32,
    0.5f32,   0.5f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.6666666f32,  0.5f32,
    0.5f32,   0.5f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.6666666f32,  0.5f32, // TOP
   -0.5f32,   0.5f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.3333333f32,  0.5f32,
   -0.5f32,   0.5f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.3333333f32,  0.75f32
];

pub static TEXTURE_CUBE_INDICES: [u32; 36] = [
    0, 2, 1, 3, 5, 4, 
    6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 16, 17, // GOOD
    18, 20, 19, 21, 23, 22, // GOOD
    24, 25, 26, 27, 28, 29, // GOOD
    30, 32, 31, 33, 35, 34, // GOOD
];