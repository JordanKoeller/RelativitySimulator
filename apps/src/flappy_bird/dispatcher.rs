use ecs::*;
use events::*;
use specs::prelude::*;

use physics::TransformComponent;
use renderer::{Drawable, Mesh};

use app::flappy_bird::{CameraDebugger, GameStateSystem, PlayerSystem, PlayerTailDelegate, WallSpawner};
use app::Skybox;
use utils::*;

use game_loop::SystemsRegistration;

pub fn get_system_registration<'a, 'b>() -> Box<SystemsRegistration<'a, 'b>> {
    Box::new(|builder: DispatcherBuilder<'a, 'b>| {
        builder
            .with(PlayerSystem::default(), "player_controller", &[])
            .with(
                EntityManager::<PlayerTailDelegate>::default(),
                "tail_spawner",
                &["player_controller"],
            )
            .with(CameraDebugger, "camera_debugger", &[])
            .with(WallSpawner::default(), "wall_spawner", &[])
            .with(
                SystemManager::<GameStateSystem>::default(),
                "game_state",
                &["player_controller", "wall_spawner"],
            )
    })
}

pub fn setup_world<'a, 'b>(world: &mut World) {
    let skybox = Skybox::new("resources/skybox");
    world.register::<MeshComponent>();
    world.register::<Material>();
    world.register::<TransformComponent>();
    world
        .create_entity()
        .with(skybox.material())
        .with(skybox.mesh_component())
        .with(TransformComponent::identity())
        .build();
}
