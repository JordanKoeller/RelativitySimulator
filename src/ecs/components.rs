use specs::{Component, VecStorage};
use specs::prelude::*;

use events::ReceiverID;
use renderer::{Material, VertexArray};
use utils::*;

use cgmath::prelude::*;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Position(pub Vec3F);

impl Component for Position {
  type Storage = VecStorage<Self>;
}

impl Default for Position {
  fn default() -> Self {
    Position(Vec3F::new(0f32, 0f32, 0f32))
  }
}

#[derive(Debug)]
pub struct Kinetics {
  pub velocity: Vec3F,
  pub acceleration: Vec3F
}

impl Component for Kinetics {
  type Storage = VecStorage<Self>;
}

impl Kinetics {
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
      }.normalize()
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
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct EventReceiver(pub ReceiverID);


#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Transform(pub Mat4F);

// #[derive(Component, Debug, Clone)]
// #[storage(VecStorage)]
// pub struct ImguiElement(pub UiCommand);