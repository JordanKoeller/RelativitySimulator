use cgmath::prelude::*;
use crate::utils::*;
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

}