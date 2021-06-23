use specs::prelude::*;
use ecs::*;
use events::*;

use app::flappy_bird::{PlayerSystem, PlayerTailDelegate, CameraDebugger};

use utils::*;

use game_loop::SystemsRegistration;

pub fn register_systems<'a, 'b>(b: GameLoopBuilder<'a, 'b>) -> GameLoopBuilder<'a, 'b> {
  b
    .with(EntityManager::<PlayerTailDelegate>::default(), "tail_spawner", &[])
    .with_player_controller(PlayerSystem)
    // .with()
}

pub fn get_system_registration<'a, 'b>() -> Box<SystemsRegistration<'a, 'b>> {
  Box::new(|builder: DispatcherBuilder<'a, 'b>| builder
    // .with(EntityManager::<PlayerTailDelegate>::default(), "tail_spawner", &[])
    .with(PlayerSystem, "player_controller", &[])
    .with(CameraDebugger, "camera_debugger", &[])
  )
}


pub fn setup_world<'a, 'b>(world: &mut World) {
}