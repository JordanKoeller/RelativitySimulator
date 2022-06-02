#[macro_use]
extern crate engine;
extern crate cgmath;
extern crate specs;

mod prefabs;
mod systems;

use engine::ecs::{MotionSystem, Sys};
use engine::info;
use engine::prefab::{SkyboxBuilder, SkyboxPrefab};
use engine::utils::Vec3F;

use crate::prefabs::{Cube, CubeState};
use crate::systems::PlayerController;

fn main() {
    info!("App began!");
    let builder = engine::get_game_builder();
    info!("Got the builder");
    let built = builder
        .with_system(Sys::<PlayerController>::default(), "player_controller", &[])
        .with_system(MotionSystem, "motion_controller", &["player_controller"])
        .with_prefab(SkyboxBuilder::default(), SkyboxPrefab::new("resources/skybox"))
        .with_prefab(
            Cube::default(),
            CubeState::new(
                1.0f32,
                Vec3F::new(4f32, 4f32, 4f32),
                "resources/debug/brickwall.jpg",
                "resources/debug/bricks_tangent.png",
            ),
        );
    // .with_prefab(
    //     Sphere::default(),
    //     SphereState::new(
    //         3f32,
    //         Vec3F::new(0f32, 4f32, 16f32),
    //         Color::new(1f32, 0.3f32, 0.3f32),
    //         "resources/earth/2k_earth_daymap.jpg",
    //         "resources/earth/2k_earth_daymap.jpg",
    //         "resources/earth/2k_earth_normal_map.png",
    //         // "resources/earth/2k_earth_specular_map.png",
    //         64,
    //     ),
    // );
    info!("Finished making the builder. About to send it off to engine::main");
    engine::main(built);
}
