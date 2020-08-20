use cgmath::prelude::*;
use cgmath::Vector3;
type Vec3 = Vector3<f32>;

struct Particle {
  pub position: Vec3,
  pub velocity: Vec3,
  pub acceleration: Vec3,
}

impl Movable for Particle {
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
    self.velocity = v;
  }
  fn set_acceleration(&mut self, v: Vec3) {
    self.acceleration = v;
  }
}
