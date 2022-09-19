use std::ops::Deref;

use cgmath::prelude::*;
use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

use crate::utils::*;

const DEG_89: cgmath::Rad<f32> = cgmath::Rad(1.5533430342749532f32);

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Camera {
  position: Vec3F,
  euler_angles: cgmath::Euler<cgmath::Rad<f32>>,
  fovy: cgmath::Rad<f32>,
  near_distance: f32,
  far_distance: f32,
}

impl Default for Camera {
  fn default() -> Self {
    Self {
      position: Vec3F::new(0f32, 0f32, 0f32),
      euler_angles: cgmath::Euler::new(cgmath::Rad(0f32), cgmath::Rad(0f32), cgmath::Rad(0f32)),
      fovy: cgmath::Deg(45f32).into(),
      near_distance: 0.1f32,
      far_distance: 1000f32,
    }
  }
}

impl Camera {
  pub fn new(position: Vec3F, facing: Vec3F) -> Self {
    let facing = facing.normalize();
    let euler_angles = cgmath::Euler::new(
      cgmath::Rad(facing.y.sin()),
      cgmath::Rad(facing.z.atan2(facing.x)),
      cgmath::Rad(0f32),
    );
    let mut ret = Self::default();
    ret.position = position;
    ret.euler_angles = euler_angles;
    ret
  }

  // Camera matrices
  pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4F {
    cgmath::perspective(self.fovy, aspect_ratio, self.near_distance, self.far_distance)
  }

  pub fn view_matrix(&self) -> Mat4F {
    let facing = self.front();
    let location = cgmath::Point3::<f32>::new(self.position.x, self.position.y, self.position.z);
    Mat4F::look_at_dir(location, facing, Vec3F::unit_y())
  }

  // Getters and Setters
  pub fn position(&self) -> Vec3F {
    self.position
  }

  pub fn push_translation(&mut self, delta: Vec3F) {
    self.position += delta;
  }

  pub fn push_rotation(&mut self, delta: cgmath::Euler<cgmath::Rad<f32>>) {
    self.euler_angles = cgmath::Euler::new(
      self.euler_angles.x + delta.x,
      self.euler_angles.y + delta.y,
      self.euler_angles.z + delta.z,
    );
    if self.euler_angles.x > DEG_89 {
      self.euler_angles.x = DEG_89;
    }
    if self.euler_angles.x < -DEG_89 {
      self.euler_angles.x = -DEG_89;
    }
  }

  pub fn front(&self) -> Vec3F {
    Vec3F::new(
      self.euler_angles.y.cos() * self.euler_angles.x.cos(),
      self.euler_angles.x.sin(),
      self.euler_angles.y.sin() * self.euler_angles.x.cos(),
    )
    .normalize()
  }

  pub fn right(&self) -> Vec3F {
    self.front().cross(Vec3F::unit_y()).normalize()
  }

  pub fn up(&self) -> Vec3F {
    self.right().cross(self.front()).normalize()
  }

  pub fn facing_matrix(&self) -> [f32; 3] {
    let facing = self.front();
    return [
      facing.x, facing.y, facing.z,
    ]

  }
}
