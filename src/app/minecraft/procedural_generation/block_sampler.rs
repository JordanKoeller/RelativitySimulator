use utils::Vec3I;

use app::minecraft::BlockType;

pub trait BlockSampler {
  fn new(seed_value: u32) -> Self;

  fn sample_block(&self, position: Vec3I) -> BlockType;
}
