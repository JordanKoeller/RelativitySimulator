use cgmath::prelude::*;
use crate::utils::*;
use crate::renderer::LIGHT_SPEED;
use crate::ecs::components::Rotation;

pub struct Camera<'a> {
  pub position: &'a Vec3F,
  pub velocity: &'a Vec3F,
  pub rotation: &'a Rotation,
}

impl<'a> Camera<'a> {

  pub fn new(pos: &'a Vec3F, vel: &'a Vec3F, rot: &'a Rotation) -> Self {
    Camera {
      position: pos,
      velocity: vel,
      rotation: rot
    }
  }




  pub fn projection_matrix(&self, dims: &Vec2F) -> Mat4F {
    cgmath::perspective(cgmath::Deg(45f32), dims.x / dims.y, 0.1, 10000.0)
  }

  pub fn view_matrix(&self) -> Mat4F {
    let pt_pos = cgmath::Point3::<f32>::new(self.position.x, self.position.y, self.position.z);
    Mat4F::look_at(pt_pos, pt_pos + self.rotation.front(), self.rotation.up())
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
      let right = vel_norm.cross(self.rotation.world_up()).normalize();
      let up = right.cross(vel_norm);
      cgmath::Matrix3::<f32>::from_cols(vel_norm, right, up).transpose()
    }
  }

  pub fn velocity_inverse_basis_matrix(&self) -> Mat3F {
    self.velocity_basis_matrix().invert().expect("Could not invert matrix")
  }

}