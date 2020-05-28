use cgmath;
use cgmath::prelude::*;
use cgmath::Vector3;


type Vec3 = cgmath::Vector3<f32>;


pub const LIGHT_SPEED: f32 = 300.0;

pub trait Movable {
  // Some helper functions exposing internal state in immutable way
  fn position(&self) -> &Vec3;
  fn velocity(&self) -> &Vec3;
  fn acceleration(&self) -> &Vec3;
  fn set_position(&mut self, v: Vec3);
  fn set_velocity(&mut self, v: Vec3);
  fn set_acceleration(&mut self, v: Vec3);

  fn beta(&self) -> f32 {
    self.velocity().magnitude() / LIGHT_SPEED
  }

  fn clear_acceleration(&mut self) {
    self.set_acceleration(Vec3 { x: 0.0, y: 0.0, z: 0.0 });
  }

  fn apply_acceleration(&mut self, v: Vec3) {
    self.set_acceleration(self.acceleration() + v);
  }

  fn integrate(&mut self, dt: f32) {
    // vf = vi + at
    // d = d0 + vit + 1/2at^2
    self.set_velocity(self.velocity() + self.acceleration()*dt);
    self.set_position(self.position() + self.velocity()*dt + self.acceleration()*dt*dt/2.0);
  }
}