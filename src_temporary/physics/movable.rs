use cgmath;
use cgmath::prelude::*;


type Vec3 = cgmath::Vector3<f32>;


pub const LIGHT_SPEED: f32 = 10.0;

pub trait Movable {
  // Some helper functions exposing internal state in immutable way
  fn position(&self) -> &Vec3;
  fn velocity(&self) -> &Vec3;
  fn acceleration(&self) -> &Vec3;
  fn set_position(&mut self, v: Vec3);
  fn set_velocity(&mut self, v: Vec3);
  fn set_acceleration(&mut self, v: Vec3);

  fn beta(&self) -> f32 {
    self.beta_helper(self.velocity())
  }

  fn beta_helper(&self, vel: &Vec3) -> f32 {
    vel.magnitude() / LIGHT_SPEED
  }

  fn beta_vector(&self) -> Vec3 {
    self.velocity() / LIGHT_SPEED
  }

  fn gamma_helper(&self, beta: f32) -> f32 {
    (1.0 - beta * beta).powf(-0.5)
  }

  fn gamma(&self) -> f32 {
    self.gamma_helper(self.beta())
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
    let new_velocity = self.velocity() + self.acceleration()*dt;
    let new_gamma = self.gamma_helper(self.beta_helper(&new_velocity));
    let dg = new_gamma - self.gamma();
    if dg > 0.0 && new_gamma / self.gamma() > 1.01 {
      // dg is too large. I want to limit the acceleration in this frame to not break that rule.
      let needed_gamma = self.gamma() * 1.01;
      // gamma = (1-v^2/c^2)^-1/2
      // gamma^-2 = (1-v^2/c^2)
      //1 - gamma^-2 = v^2/c^2
      // (1 - gamma^-2)*c^2 = v^2
      let new_v = new_velocity.normalize_to(((1.0 - needed_gamma.powf(-2.0)) * LIGHT_SPEED * LIGHT_SPEED).sqrt());
      self.set_velocity(new_v);
    } else {
      self.set_velocity(new_velocity);
    }
    self.set_position(self.position() + self.velocity()*dt + self.acceleration()*dt*dt/2.0);
  }

}