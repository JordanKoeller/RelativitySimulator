#[macro_use]
extern crate engine;
extern crate cgmath;
extern crate specs;

mod prefabs;
mod systems;

use engine::ecs::{MotionSystem, Sys};
use engine::info;
use engine::prefab::{SkyboxBuilder, SkyboxPrefab, Sphere, SphereState};
use engine::utils::{Color, Vec3F};

use crate::prefabs::{Cube, CubeState};
use crate::systems::PlayerController;

fn main() {
    info!("App began!");
    let builder = engine::get_game_builder();
    info!("Got the builder");
    let mut builder = builder
        .with_system(Sys::<PlayerController>::default(), "player_controller", &[])
        .with_system(MotionSystem, "motion_controller", &["player_controller"])
        .with_prefab(&mut SkyboxBuilder::default(), SkyboxPrefab::new("resources/skybox"))
        .with_prefab(
            &mut Cube::default(),
            CubeState::new(
                1.0f64,
                Vec3F::new(4f64, 4f64, 4f64),
                "resources/debug/brickwall.jpg",
                "resources/debug/bricks_tangent.png",
            ),
        );
    let mut sphere_builder = Sphere::default();
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..5 {
                builder = builder.with_prefab(
                    &mut sphere_builder,
                    SphereState::new(
                        3f64,
                        Vec3F::new(0f64 + (10 * i) as f64, 4f64 + (10 * j) as f64, 16f64 + (10 * k) as f64),
                        Color::new(1f64, 0.3f64, 0.3f64),
                        "resources/earth/2k_earth_daymap.jpg",
                        "resources/earth/2k_earth_specular_map.png",
                        "resources/earth/2k_earth_normal_map.png",
                        64,
                    ),
                );
            }
        }
    }
    info!("Finished making the builder. About to send it off to engine::main");
    engine::main(builder);
}
