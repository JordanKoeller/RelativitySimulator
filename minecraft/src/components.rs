use specs::{Component, VecStorage};


pub struct ChunkComponent {
}
impl Component for ChunkComponent {
    type Storage = VecStorage<ChunkComponent>;
}
