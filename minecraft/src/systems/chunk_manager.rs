use engine::ecs::{MonoBehavior, PrefabBuilder, WorldProxy};

use crate::prefabs::{Chunk, ChunkBuilder};

#[derive(Default)]
pub struct ChunkManager;

impl<'a> MonoBehavior<'a> for ChunkManager {
    type SystemData = ();

    fn setup(&mut self, world: WorldProxy) {
        let mut builder = ChunkBuilder::default();
        builder.build(&world.utilities(), Chunk);
    }
}
