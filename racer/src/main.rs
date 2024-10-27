#[macro_use]
extern crate engine;
extern crate cgmath;
extern crate env_logger;
extern crate specs;

mod city;
mod systems;
mod relativity;

use engine::get_game_builder;

use engine::ecs::{MotionSystem};
use engine::info;
use engine::prefab::{SkyboxBuilder, SkyboxPrefab};

use crate::relativity::Relativity;
use crate::systems::{CityManager, PlayerController};
fn main() {
  env_logger::init();
  info!("Started app");
  let game_builder = get_game_builder();
  let game_builder = game_builder
    .with_behavior_type::<CityManager>("city-manager", &[])
    .with_behavior_type::<PlayerController>("player_controller", &[])
    .with_system(MotionSystem, "motion_controller", &["player_controller"])
    .with_behavior_type::<Relativity>("relativity", &["player_controller", "motion_controller"])
    .with_prefab(&mut SkyboxBuilder::default(), SkyboxPrefab::new("resources/skybox"));
  info!("Setup finshed. Starting game.");
  engine::main(game_builder);
}
