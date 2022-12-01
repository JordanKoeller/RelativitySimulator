#[macro_use]
extern crate engine;
extern crate cgmath;
extern crate specs;
extern crate env_logger;

mod components;
mod prefabs;
mod systems;

use engine::ecs::{MotionSystem, Sys};
use engine::info;
use engine::prefab::{ModelBuilder, ModelLoader, SkyboxBuilder, SkyboxPrefab,};
use engine::utils::{Vec3F};

use crate::prefabs::{Cube, CubeState};
use crate::systems::{Multiplayer, PlayerController, SinSphere};

fn main() {
  env_logger::init();
  info!("App began!");
  let builder = engine::get_game_builder();
  info!("Got the builder");
  let builder = builder
    .with_system(Sys::<PlayerController>::default(), "player_controller", &[])
    .with_system(MotionSystem, "motion_controller", &["player_controller"])
    .with_system(Sys::<SinSphere>::default(), "sin_sphere", &[])
    .with_system(Sys::<Multiplayer>::default(), "multiplayer", &["motion_controller"])
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

  info!("Finished making the builder. About to send it off to engine::main");
  engine::main(builder);
}
