#[macro_use]
extern crate engine;
extern crate cgmath;
extern crate specs;

mod components;
mod prefabs;
mod systems;

use engine::info;


use engine::ecs::Sys;

use engine::prefab::{SkyboxBuilder, SkyboxPrefab};

use crate::systems::{ChunkManager, PlayerController};

fn main() {
    info!("App began!");
    let builder = engine::get_game_builder();
    info!("Got the builder");
    let built = builder
        .with_system(Sys::<PlayerController>::default(), "player_controller", &[])
        .with_system(Sys::<ChunkManager>::default(), "chunk_manager", &[])
        .with_prefab(SkyboxBuilder::default(), SkyboxPrefab::new("resources/skybox"));
    info!("Finished making the builder. About to send it off to engine::main");
    engine::main(built);
}
