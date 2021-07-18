use specs::{Join, Read, ReadStorage, System, WriteStorage};

use cgmath::prelude::*;
use physics::{CanCollide, Collision, CollisionQueue, CollisionSummary};
use utils::*;

use physics::{TransformComponent, RigidBody, Gravity, Drag};

use app::AxisAlignedCubeCollision;

use renderer::LIGHT_SPEED;
const MAX_ACCELERATION: f32 = 6f32;

const DRAG: f32 = MAX_ACCELERATION / LIGHT_SPEED / LIGHT_SPEED;
const GRAVITY: f32 = 14f32;

pub struct MotionSystem;

impl<'a> System<'a> for MotionSystem {
  type SystemData = (
    WriteStorage<'a, TransformComponent>,
    WriteStorage<'a, RigidBody>,
    ReadStorage<'a, CanCollide>,
    ReadStorage<'a, Gravity>,
    ReadStorage<'a, Drag>,
    ReadStorage<'a, AxisAlignedCubeCollision>,
    Read<'a, Timestep>,
  );

  fn run(
    &mut self,
    (mut transform_s, mut rigid_storage, collidable_storage, gravity_storage, drag_storage, colliders_storage, dt): Self::SystemData,
  ) {
    for (transform, collidable, rigid_body, gravity, drag) in (
      &mut transform_s,
      (&collidable_storage).maybe(),
      &mut rigid_storage,
      (&gravity_storage).maybe(),
      (&drag_storage).maybe(),
    )
      .join()
    {
      self.compute_kinematics(rigid_body, gravity, drag, dt.click);
      transform.push_translation(rigid_body.velocity * dt.click);
    }
  }
}

impl MotionSystem {
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
      // } else if norm_acc.magnitude2() > 0.1 {
      //   kinetics.velocity += norm_acc * dt;
      // }
    }
    rigid_body.acceleration = Vec3F::new(0.0, 0.0, 0.0);
  }
}
