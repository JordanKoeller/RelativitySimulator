use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use cgmath::prelude::*;
use ecs::components::{Kinetics, Position};
use utils::*;

use renderer::LIGHT_SPEED;
const MAX_ACCELERATION: f32 = 6f32;

pub struct Motion;

impl<'a> System<'a> for Motion {
  type SystemData = (
    WriteStorage<'a, Position>,
    WriteStorage<'a, Kinetics>,
    Read<'a, Timestep>,
  );

  fn run(&mut self, (mut pos_storage, mut kin_storage, dt): Self::SystemData) {
    for (position, kinetics) in (&mut pos_storage, &mut kin_storage).join() {
      let norm_acc = if kinetics.acceleration.magnitude() != 0.0 {
        kinetics.acceleration.normalize_to(MAX_ACCELERATION)
      } else {
        kinetics.acceleration
      };
      if kinetics.velocity.magnitude() > 0.1 {
        kinetics.velocity += norm_acc * dt.0;
      } else if norm_acc.magnitude() > 0.1 {
        kinetics.velocity += norm_acc * dt.0;
      }
      kinetics.acceleration = Vec3F::new(0.0, 0.0, 0.0);
      position.0 += kinetics.velocity * dt.0;
    }
  }
}