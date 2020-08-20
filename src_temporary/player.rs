use cgmath::prelude::*;
use cgmath::Vector3;

use camera::Camera;
use physics::{Movable, LIGHT_SPEED};

type Vec3 = Vector3<f32>;
type Point3 = cgmath::Point3<f32>;

pub struct Player {
  pub position: Vec3,
  pub velocity: Vec3,
  pub acceleration: Vec3,

  pub front: Vec3,
  pub up: Vec3,
  pub right: Vec3,
  pub world_up: Vec3,
  // Euler Angles
  pub yaw: f32,
  pub pitch: f32,
  // Camera options
  pub movement_speed: f32,
  pub mouse_sensitivity: f32,
  pub zoom: f32,
}

impl Player {
  pub fn pos(&self) -> Vec3 {
    self.position
  }
}

impl Default for Player {
  fn default() -> Player {
    let mut cam = Player {
      position: Vec3::new(0.0, 0.0, 0.0),
      velocity: Vec3::zero(),
      acceleration: Vec3::zero(),
      front: Vec3::new(0.0, 0.0, -1.0),
      up: Vec3::zero(),
      right: Vec3::zero(),
      world_up: Vec3::unit_y(),
      yaw: -90.0,
      pitch: 0.0,
      movement_speed: 2.5,
      mouse_sensitivity: 0.1,
      zoom: 45.0,
    };
    cam.update_camera_vectors();
    cam
  }
}

impl Movable for Player {
  fn position(&self) -> &Vec3 {
    &self.position
  }
  fn velocity(&self) -> &Vec3 {
    &self.velocity
  }
  fn acceleration(&self) -> &Vec3 {
    &self.acceleration
  }
  fn set_position(&mut self, v: Vec3) {
    self.position = v;
  }
  fn set_velocity(&mut self, v: Vec3) {
    if v.magnitude() > LIGHT_SPEED {
      let dir = v.normalize_to(LIGHT_SPEED - 0.01);
      self.velocity = dir;
    } else {
      self.velocity = v;
    }
  }
  fn set_acceleration(&mut self, v: Vec3) {
    self.acceleration = v;
  }
}

impl Camera for Player {
  fn position(&self) -> Point3 {
    Point3::new(self.position.x, self.position.y, self.position.z)
  }
  fn front(&self) -> Vec3 {
    self.front
  }
  fn set_front(&mut self, v: Vec3) {
    self.front = v;
  }
  fn up(&self) -> Vec3 {
    self.up
  }
  fn set_up(&mut self, v: Vec3) {
    self.up = v;
  }
  fn right(&self) -> Vec3 {
    self.right
  }
  fn set_right(&mut self, v: Vec3) {
    self.right = v;
  }
  fn world_up(&self) -> Vec3 {
    Vec3::unit_y()
  }
  fn yaw(&self) -> f32 {
    self.yaw
  }
  fn set_yaw(&mut self, v: f32) {
    self.yaw = v;
  }
  fn pitch(&self) -> f32 {
    self.pitch
  }
  fn set_pitch(&mut self, v: f32) {
    self.pitch = v;
  }
  fn camera_speed(&self) -> f32 {
    self.movement_speed
  }
  fn mouse_sensitivity(&self) -> f32 {
    self.mouse_sensitivity
  }
  fn zoom(&self) -> f32 {
    self.zoom
  }
}

impl Player {
  pub fn velocity_basis_matrix(&self) -> cgmath::Matrix3<f32> {
    if self.beta() == 0.0 {
      cgmath::Matrix3::<f32>::identity()
    } else {
      let vel_norm = self.velocity().normalize();
      let right = vel_norm.cross(self.world_up()).normalize();
      let up = right.cross(vel_norm);
      cgmath::Matrix3::<f32>::from_cols(vel_norm, right, up).transpose()
    }
  }

  pub fn velocity_inverse_basis_matrix(&self) -> cgmath::Matrix3<f32> {
    let ret: cgmath::Matrix3<f32> = self.velocity_basis_matrix().invert().expect("Could not invert matrix");
    ret
  }
}
