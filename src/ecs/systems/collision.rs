use specs::prelude::*;

use app::AxisAlignedCubeCollision;
use ecs::components::*;
use ecs::{Collision, CanCollide};
use utils::Timestep;
pub struct CollisionSystem;

impl Default for CollisionSystem {
  fn default() -> Self {
    Self
  }
}

impl<'a> System<'a> for CollisionSystem {
  type SystemData = (
    ReadStorage<'a, Player>,
    WriteStorage<'a, CanCollide>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Kinetics>,
    ReadStorage<'a, AxisAlignedCubeCollision>,
    Read<'a, Timestep>,
  );

  fn run(&mut self, (player_storage, mut collide_storage, position_storage, kinetic_storage, cube_storage, dt): Self::SystemData) {
    let radius = 1f32;
    for (_, collider, position, kinetics) in (&player_storage, &mut collide_storage, &position_storage, &kinetic_storage).join() {
      for cube in cube_storage.join() {
        if let Some(collision_summary) = cube.sphere_collision((&position.0, &radius), &kinetics.velocity) {
          if collision_summary.time < dt.0 {
            // collider.summary = Some(collision_summary);
          }
        }
      }
    }
  }

  fn setup(&mut self, world: &mut World) {
    world.register::<AxisAlignedCubeCollision>();
  }
}
