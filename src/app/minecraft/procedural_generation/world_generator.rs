use noise::{NoiseFn, Perlin, Seedable};

use super::super::{BlockType, CHUNK_DIMENSIONS};
use super::BlockSampler;
use utils::{lerp, Vec3I};

#[derive(Debug, Clone)]
pub struct BlockGenerator {
  seed: u32,
  rng: Perlin,
}

impl Default for BlockGenerator {
  fn default() -> Self {
    Self {
      rng: Perlin::default(),
      seed: 0u32,
    }
  }
}

impl BlockSampler for BlockGenerator {
  fn new(seed_value: u32) -> Self {
    let rng = Perlin::default();
    rng.set_seed(seed_value);
    Self { seed: seed_value, rng }
  }

  fn sample_block(&self, position: Vec3I) -> BlockType {
    // let scaled_block_coord = [
    //   (position.x as f64) * 0.5f64,
    //   (position.y as f64) * 0.5f64,
    //   (position.z as f64) * 0.5f64,
    // ];
    let scaled_planar_coord = [(position.x as f64) * 0.02f64, (position.z as f64) * 0.02f64];
    let height_factor = self.sample_generator_2(scaled_planar_coord);
    let block_height = (position.y as f32) / (CHUNK_DIMENSIONS.y as f32);
    if block_height > height_factor {
      BlockType::Empty
    } else {
      BlockType::Grass
    }
  }
}

impl BlockGenerator {
  fn sample_generator_3(&self, input: [f64; 3]) -> f32 {
    let ret = self.rng.get(input) as f32;
    lerp(-1f32, 1f32, 0f32, 1f32, ret)
  }

  fn sample_generator_2(&self, input: [f64; 2]) -> f32 {
    let ret = self.rng.get(input) as f32;
    lerp(-1f32, 1f32, 0f32, 1f32, ret)
  }
}
