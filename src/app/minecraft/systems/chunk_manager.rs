use cgmath::prelude::Zero;
use ecs::{EntitySpawner, PrefabBuilder, PrefabManager};
use events::{EventChannel, StatefulEventChannel};
use physics::TransformComponent;
use renderer::{Drawable, Mesh, Renderer};
use shapes::{Block, Sprite};
use specs::prelude::*;
use utils::{QuatF, Vec2F, Vec2I, Vec3F};

use app::minecraft::prefabs::{BlockType, ChunkBuilder, ChunkBuilderState, ChunkComponent};
use app::minecraft::{BlockGenerator, ChunkGrid};

#[derive(Default)]
pub struct ChunkManager {
    world_generator: BlockGenerator,
}

impl<'a> System<'a> for ChunkManager {
    type SystemData = (WriteStorage<'a, ChunkComponent>,);

    fn run(&mut self, _data: Self::SystemData) {}

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        {
            let mut builder = ChunkBuilder::default();
            builder.setup_delegate(world);
            world.insert(builder);
        }
        let mut builder = world.system_data::<PrefabManager<ChunkBuilder>>();
        // world.insert(builder);
        //   world.insert::<PrefabManager<ChunkBuilder>>();
        // world.insert(PrefabManager::<ChunkBuilder>::default());
        for x in 0..16 {
            for z in 0..16 {
                let seed_vec = Vec2I::new(x, z);
                let state = ChunkBuilderState::new(seed_vec, self.world_generator.clone());
                builder.create(state);
                // event_queue.publish((
                //   EntityCrudEvent::Create,
                //   ChunkBuilderState::new(seed_vec, self.world_generator.clone())
                // ));
            }
        }
        println!("Done setting up chunk manager!");
    }
}
