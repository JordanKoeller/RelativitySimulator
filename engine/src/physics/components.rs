use crate::utils::*;
use cgmath::prelude::*;
use cgmath::{Deg, Rad, Rotation3};
use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct TransformComponent {
  pub translation: Vec3F,
  pub scale: Vec3F,
  pub rotation: QuatF,
}

impl TransformComponent {
  pub fn new(translation: Vec3F, scale: Vec3F, rotation: QuatF) -> Self {
    Self {
      translation,
      scale,
      rotation,
    }
  }

  pub fn identity() -> Self {
    Self {
      translation: Vec3F::zero(),
      scale: Vec3F::new(1f32, 1f32, 1f32),
      rotation: QuatF::one(),
    }
  }

  pub fn from_buffer(data: [f32; 10]) -> Self {
    let translation = Vec3F::from([data[0], data[1], data[2]]);
    let scale = Vec3F::from([data[3], data[4], data[5]]);
    let rotation = QuatF::new(data[6], data[7], data[8], data[9]);
    Self {
      translation,
      rotation,
      scale,
    }
  }

  pub fn matrix(&self) -> Mat4F {
    let rotation_3 = Mat3F::from(self.rotation);
    let mut rotation_4 = Mat4F::from(rotation_3);
    rotation_4.w.w = 1f32;
    Mat4F::from_translation(self.translation) * rotation_4 * nonunif_scale(self.scale)
  }

  pub fn push_translation(&mut self, dr: Vec3F) {
    self.translation = dr + self.translation;
  }

  pub fn push_scale(&mut self, ds: Vec3F) {
    self.scale = Vec3F::new(self.scale.x * ds.x, self.scale.y * ds.y, self.scale.z * ds.z);
  }

  pub fn push_rotation(&mut self, dtheta: &QuatF) {
    self.rotation = (dtheta * self.rotation).normalize();
  }

  pub fn front(&self) -> Vec3F {
    self.rotation.rotate_vector(Vec3F::unit_z()).normalize()
  }

  pub fn right(&self) -> Vec3F {
    -self.rotation.rotate_vector(Vec3F::unit_x()).normalize()
    // -self.up().cross(self.front()).normalize()
  }

  pub fn up(&self) -> Vec3F {
    self.rotation.rotate_vector(Vec3F::unit_y()).normalize()
  }

  pub fn world_up(&self) -> Vec3F {
    Vec3F::unit_y()
  }

  pub fn matrix_buffer(&self) -> [f32; 10] {
    [
      self.translation.x,
      self.translation.y,
      self.translation.z,
      self.scale.x,
      self.scale.y,
      self.scale.z,
      self.rotation.s,
      self.rotation.v.x,
      self.rotation.v.y,
      self.rotation.v.z,
    ]
  }
}

impl Default for TransformComponent {
  fn default() -> Self {
    Self::new(Vec3F::zero(), Vec3F::new(1f32, 1f32, 1f32), QuatF::one())
  }
}

#[derive(Component, Debug, Clone, Default)]
#[storage(NullStorage)]
pub struct Drag;

#[derive(Component, Debug, Clone, Default)]
#[storage(NullStorage)]
pub struct Gravity;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct RigidBody {
  pub velocity: Vec3F,
  pub acceleration: Vec3F,
  pub angular_velocity: QuatF,     // Euler Angles
  pub angular_acceleration: QuatF, // Euler Angles
}

impl Default for RigidBody {
  fn default() -> Self {
    Self {
      velocity: Vec3F::zero(),
      acceleration: Vec3F::zero(),
      angular_velocity: QuatF::one(),
      angular_acceleration: QuatF::one(),
    }
  }
}

impl RigidBody {
  pub fn new_stationary() -> Self {
    Self {
      velocity: Vec3F::zero(),
      acceleration: Vec3F::zero(),
      angular_velocity: QuatF::one(),
      angular_acceleration: QuatF::one(),
    }
  }

  pub fn new(velocity: Vec3F, acceleration: Vec3F, angular_velocity: QuatF, angular_acceleration: QuatF) -> Self {
    Self {
      velocity,
      acceleration,
      angular_velocity,
      angular_acceleration,
    }
  }

  pub fn reset_acceleration(&mut self) {
    self.acceleration = Vec3F::zero();
    self.angular_acceleration = QuatF::zero();
  }
}
