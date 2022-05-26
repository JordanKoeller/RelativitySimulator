#[macro_use]
extern crate engine;
extern crate cgmath;
extern crate specs;

mod components;
mod prefabs;
mod systems;

use engine::info;

use engine::ecs::Sys;

use engine::utils::{Color, Vec3F};

use engine::prefab::{SkyboxBuilder, SkyboxPrefab, Sphere, SphereState};

use crate::systems::{ChunkManager, PlayerController};

fn main() {
    info!("App began!");
    let builder = engine::get_game_builder();
    info!("Got the builder");
    let built = builder
        .with_system(Sys::<PlayerController>::default(), "player_controller", &[])
        .with_system(Sys::<ChunkManager>::default(), "chunk_manager", &[])
        .with_prefab(SkyboxBuilder::default(), SkyboxPrefab::new("resources/skybox"))
        .with_prefab(
            Sphere::default(),
            SphereState::new(
                3f32,
                Vec3F::new(16f32, 10f32, 16f32),
                Color::new(1f32, 0.3f32, 0.3f32),
                16,
            ),
        );
    info!("Finished making the builder. About to send it off to engine::main");
    engine::main(built);
}
