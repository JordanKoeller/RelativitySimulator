use specs::prelude::*;
use cgmath::prelude::*;

use ecs::components::*;

use utils::*;

pub struct WallSpawner {
  last_spawn_time: f32,
  min_spawn_duration: f32,
  window_spawn_time: f32,
  gap_width: f32

}

impl <'a> System<'a> for WallSpawner {
  type SystemData = (

  );
}