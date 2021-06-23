use specs::prelude::*;
use specs::{Component, VecStorage};
use specs::world::LazyBuilder;

use ecs::entity::MyBuilder;

use utils::Timestep;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Particle {
  pub lifetime: f32,
  pub decay: f32, // TODO: Add some shader support for a global alpha parameter.
}






pub struct ParticleUpdater;

impl <'a> System<'a> for ParticleUpdater {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, Particle>,
    Read<'a, Timestep>,
  );

  fn run(&mut self, (entities, mut particle_storage, dt): Self::SystemData) {
    for (entity, particle) in (&entities, &mut particle_storage).join() {
      let remaining_time = particle.lifetime - dt.0;
      if remaining_time <= 0f32 {
        entities.delete(entity).expect("Could not delete entity");
      } else {
        particle.lifetime = remaining_time;
      }
    }
  }
}