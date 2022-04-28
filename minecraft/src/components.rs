use specs::{Component, VecStorage};

use crate::prefabs::MCBlock;

pub struct ChunkComponent {
    blocks: [MCBlock; 65536],
}
impl Component for ChunkComponent {
    type Storage = VecStorage<ChunkComponent>;
}
