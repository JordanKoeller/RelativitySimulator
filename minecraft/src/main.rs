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

use crate::systems::{PlayerController};

fn main() {
    info!("App began!");
    let builder = engine::get_game_builder();
    info!("Got the builder");
    let built = builder
        .with_system(Sys::<PlayerController>::default(), "player_controller", &[])
        .with_prefab(SkyboxBuilder::default(), SkyboxPrefab::new("resources/skybox"))
        .with_prefab(
            Sphere::default(),
            SphereState::new(
                3f32,
                Vec3F::new(0f32, 4f32, 16f32),
                Color::new(1f32, 0.3f32, 0.3f32),
                "resources/earth/2k_earth_daymap.jpg",
                "resources/earth/2k_earth_daymap.jpg",
                "resources/earth/2k_earth_normal_map.png",
                16,
            ),
        );
    info!("Finished making the builder. About to send it off to engine::main");
    engine::main(built);
}
