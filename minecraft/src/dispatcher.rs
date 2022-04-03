use specs::prelude::*;

use engine::ecs::*;
use engine::events::*;
use engine::physics::TransformComponent;
use engine::renderer::{Drawable, Mesh};
use engine::utils::*;
use engine::game_loop::SystemsRegistration;

use super::systems::{ChunkManager, PlayerController};
use crate::skybox::Skybox;

pub fn get_system_registration<'a, 'b>() -> Box<SystemsRegistration<'a, 'b>> {
    Box::new(|builder: DispatcherBuilder<'a, 'b>| {
        builder
            .with(SystemManager::<PlayerController>::default(), "player_controller", &[])
            .with(ChunkManager::default(), "chunk_manager", &[])
    })
}

pub fn setup_world<'a, 'b>(world: &mut World) {
    let skybox = Skybox::new("resources/skybox");
    world.register::<MeshComponent>();
    world.register::<Material>();
    world.register::<TransformComponent>();
    world
        .create_entity()
        .with(skybox.material())
        .with(skybox.mesh_component())
        .with(TransformComponent::identity())
        .build();
}
