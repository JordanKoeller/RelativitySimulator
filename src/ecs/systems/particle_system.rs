use specs::prelude::*;
use specs::{Component, VecStorage};
use specs::world::LazyBuilder;

use ecs::entity::MyBuilder;
use ecs::DrawableId;

use utils::Timestep;
use renderer::{RenderQueue, DrawCall, RenderCommand};

#[derive(Component, Debug, Default, Clone)]
#[storage(VecStorage)]
pub struct Particle {
  pub lifetime: f32,
  // pub decay: f32, // TODO: Add some shader support for a global alpha parameter.
}






pub struct ParticleUpdater;

impl <'a> System<'a> for ParticleUpdater {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, DrawableId>,
    WriteStorage<'a, Particle>,
    Read<'a, Timestep>,
    Read<'a, RenderQueue>,
  );

  fn run(&mut self, (entities, drawable_id, mut particle_storage, dt, render_queue): Self::SystemData) {
    for (entity, drawable_id, particle) in (&entities, &drawable_id, &mut particle_storage).join() {
      let remaining_time = particle.lifetime - dt.click;
      if remaining_time <= 0f32 {
        render_queue.push(DrawCall {
          drawable: drawable_id.clone(),
          entity: entity,
          cmd: RenderCommand::Free,
        });
        entities.delete(entity).expect("Could not delete entity");
      } else {
        particle.lifetime = remaining_time;
      }
    }
  }
}