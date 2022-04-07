#[macro_use]
extern crate engine;
extern crate cgmath;
extern crate specs;

mod components;
mod prefabs;
mod skybox;
mod systems;

use specs::prelude::*;

use crate::skybox::Skybox;
use crate::systems::{ChunkManager, PlayerController};
use engine::ecs::SystemManager;
use engine::physics::TransformComponent;
use engine::renderer::Drawable;

fn main() {
    let builder = engine::engine_builder()
        .with_system(SystemManager::<PlayerController>::default(), "player_controller", &[])
        .with_system(ChunkManager::default(), "chunk_manager", &[])
        .with_entity(|ett| {
            let skybox = Skybox::new("resources/skybox");
            ett.with(skybox.material())
                .with(skybox.mesh_component())
                .with(TransformComponent::identity())
                .build()
        });
    engine::main(builder);
}
