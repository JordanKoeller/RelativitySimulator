#[macro_use]
extern crate engine;
extern crate cgmath;
extern crate specs;

mod components;
mod prefabs;
mod skybox;
mod systems;


use engine::{SkyboxBuilder, SkyboxPrefab};
use crate::systems::{ChunkManager, PlayerController};
use engine::ecs::SystemManager;

fn main() {
    let builder = engine::get_game_builder()
        .with_system(SystemManager::<PlayerController>::default(), "player_controller", &[])
        .with_system(ChunkManager::default(), "chunk_manager", &[])
        .with_prefab(SkyboxBuilder, SkyboxPrefab::new("resources/skybox"));
    engine::main(builder);
}
