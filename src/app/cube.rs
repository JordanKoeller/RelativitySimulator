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

pub struct TexturedCube {
  filename: String
}

impl Drawable for TexturedCube {
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
    "lorentz".to_string()
  }
}


pub struct Cube {
  material: Material,
}

impl Cube {
  pub fn new(material: Material) -> Cube {

    Cube {
      material,
    }
  }
}

impl Drawable for Cube {
  fn vertex_array(&self) -> VertexArray {
    let layout = BufferLayout::new(vec![AttributeType::Float3, AttributeType::Float3, AttributeType::Float2]);
    let vert_buff = DataBuffer::static_buffer(&TEXTURE_CUBE_VERTICES, layout);
    let ind_buff = IndexBuffer::create(TEXTURE_CUBE_INDICES.to_vec());
    VertexArray::new(vert_buff, ind_buff)
  }
  fn material(&self) -> Material {
    self.material.clone()
  }

  fn shader_name(&self) -> String {
    "lorentz".to_string()
  }
}

pub struct FaceCube {
  pub c: Cube,
}

impl Drawable for FaceCube {
  fn vertex_array(&self) -> VertexArray {
    self.c.vertex_array()
  }
  fn material(&self) -> Material {
    self.c.material()
  }

  fn shader_name(&self) -> String {
    "face_cube".to_string()
  }
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct AxisAlignedCubeCollision {
  center: Vec3F,
  dims: Vec3F
}

impl AxisAlignedCubeCollision {

  pub fn from_transform(transform: &TransformComponent) -> Self {
    let c1 = Vec3F::new(-0.5f32, -0.5f32, -0.5f32);
    let c2 = Vec3F::new(0.5f32, 0.5f32, 0.5f32);
    let matrix = transform.matrix();
    let p1 = matrix * swizzle_up(&c1);
    let p2 = matrix * swizzle_up(&c2);
    let center = (p1 + p2) / 2f32;
    let dims = p2 - p1;
    Self {
      center: swizzle_down(&center),
      dims: swizzle_down(&dims),
    }
  }

  fn within_box(&self, pt: &Vec3F, bl: &Vec3F, tr: &Vec3F) -> bool {
    self.approx_between(&bl.x, &tr.x, &pt.x) &&
    self.approx_between(&bl.y, &tr.y, &pt.y) &&
    self.approx_between(&bl.z, &tr.z, &pt.z)
  }
}

impl Collision for AxisAlignedCubeCollision {

  fn distance_to(&self, pt: &Vec3F) -> f32 {
    0f32
  }

  fn sphere_collision(&self, sphere: (&Vec3F, &f32), velocity: &Vec3F) -> Option<CollisionSummary> {
    let new_dims = self.dims + Vec3F::new(1f32, 1f32, 1f32) * *sphere.1 * 2f32;
    let c = sphere.0;

    let lows = self.center - new_dims / 2f32;
    let highs = self.center + new_dims / 2f32;

    let checks = [
      (&lows, &-Vec3F::unit_x()),
      (&lows, &-Vec3F::unit_y()),
      (&lows, &-Vec3F::unit_z()),
      (&highs, &Vec3F::unit_x()),
      (&highs, &Vec3F::unit_y()),
      (&highs, &Vec3F::unit_z()),
    ];

    checks.iter().fold(None, |acc, elem| {
      if let Some(summary) = self.get_collision(elem.0, &c, velocity, elem.1) {
        if summary.time >= 0f32 && self.within_box(&summary.position, &lows, &highs) {
          if let Some(prev_best) = acc {
            if summary.time < prev_best.time {
              Some(summary)
            } else {
              Some(prev_best)
            }
          } else {
            Some(summary)
          }
        } else {
          acc
        }
      } else {
        acc
      }
    })
  }
}



pub static TEXTURE_CUBE_VERTICES: [f32; 288] = [
    // positions       // normals        // texture coords
    -0.5f32, 0f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32,
    0.5f32,  0f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  1.0f32,  0.0f32,
    0.5f32,  1f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  1.0f32,  1.0f32,
    0.5f32,  1f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  1.0f32,  1.0f32,
   -0.5f32,  1f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.0f32,  1.0f32,
   -0.5f32,  0f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32,

   -0.5f32,  0f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32,
    0.5f32,  0f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  1.0f32,  0.0f32,
    0.5f32,  1f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  1.0f32,  1.0f32,
    0.5f32,  1f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  1.0f32,  1.0f32,
   -0.5f32,  1f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,  1.0f32,
   -0.5f32,  0f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32,

   -0.5f32,  1f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,
   -0.5f32,  1f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32,  1.0f32,
   -0.5f32,  0f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0f32,  1.0f32,
   -0.5f32,  0f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0f32,  1.0f32,
   -0.5f32,  0f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0f32,  0.0f32,
   -0.5f32,  1f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,

    0.5f32,  1f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,
    0.5f32,  1f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32,  1.0f32,
    0.5f32,  0f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0f32,  1.0f32,
    0.5f32,  0f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0f32,  1.0f32,
    0.5f32,  0f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0f32,  0.0f32,
    0.5f32,  1f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,

   -0.5f32,  0f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32,
    0.5f32,  0f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  1.0f32,  1.0f32,
    0.5f32,  0f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  1.0f32,  0.0f32,
    0.5f32,  0f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  1.0f32,  0.0f32,
   -0.5f32,  0f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32,  0.0f32,
   -0.5f32,  0f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32,

   -0.5f32,  1f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32,
    0.5f32,  1f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  1.0f32,  1.0f32,
    0.5f32,  1f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  1.0f32,  0.0f32,
    0.5f32,  1f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  1.0f32,  0.0f32,
   -0.5f32,  1f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32,  0.0f32,
   -0.5f32,  1f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32
];

pub static TEXTURE_CUBE_INDICES: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];