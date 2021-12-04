use specs::prelude::*;
use cgmath::prelude::Zero;
use physics::{TransformComponent};
use shapes::{Block, Sprite};
use utils::{Vec3F, QuatF, Vec2F, Vec2I};
use ecs::{EntityManager, EntityCrudEvent};
use renderer::{Renderer, Drawable, Mesh};
use events::{StatefulEventChannel, EventChannel};

use app::minecraft::prefabs::{ChunkComponent, BlockType, ChunkBuilder, ChunkBuilderState};
use app::minecraft::{BlockGenerator, ChunkGrid};

#[derive(Default)]
pub struct ChunkManager {
  world_generator: BlockGenerator,
}

impl <'a> System<'a> for ChunkManager {
  type SystemData = (
    WriteStorage<'a, ChunkComponent>,
  );

  fn run(&mut self, _data: Self::SystemData) {

  }

  fn setup(&mut self, world: &mut World) {
    for x in 0..16 {
      for z in 0..16 {
        let seed_vec = Vec2I::new(x, z);
        let mut event_queue = world.write_resource::<StatefulEventChannel<EntityCrudEvent, ChunkBuilderState>>();
        event_queue.publish((
          EntityCrudEvent::Create,
          ChunkBuilderState::new(seed_vec, self.world_generator.clone())
        ));
      }
    }
  }
}
