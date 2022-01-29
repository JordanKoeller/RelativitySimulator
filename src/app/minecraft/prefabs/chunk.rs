use noise::{NoiseFn, Perlin, Seedable};
use specs::prelude::*;
use specs::{Component, HashMapStorage, VecStorage};
use utils::{Vec2F, Vec2I, Vec3I, Vec3F, line_intersects_block};

use super::super::BlockSampler;
use super::{BlockType, BlockDescriptor};
use shapes::{BlockFace, get_index_shift};

pub const CHUNK_DIMENSIONS: Vec3I = Vec3I::new(32, 16, 32);

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct ChunkComponent {
  chunk_index: Vec3I,
  pub world_origin: Vec3I,
  blocks: [BlockType; (CHUNK_DIMENSIONS.x * CHUNK_DIMENSIONS.y * CHUNK_DIMENSIONS.z) as usize],
}

impl Default for ChunkComponent {
  fn default() -> Self {
    Self {
      chunk_index: Vec3I::new(0, 0, 0),
      blocks: [BlockType::Empty; (CHUNK_DIMENSIONS.x * CHUNK_DIMENSIONS.y * CHUNK_DIMENSIONS.z) as usize],
      world_origin: Vec3I::new(0, 0, 0),
    }
  }
}

impl ChunkComponent {
  pub fn new<RNG: BlockSampler>(chunk_index: Vec2I, rng: &RNG) -> Self {
    let mut ret = Self {
      chunk_index: Vec3I::new(chunk_index.x, 0, chunk_index.y ),
      world_origin: Vec3I::new(
        chunk_index.x * CHUNK_DIMENSIONS.x,
        0,
        chunk_index.y * CHUNK_DIMENSIONS.z,
      ),
      ..Self::default()
    };
    for x in 0..CHUNK_DIMENSIONS.x {
      for y in 0..CHUNK_DIMENSIONS.y {
        for z in 0..CHUNK_DIMENSIONS.z {
          let idx = Vec3I::new(x, y, z);
          ret.update_block(idx, rng.sample_block(idx + ret.world_origin));
        }
      }
    }
    ret
  }
}

impl ChunkComponent {
  /**
   * Updates the block type at a specified index.
   * Returns true if the update modified the block (meaning the previous block was in fact different)
   */
  pub fn update_block(&mut self, index: Vec3I, block_type: BlockType) -> bool {
    let block_index = self.flat_index(&index);
    let ret = self.blocks[block_index] == block_type;
    self.blocks[block_index] = block_type;
    !ret
  }

  pub fn surface_area(&self) -> i32 {
    let front_area = CHUNK_DIMENSIONS.x * CHUNK_DIMENSIONS.y;
    let side_area = CHUNK_DIMENSIONS.z * CHUNK_DIMENSIONS.y;
    let top_area = CHUNK_DIMENSIONS.x * CHUNK_DIMENSIONS.z;
    2 * top_area + 2 * front_area + 2 * side_area
  }

  pub fn valid_index(&self, x: &i32, y: &i32, z: &i32) -> bool {
    x < &CHUNK_DIMENSIONS.x && y < &CHUNK_DIMENSIONS.y && z < &CHUNK_DIMENSIONS.z
  }

  fn flat_index(&self, index: &Vec3I) -> usize {
    (index.z + index.y * CHUNK_DIMENSIONS.z + index.x * CHUNK_DIMENSIONS.z * CHUNK_DIMENSIONS.y) as usize
  }

  fn flat_index_unwrap(&self, x: &i32, y: &i32, z: &i32) -> usize {
    (z + y * CHUNK_DIMENSIONS.z + x * CHUNK_DIMENSIONS.z * CHUNK_DIMENSIONS.y) as usize
  }

  pub fn get(&self, x: &i32, y: &i32, z: &i32) -> &BlockType {
    &self.blocks[self.flat_index_unwrap(x, y, z)]
  }

  pub fn get_index(&self, index: &Vec3I) -> &BlockType {
    &self.blocks[self.flat_index(index)]
  }

  pub fn world_coordinate(&self, x: &i32, y: &i32, z: &i32) -> Vec3I {
    Vec3I::new(
      self.world_origin.x + x,
      *y,
      self.world_origin.z + z,
    )
  }

  pub fn foreach_face<F: FnMut(&BlockDescriptor, BlockFace) -> ()>(&self, f: &mut F) {
    let faces = BlockFace::faces_array();
    for x in 0..CHUNK_DIMENSIONS.x {
      for y in 0..CHUNK_DIMENSIONS.y {
        for z in 0..CHUNK_DIMENSIONS.z {
          let descriptor = BlockDescriptor::new(Vec3I::new(x,y,z), &self);
          for face_index in 0..faces.len() {
            if descriptor.face_exposed(faces[face_index]) {
              f(&descriptor, faces[face_index]);
            }
          }
        }
      }
    }
  }

  fn contains(&self, position: &Vec3F) -> bool {
    let opposite_corner = self.world_origin + CHUNK_DIMENSIONS;
    (position.x.floor() as i32) >= self.world_origin.x && (position.x.floor() as i32) < opposite_corner.x &&
    (position.y.floor() as i32) >= self.world_origin.y && (position.y.floor() as i32) < opposite_corner.y &&
    (position.z.floor() as i32) >= self.world_origin.z && (position.z.floor() as i32) < opposite_corner.z
  }

  fn is_empty_block(&self, position: &Vec3F) -> bool {
      let index = self.get_index_from_floats(position);
      self.get_index(&index) == &BlockType::Empty
    }
    
    fn get_index_from_floats(&self, position: &Vec3F) -> Vec3I {
      let world_index = Vec3I::new(position.x.floor() as i32, position.y.floor() as i32, position.z.floor() as i32);
      world_index - self.world_origin

  }



  pub fn collides(&self, position: &Vec3F) -> bool {
    self.contains(position) && !self.is_empty_block(position)
  }

  pub fn get_first_on_line(&self, start: &Vec3F, end: &Vec3F) -> Option<(Vec3I, BlockFace)> {
      let mut query_pt = start.clone();
      let mut block_index = self.get_index_from_floats(&query_pt);
      let mut flag = true;
      while flag {
          let (low, high) = self.block_dimensions(&block_index);
          if let Some((intersection, face)) = line_intersects_block(&query_pt, end, &low, &high) {
            if self.get_index(&block_index) == &BlockType::Empty {
                block_index = block_index + get_index_shift(&face);
                query_pt = intersection;
                flag = self.valid_index(&block_index.x, &block_index.y, &block_index.z);
            } else {
                return Some((block_index, face))
            }
          } else {
              flag = false;
          }
      }
      None
  }


    // Returns the lows and highs
    pub fn block_dimensions(&self, index: &Vec3I) -> (Vec3F, Vec3F) {
        let lows = Vec3F::new(
            (self.world_origin.x + index.x) as f32,
            (self.world_origin.y + index.y) as f32,
            (self.world_origin.z + index.z) as f32,
        );
        (
            lows,
            lows + Vec3F::new(1f32, 1f32, 1f32)
        )
    }

  // Returns the lows and highs
  pub fn chunk_dimensions(&self) -> (Vec3F, Vec3F) {
      (
          Vec3F::new(
              self.world_origin.x as f32,
              self.world_origin.y as f32,
              self.world_origin.z as f32,
          ),
          Vec3F::new(
              (self.world_origin.x + CHUNK_DIMENSIONS.x) as f32,
              (self.world_origin.y + CHUNK_DIMENSIONS.y) as f32,
              (self.world_origin.z + CHUNK_DIMENSIONS.z) as f32,
          )
      )
  }
}