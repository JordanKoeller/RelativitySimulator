#[macro_use]
extern crate engine;
extern crate cgmath;
extern crate specs;
extern crate env_logger;

mod city;
mod systems;

use engine::get_game_builder;
use engine::ecs::Sys;
use systems::CityManager;

fn main() {
  env_logger::init();
  info!("Started app");
  let game_builder = get_game_builder();
  let game_builder = game_builder.with_system(Sys::<CityManager>::default(), "city-manager", &[]);
  info!("Setup finshed. Starting game.");
  engine::main(game_builder);
}