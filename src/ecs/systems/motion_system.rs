use specs::{Join, Read, ReadStorage, System, WriteStorage};

use cgmath::prelude::*;
use physics::{CanCollide, Collision, CollisionQueue, CollisionSummary};
use utils::*;

use physics::{TransformComponent, RigidBody, Gravity, Drag};

use app::AxisAlignedCubeCollision;
use app::minecraft::{ChunkGrid, ChunkComponent};

use renderer::LIGHT_SPEED;
pub const MAX_ACCELERATION: f32 = 6f32;

pub const DRAG: f32 = MAX_ACCELERATION / LIGHT_SPEED / LIGHT_SPEED;
pub const GRAVITY: f32 = 14f32;

pub struct Motion;

impl<'a> System<'a> for Motion {
  type SystemData = (
    WriteStorage<'a, TransformComponent>,
    ReadStorage<'a, CanCollide>,
    WriteStorage<'a, RigidBody>,
    Read<'a, Timestep>,
    ReadStorage<'a, Gravity>,
    Read<'a, ChunkGrid>,
    ReadStorage<'a, ChunkComponent>,
  );

  fn run(&mut self, (mut transform_storage, collide_storage, mut rigid_storage, dt, grav_storage, chunk_grid, chunk_storage): Self::SystemData) {
    for (transform, collide_opt, rigid_body, grav_opt) in (&mut transform_storage, (&collide_storage).maybe(), &mut rigid_storage, (&grav_storage).maybe()).join() {
      self.compute_kinematics(rigid_body, grav_opt, &dt);
      let mut remaining_time = dt.dt().as_secs_f32();
      if let Some(sphere) = collide_opt {
        let speculative_collidables = chunk_grid.get_nearby_collidables(&transform.translation, &chunk_storage);
        let mut more_work = speculative_collidables.len() > 0;
        while more_work {
          more_work = false;
          if let Some(collision_summary) =
            speculative_collidables
              .iter()
              .fold(None, |acc: Option<CollisionSummary>, cube| 
                cube
                  .sphere_collision((&transform.translation, &sphere.radius), &rigid_body.velocity)
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
            println!("Found collision");
            // Process the next collision in here.
            more_work = true;
            remaining_time = remaining_time - collision_summary.time;
            transform.translation = collision_summary.position + collision_summary.surface_normal * 0.01;
            let canceled_velocity = rigid_body.velocity
              - collision_summary.surface_normal * collision_summary.surface_normal.dot(rigid_body.velocity);
              rigid_body.velocity = canceled_velocity;
          }
        }
      }
      transform.translation += rigid_body.velocity * remaining_time;
    }
  }
}

impl Motion {
  fn compute_kinematics(&self, rigid_body: &mut RigidBody, gravity: Option<&Gravity>, dt: &Timestep) {
    if let Some(_g) = gravity {
      rigid_body.acceleration -= Vec3F::unit_y() * GRAVITY;
    }
    // if let Some(_d) = drag {
    //   if rigid_body.velocity.magnitude2() > 0.1 {
    //     rigid_body.acceleration -= DRAG * rigid_body.velocity.magnitude2() * rigid_body.velocity.normalize();
    //   }
    // }
    if rigid_body.acceleration.magnitude2() > 0.000001 {
      rigid_body.velocity += rigid_body.acceleration * dt.dt().as_secs_f32();
      // } else if norm_acc.magnitude2() > 0.1 {
      //   kinetics.velocity += norm_acc * dt;
      // }
    }
    rigid_body.acceleration = Vec3F::new(0.0, 0.0, 0.0);
  }
}