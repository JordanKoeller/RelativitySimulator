use std::ops::Deref;

use specs::prelude::*;
use specs::{Component, VecStorage, NullStorage, DefaultVecStorage};

use events::ReceiverID;
use utils::*;

use cgmath::prelude::*;
use ecs::Debugger;

use renderer::LIGHT_SPEED;

#[derive(Debug)]
pub struct Position(pub Vec3F);

impl Component for Position {
  type Storage = DefaultVecStorage<Self>;
}

impl Default for Position {
  fn default() -> Self {
    Position(Vec3F::new(0f32, 0f32, 0f32))
  }
}

#[derive(Debug)]
pub struct Kinetics {
  pub velocity: Vec3F,
  pub acceleration: Vec3F,
}

impl Component for Kinetics {
  type Storage = DefaultVecStorage<Self>;
}

impl Kinetics {
  #[allow(dead_code)]
  pub fn speed(&self) -> f32 {
    self.velocity.magnitude()
  }
}

impl Default for Kinetics {
  fn default() -> Self {
    Kinetics {
      velocity: Vec3F::new(0f32, 0f32, 0f32),
      acceleration: Vec3F::new(0f32, 0f32, 0f32),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Rotation(pub Vec2F);

impl Rotation {
  pub fn front(&self) -> Vec3F {
    Vec3F {
      x: self.0.y.to_radians().cos() * self.0.x.to_radians().cos(),
      y: self.0.x.to_radians().sin(),
      z: self.0.y.to_radians().sin() * self.0.x.to_radians().cos(),
    }
    .normalize()
  }

  pub fn right(&self) -> Vec3F {
    self.front().cross(self.world_up()).normalize()
  }

  pub fn up(&self) -> Vec3F {
    self.right().cross(self.front()).normalize()
  }

  pub fn world_up(&self) -> Vec3F {
    Vec3F::unit_y()
  }

  pub fn rotate(&mut self, xoffset: f32, yoffset: f32) {
    // println!("Rotating");
    self.0.y += xoffset;
    self.0.x += yoffset;
    // Make sure that when pitch is out of bounds, screen doesn't get flipped
    if self.0.x > 89.0 {
      self.0.x = 89f32;
    }
    if self.0.x < -89.0 {
      self.0.x = -89f32;
    }
    // Update Front, Right and Up Vectors using the updated Euler angles
  }
}

impl Component for Rotation {
  type Storage = VecStorage<Self>;
}

impl Default for Rotation {
  fn default() -> Self {
    Rotation(Vec2F::new(0f32, 0f32))
  }
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Player;

#[derive(Component, Default, Debug)]
#[storage(DefaultVecStorage)]
pub struct Gravity;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Drag;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct EventReceiver(pub ReceiverID);

#[derive(Component, Debug, Clone)]
#[storage(DefaultVecStorage)]
pub struct Transform(pub Mat4F);

impl Default for Transform {
  fn default() -> Self {
    Self(Mat4F::one())
  }
}

impl Deref for Transform {
  type Target = Mat4F;

  fn deref(&self) -> &Mat4F {
    &self.0
  }
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Camera {
  perspective: Mat4F,
  velocity: Vec3F,
}

impl Default for Camera {
  fn default() -> Self {
    Self {
      perspective: Mat4F::one(),
      velocity: Vec3F::new(0f32, 0f32, 0f32),
    }
  }
}

impl Camera {
  pub fn new(pers: Mat4F, vel: Vec3F) -> Self {
    Self {
      perspective: pers,
      velocity: vel
    }
  }
  pub fn projection_matrix(&self, dims: &Vec2F) -> Mat4F {
    cgmath::perspective(cgmath::Deg(45f32), dims.x / dims.y, 0.1, 10000.0)
  }

  pub fn view_matrix(&self) -> &Mat4F {
    &self.perspective
  }

  pub fn beta(&self) -> f32 {
    self.velocity.magnitude() / LIGHT_SPEED
  }

  pub fn beta2(&self) -> f32 {
    self.velocity.magnitude2() / LIGHT_SPEED / LIGHT_SPEED
  }

  pub fn gamma(&self) -> f32 {
    (1.0 - self.beta2()).powf(-0.5)
  }

  pub fn velocity_basis_matrix(&self) -> Mat3F {
    if self.beta() == 0.0 {
      cgmath::Matrix3::<f32>::identity()
    } else {
      let vel_norm = self.velocity.normalize();
      let right = vel_norm.cross(Vec3F::unit_y()).normalize();
      let up = right.cross(vel_norm);
      cgmath::Matrix3::<f32>::from_cols(vel_norm, right, up).transpose()
    }
  }

  pub fn velocity_inverse_basis_matrix(&self) -> Mat3F {
    self.velocity_basis_matrix().invert().expect("Could not invert matrix")
  }

  pub fn set_matrix(&mut self, mat: Mat4F) {
    self.perspective = mat;
  }
}