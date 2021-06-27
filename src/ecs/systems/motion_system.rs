use specs::{Join, Read, ReadStorage, System, WriteStorage};

use cgmath::prelude::*;
use ecs::components::{Gravity, Kinetics, Position, Transform, Drag};
use ecs::{CanCollide, Collision, CollisionQueue, CollisionSummary};
use utils::*;

use app::AxisAlignedCubeCollision;

use renderer::LIGHT_SPEED;
const MAX_ACCELERATION: f32 = 6f32;

const DRAG: f32 = MAX_ACCELERATION / LIGHT_SPEED / LIGHT_SPEED;
const GRAVITY: f32 = 14f32;

pub struct MotionSystem;

impl<'a> System<'a> for MotionSystem {
  type SystemData = (
    WriteStorage<'a, Position>,
    WriteStorage<'a, Transform>,
    ReadStorage<'a, CanCollide>,
    WriteStorage<'a, Kinetics>,
    ReadStorage<'a, Gravity>,
    ReadStorage<'a, Drag>,
    ReadStorage<'a, AxisAlignedCubeCollision>,
    Read<'a, Timestep>,
  );

  fn run(
    &mut self,
    (mut pos_storage, mut transform_s, collidable_storage, mut kin_storage, gravity_storage, drag_storage, colliders_storage, dt): Self::SystemData,
  ) {
    for (position, transform, collidable, kinetics, gravity, drag) in (
      &mut pos_storage,
      (&mut transform_s).maybe(),
      (&collidable_storage).maybe(),
      &mut kin_storage,
      (&gravity_storage).maybe(),
      (&drag_storage).maybe(),
    )
      .join()
    {
      self.compute_kinematics(kinetics, gravity, drag, dt.0);
      let mut remaining_time = dt.0;
      if let Some(sphere) = collidable {
        let speculative_collidables: Vec<&AxisAlignedCubeCollision> = colliders_storage
          .join()
          .filter(|&cube| cube.distance_to(&position.0) < kinetics.speed() * dt.0 + sphere.radius)
          .collect();
        let mut more_work = speculative_collidables.len() > 0;
        while more_work {
          more_work = false;
          if let Some(collision_summary) =
            speculative_collidables
              .iter()
              .fold(None, |acc: Option<CollisionSummary>, cube| {
                cube
                  .sphere_collision((&position.0, &sphere.radius), &kinetics.velocity)
                  .map_or_else(
                    || acc,
                    |summary| {
                      if summary.time < remaining_time {
                        if let Some(best) = acc {
                          if best.time < summary.time {
                            Some(best)
                          } else {
                            Some(summary)
                          }
                        } else {
                          Some(summary)
                        }
                      } else {
                        acc
                      }
                    },
                  )
              })
          {
            // Process the next collision in here.
            more_work = true;
            remaining_time = remaining_time - collision_summary.time;
            position.0 = collision_summary.position + collision_summary.surface_normal * 0.01;
            let canceled_velocity = kinetics.velocity
              - collision_summary.surface_normal * collision_summary.surface_normal.dot(kinetics.velocity);
            kinetics.velocity = canceled_velocity;
          }
        }
      }
      let dr = kinetics.velocity * remaining_time;
      position.0 += dr;
      if let Some(matrix) = transform {
        matrix.0 = Mat4F::from_translation(dr) * matrix.0;
      }
    }
  }
}

impl MotionSystem {
  fn compute_kinematics(&self, kinetics: &mut Kinetics, gravity: Option<&Gravity>, drag: Option<&Drag>, dt: f32) {
    if let Some(_g) = gravity {
      kinetics.acceleration -= Vec3F::unit_y() * GRAVITY;
    }
    if let Some(_d) = drag {
      if kinetics.velocity.magnitude2() > 0.1 {
        kinetics.acceleration -= DRAG * kinetics.velocity.magnitude2() * kinetics.velocity.normalize();
      }
    }
    if kinetics.acceleration.magnitude2() > 0.000001 {
      kinetics.velocity += kinetics.acceleration * dt;
      // } else if norm_acc.magnitude2() > 0.1 {
      //   kinetics.velocity += norm_acc * dt;
      // }
    }
    kinetics.acceleration = Vec3F::new(0.0, 0.0, 0.0);
  }
}
