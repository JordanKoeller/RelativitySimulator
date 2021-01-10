use specs::{Join, Read, System, WriteStorage};

use cgmath::prelude::*;
use ecs::components::{Kinetics, Position};
use utils::*;

use renderer::LIGHT_SPEED;
const MAX_ACCELERATION: f32 = 6f32;

const DRAG: f32 = MAX_ACCELERATION / LIGHT_SPEED / LIGHT_SPEED;

pub struct Motion;

impl<'a> System<'a> for Motion {
  type SystemData = (
    WriteStorage<'a, Position>,
    WriteStorage<'a, Kinetics>,
    Read<'a, Timestep>,
  );

  fn run(&mut self, (mut pos_storage, mut kin_storage, dt): Self::SystemData) {
    for (position, kinetics) in (&mut pos_storage, &mut kin_storage).join() {
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
        kinetics.velocity += norm_acc * dt.0;
      } else if norm_acc.magnitude2() > 0.1 {
        kinetics.velocity += norm_acc * dt.0;
      }
      kinetics.acceleration = Vec3F::new(0.0, 0.0, 0.0);
      position.0 += kinetics.velocity * dt.0;
    }
  }
}