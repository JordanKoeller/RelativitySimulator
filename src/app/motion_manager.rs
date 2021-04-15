use specs::{Join, Read, ReadStorage, System, WriteStorage};

use cgmath::prelude::*;
use ecs::components::{Kinetics, Position};
use ecs::{CanCollide, Collision, CollisionQueue, CollisionSummary};
use utils::*;

use app::AxisAlignedCubeCollision;

use renderer::LIGHT_SPEED;
const MAX_ACCELERATION: f32 = 6f32;

const DRAG: f32 = MAX_ACCELERATION / LIGHT_SPEED / LIGHT_SPEED;

pub struct Motion;

impl<'a> System<'a> for Motion {
  type SystemData = (
    WriteStorage<'a, Position>,
    ReadStorage<'a, CanCollide>,
    WriteStorage<'a, Kinetics>,
    ReadStorage<'a, AxisAlignedCubeCollision>,
    Read<'a, Timestep>,
  );

  fn run(&mut self, (mut pos_storage, collidable_storage, mut kin_storage, colliders_storage, dt): Self::SystemData) {
    for (position, collidable, kinetics) in (&mut pos_storage, (&collidable_storage).maybe(), &mut kin_storage).join() {
      self.compute_kinematics(kinetics, dt.0);
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
              .fold(None, |acc: Option<CollisionSummary>, cube| 
                cube
                  .sphere_collision((&position.0, &sphere.radius), &kinetics.velocity)
                  .map_or_else(|| {acc}, |summary| {
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
                  }))
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
      position.0 += kinetics.velocity * remaining_time;
    }
  }
}

impl Motion {
  fn compute_kinematics(&self, kinetics: &mut Kinetics, dt: f32) {
    let norm_acc = if kinetics.acceleration.magnitude2() != 0.0 {
      let norm_acc = kinetics.acceleration.normalize_to(MAX_ACCELERATION);
      if kinetics.velocity.magnitude2() > 0.1 {
        norm_acc - DRAG * kinetics.velocity.magnitude2() * kinetics.velocity.normalize()
      } else {
        norm_acc
      }
    } else {
      kinetics.acceleration
    };
    if kinetics.velocity.magnitude2() > 0.1 {
      kinetics.velocity += norm_acc * dt;
    } else if norm_acc.magnitude2() > 0.1 {
      kinetics.velocity += norm_acc * dt;
    }
    kinetics.acceleration = Vec3F::new(0.0, 0.0, 0.0);
  }
}
