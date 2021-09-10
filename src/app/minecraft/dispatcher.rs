use ecs::*;
use events::*;
use specs::prelude::*;

use physics::TransformComponent;
use renderer::{Drawable, Mesh};
use utils::*;
use app::Skybox;

use game_loop::SystemsRegistration;

use super::systems::{PlayerController, ChunkManager};

pub fn get_system_registration<'a, 'b>() -> Box<SystemsRegistration<'a, 'b>> {
  Box::new(|builder: DispatcherBuilder<'a, 'b>| {
    builder
      .with(SystemManager::<PlayerController>::default(), "player_controller", &[])
      .with(ChunkManager::default(), "chunk_manager", &[])
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
