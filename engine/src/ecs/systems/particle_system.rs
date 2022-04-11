use specs::prelude::*;
use specs::world::LazyBuilder;
use specs::{Component, VecStorage};

use crate::ecs::entity::MyBuilder;
use crate::ecs::DrawableId;

use crate::debug::*;
use crate::renderer::{DrawCall, RenderCommand, RenderQueue};
use crate::utils::Timestep;

#[derive(Component, Debug, Default, Clone)]
#[storage(VecStorage)]
pub struct Particle {
    pub lifetime: std::time::Duration,
    // pub decay: f32, // TODO: Add some shader support for a global alpha parameter.
}

impl Particle {
    pub fn new(lifetime: std::time::Duration) -> Self {
        Self {
            lifetime,
        }
    }
}

pub struct ParticleUpdater;

impl<'a> System<'a> for ParticleUpdater {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, DrawableId>,
        WriteStorage<'a, Particle>,
        Read<'a, Timestep>,
        Read<'a, RenderQueue>,
    );

    fn run(&mut self, (entities, drawable_id, mut particle_storage, dt, render_queue): Self::SystemData) {
        for (entity, drawable_id, particle) in (&entities, &drawable_id, &mut particle_storage).join() {
            if let Some(remaining_time) = particle.lifetime.checked_sub(dt.dt()) {
                particle.lifetime = remaining_time;
            } else {
                render_queue.push(DrawCall {
                    drawable: drawable_id.clone(),
                    entity: entity,
                    cmd: RenderCommand::Free,
                });
                entities.delete(entity).expect("Could not delete entity");
            }
        }
    }
}
