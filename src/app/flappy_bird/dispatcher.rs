use specs::prelude::*;
use ecs::*;
use events::*;

use renderer::{Drawable, DrawableState};

use app::flappy_bird::{PlayerSystem, PlayerTailDelegate, CameraDebugger, WallSpawner, GameState};
use app::Skybox;
use utils::*;

use game_loop::SystemsRegistration;

pub fn register_systems<'a, 'b>(b: GameLoopBuilder<'a, 'b>) -> GameLoopBuilder<'a, 'b> {
  b
    .with(EntityManager::<PlayerTailDelegate>::default(), "tail_spawner", &[])
    .with_player_controller(PlayerSystem::default())
    // .with()
}

pub fn get_system_registration<'a, 'b>() -> Box<SystemsRegistration<'a, 'b>> {
  Box::new(|builder: DispatcherBuilder<'a, 'b>| builder
    .with(PlayerSystem::default(), "player_controller", &[])
    .with(EntityManager::<PlayerTailDelegate>::default(), "tail_spawner", &["player_controller"])
    .with(CameraDebugger, "camera_debugger", &[])
    .with(WallSpawner::default(), "wall_spawner", &[])
    .with(GameState::default(), "game_state", &["player_controller", "wall_spawner"])
  )
}


pub fn setup_world<'a, 'b>(world: &mut World) {
  let skybox = Skybox::new("resources/flappy_bird/skybox");
  world.register::<DrawableState>();
  world.create_entity()
    .with(skybox.state())
    .build();
}