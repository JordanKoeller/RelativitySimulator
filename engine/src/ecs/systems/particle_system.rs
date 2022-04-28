use specs::prelude::*;
use specs::world::LazyBuilder;
use specs::{Component, VecStorage};

use crate::ecs::entity::MyBuilder;

use crate::debug::*;
use crate::graphics::{MaterialComponent, MeshComponent};
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
        Self { lifetime }
    }
}

pub struct ParticleUpdater;

impl<'a> System<'a> for ParticleUpdater {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Particle>,
        ReadStorage<'a, MeshComponent>,
        Read<'a, Timestep>,
        Read<'a, RenderQueue>,
    );

    fn run(&mut self, (entities, mut particle_storage, meshes, dt, render_queue): Self::SystemData) {
        for (entity, mesh, particle) in (&entities, &meshes, &mut particle_storage).join() {
            if let Some(remaining_time) = particle.lifetime.checked_sub(dt.dt()) {
                particle.lifetime = remaining_time;
            } else {
                render_queue.push(DrawCall {
                    mesh_component: mesh.clone(),
                    entity: entity,
                    cmd: RenderCommand::Free,
                });
                entities.delete(entity).expect("Could not delete entity");
            }
        }
    }
}
