use specs::{Join, Read, ReadStorage, System, WriteStorage};
use cgmath::prelude::*;
use utils::*;
use physics::{TransformComponent, RigidBody, Gravity, Drag, LIGHT_SPEED};

pub const MAX_ACCELERATION: f32 = 6f32;

pub const DRAG: f32 = MAX_ACCELERATION / LIGHT_SPEED / LIGHT_SPEED;
pub const GRAVITY: f32 = 14f32;


struct KinematicsCalculator;

impl KinematicsCalculator {

  fn compute_kinematics(&self, rigid_body: &mut RigidBody, gravity: Option<&Gravity>, drag: Option<&Drag>, dt: f32) {
    if let Some(_g) = gravity {
      rigid_body.acceleration -= Vec3F::unit_y() * GRAVITY;
    }
    if let Some(_d) = drag {
      if rigid_body.velocity.magnitude2() > 0.1 {
        rigid_body.acceleration -= DRAG * rigid_body.velocity.magnitude2() * rigid_body.velocity.normalize();
      }
    }
    if rigid_body.acceleration.magnitude2() > 0.000001 {
      rigid_body.velocity += rigid_body.acceleration * dt;
    }
    rigid_body.acceleration = Vec3F::new(0.0, 0.0, 0.0);
  }
}