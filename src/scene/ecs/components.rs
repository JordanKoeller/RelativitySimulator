use utils::*;

use super::Entity;
use super::super::Scene;
use renderer::RenderCommand;

use physics::{AABB, RigidBody, CollisionPriority};




pub trait Kinematics: Entity {
  fn rigid_body(&self) -> &RigidBody;
  fn rigid_body_mut(&mut self) -> &mut RigidBody;
}

pub trait Script: Entity {
  fn run_script(&self, timestep: Timestep, scene: MutRef<Scene>);
}

pub trait Renderable: Entity {
  fn draw(&self) -> RenderCommand;

}


pub trait Collision: Entity {
  fn rigid_body(&self) -> &RigidBody;
  fn rigid_body_mut(&mut self) -> &mut RigidBody;

  fn aabb(&self) -> &AABB;
  fn collision_priority(&self) -> CollisionPriority;

}





// impl Kinematics for Collision {
//   fn rigid_body(&self) -> &RigidBody {
//     <Self as Collision>::rigid_body(self)
//   }
//   fn rigid_body_mut(&mut self) -> &mut RigidBody {
//     <Self as Collision>::rigid_body_mut(self)
//   }
// }