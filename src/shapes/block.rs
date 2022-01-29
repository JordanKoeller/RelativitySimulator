use std::ops::Deref;
use specs::prelude::*;
use specs::{Component, VecStorage};
use cgmath::prelude::{InnerSpace};
use renderer::{Drawable, Texture};
use renderer::{AttributeType, BufferLayout, IndexBuffer, VertexArray, DataBuffer};

use physics::{Collision, CollisionSummary};

use utils::{Vec3F, Vec4F, swizzle_down, swizzle_up, Mat3F, Vec3I};
use physics::TransformComponent;

use ecs::Material;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockFace {
  Left,
  Right,
  Top,
  Bottom,
  Front,
  Back,
}

impl BlockFace {
    pub fn buffer_info(self, translation: Vec3F, starting_index: u32) -> ([f32; 48], [u32; 6]) {
      let mut output_coords = [0f32; 48];
      let mut output_inds = [0u32; 6];
      match self {
        BlockFace::Right => {
          output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[144..192]);
          output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
        },
        BlockFace::Left => {
          output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[96..144]);
          output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
        },
        BlockFace::Front => {
          output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[0..48]);
          output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
        },
        BlockFace::Back => {
          output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[48..96]);
          output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
        },
        BlockFace::Bottom => {
          output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[192..240]);
          output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
        },
        BlockFace::Top => {
          output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[240..288]);
          output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
        },
      };
      for i in 0..6 {
        output_inds[i] += starting_index;
      }
      for i in 0..6 {
        let t_x = i * 8;
        output_coords[t_x] += translation.x;
        output_coords[t_x+1] += translation.y;
        output_coords[t_x+2] += translation.z;
      }
      (output_coords, output_inds)
    }
  
    pub fn faces_array() -> [BlockFace; 6] {
      [
        BlockFace::Left,
        BlockFace::Right,
        BlockFace::Front,
        BlockFace::Back,
        BlockFace::Top,
        BlockFace::Bottom,
      ]
    }
  
  }

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

pub fn get_index_shift(side: &BlockFace) -> Vec3I {
    let mut ret = Vec3I::new(0,0,0);
    match side {
        BlockFace::Left => {ret.x = -1},
        BlockFace::Right => {ret.x = 1},
        BlockFace::Top => {ret.y = 1},
        BlockFace::Bottom => {ret.y = -1},
        BlockFace::Front => {ret.z = -1},
        BlockFace::Back => {ret.z = 1},
    }
    ret
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