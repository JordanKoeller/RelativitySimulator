use app::minecraft::ChunkComponent;
use shapes::{ BlockFace};
use utils::{Vec3I, Vec3F};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
  Empty,
  Grass,
  Rock,
  Water,
  Wood,
  Leaf,
}




pub struct BlockDescriptor<'a> {
  index: Vec3I,
  parent_chunk: &'a ChunkComponent,
}

impl<'a> BlockDescriptor<'a> {
  pub fn new(index: Vec3I, parent_chunk: &'a ChunkComponent) -> Self {
    Self { index, parent_chunk }
  }

  pub fn block_type(&self) -> &BlockType {
    self.parent_chunk.get(&self.index.x, &self.index.y, &self.index.z)
  }

  pub fn world_coordinate(&self) -> Vec3F {
    self.parent_chunk.world_coordinate(&self.index.x, &self.index.y, &self.index.z).cast::<f32>().expect("Could not get world coord")
  }

  pub fn face_exposed(&self, face: BlockFace) -> bool {
    let is_empty = |x, y, z| !self.parent_chunk.valid_index(x, y, z) || self.parent_chunk.get(x, y, z) == &BlockType::Empty;
    match face {
      BlockFace::Left => {
        self.block_type() != &BlockType::Empty && is_empty(&(self.index.x - 1), &self.index.y, &self.index.z)
      }
      BlockFace::Right => {
        self.block_type() != &BlockType::Empty && is_empty(&(self.index.x + 1), &self.index.y, &self.index.z)
      }
      BlockFace::Front => {
        self.block_type() != &BlockType::Empty && is_empty(&self.index.x, &self.index.y, &(self.index.z - 1))
      }
      BlockFace::Back => {
        self.block_type() != &BlockType::Empty && is_empty(&self.index.x, &self.index.y, &(self.index.z + 1))
      }
      BlockFace::Top => {
        self.block_type() != &BlockType::Empty && is_empty(&self.index.x, &(self.index.y + 1), &self.index.z)
      }
      BlockFace::Bottom => {
        self.block_type() != &BlockType::Empty && is_empty(&self.index.x, &(self.index.y - 1), &self.index.z)
      }
    }
  }
}
