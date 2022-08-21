#[macro_use]
extern crate engine;
extern crate cgmath;
extern crate specs;

mod components;
mod prefabs;
mod systems;

use engine::ecs::{MotionSystem, Sys};
use engine::info;
use engine::prefab::{ModelBuilder, ModelLoader, SkyboxBuilder, SkyboxPrefab, Sphere, SphereState};
use engine::utils::{Color, Vec3F};

use crate::prefabs::{Cube, CubeState};
use crate::systems::{Party, PlayerController, SinSphere};

fn main() {
  info!("App began!");
  let builder = engine::get_game_builder();
  info!("Got the builder");
  let mut builder = builder
    .with_system(Sys::<PlayerController>::default(), "player_controller", &[])
    .with_system(MotionSystem, "motion_controller", &["player_controller"])
    .with_system(Sys::<SinSphere>::default(), "sin_sphere", &[])
    .with_system(Sys::<Party>::default(), "party", &["motion_controller"])
    .with_prefab(&mut SkyboxBuilder::default(), SkyboxPrefab::new("resources/skybox"))
    .with_prefab(
      &mut Cube::default(),
      CubeState::new(
        1.0f32,
        Vec3F::new(8f32, 9f32, 10f32),
        "resources/debug/brickwall.jpg",
        "resources/debug/bricks_tangent.png",
      ),
    )
    .with_prefab(
      &mut ModelBuilder::default(),
      ModelLoader::new("resources/debug/backpack/backpack.obj"),
    );

  // let mut sphere_builder = Sphere::default();
  // let (x, y, z) = (5, 5, 5);
  // for i in 0..x {
  //     for j in 0..y {
  //         for k in 0..z {
  //             builder = builder.with_prefab(
  //                 &mut sphere_builder,
  //                 SphereState::new(
  //                     3f32,
  //                     Vec3F::new(0f32 + (10 * i) as f32, 4f32 + (10 * j) as f32, 16f32 + (10 * k) as f32),
  //                     Color::new(1f32, 0.3f32, 0.3f32),
  //                     "resources/earth/2k_earth_daymap.jpg",
  //                     "resources/earth/2k_earth_specular_map.png",
  //                     "resources/earth/2k_earth_normal_map.png",
  //                     64,
  //                 ),
  //             );
  //         }
  //     }
  // }
  info!("Finished making the builder. About to send it off to engine::main");
  engine::main(builder);
}
