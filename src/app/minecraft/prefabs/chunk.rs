use noise::{NoiseFn, Perlin, Seedable};
use specs::prelude::*;
use specs::{Component, HashMapStorage, VecStorage};
use utils::{Vec2F, Vec2I, Vec3I, Vec3F};

use super::super::BlockSampler;
use super::{BlockFace, BlockType, BlockDescriptor};

pub const CHUNK_DIMENSIONS: Vec3I = Vec3I::new(32, 16, 32);

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct ChunkComponent {
  chunk_index: Vec3I,
  pub world_origin: Vec3I,
  blocks: [BlockType; CHUNK_DIMENSIONS.x * CHUNK_DIMENSIONS.y * CHUNK_DIMENSIONS.z],
}

impl Default for ChunkComponent {
  fn default() -> Self {
    Self {
      chunk_index: Vec3I::new(0usize, 0usize, 0usize),
      blocks: [BlockType::Empty; CHUNK_DIMENSIONS.x * CHUNK_DIMENSIONS.y * CHUNK_DIMENSIONS.z],
      world_origin: Vec3I::new(0usize, 0usize, 0usize),
    }
  }
}

impl ChunkComponent {
  pub fn new<RNG: BlockSampler>(chunk_index: Vec2I, rng: &RNG) -> Self {
    let mut ret = Self {
      chunk_index: Vec3I::new(chunk_index.x as usize, 0usize, chunk_index.y as usize),
      world_origin: Vec3I::new(
        chunk_index.x as usize * CHUNK_DIMENSIONS.x,
        0usize,
        chunk_index.y as usize * CHUNK_DIMENSIONS.z,
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

  pub fn surface_area(&self) -> usize {
    let front_area = CHUNK_DIMENSIONS.x * CHUNK_DIMENSIONS.y;
    let side_area = CHUNK_DIMENSIONS.z * CHUNK_DIMENSIONS.y;
    let top_area = CHUNK_DIMENSIONS.x * CHUNK_DIMENSIONS.z;
    2 * top_area + 2 * front_area + 2 * side_area
  }

  pub fn valid_index(&self, x: &usize, y: &usize, z: &usize) -> bool {
    x < &CHUNK_DIMENSIONS.x && y < &CHUNK_DIMENSIONS.y && z < &CHUNK_DIMENSIONS.z
  }

  fn flat_index(&self, index: &Vec3I) -> usize {
    index.z + index.y * CHUNK_DIMENSIONS.z + index.x * CHUNK_DIMENSIONS.z * CHUNK_DIMENSIONS.y
  }

  fn flat_index_unwrap(&self, x: &usize, y: &usize, z: &usize) -> usize {
    z + y * CHUNK_DIMENSIONS.z + x * CHUNK_DIMENSIONS.z * CHUNK_DIMENSIONS.y
  }

  pub fn get(&self, x: &usize, y: &usize, z: &usize) -> &BlockType {
    &self.blocks[self.flat_index_unwrap(x, y, z)]
  }

  pub fn get_index(&self, index: &Vec3I) -> &BlockType {
    &self.blocks[self.flat_index(index)]
  }

  pub fn world_coordinate(&self, x: &usize, y: &usize, z: &usize) -> Vec3I {
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
    (position.x.floor() as usize) >= self.world_origin.x && (position.x.floor() as usize) < opposite_corner.x &&
    (position.y.floor() as usize) >= self.world_origin.y && (position.y.floor() as usize) < opposite_corner.y &&
    (position.z.floor() as usize) >= self.world_origin.z && (position.z.floor() as usize) < opposite_corner.z
  }

  fn is_empty_block(&self, position: &Vec3F) -> bool {
    let world_index = Vec3I::new(position.x.floor() as usize, position.y.floor() as usize, position.z.floor() as usize);
    let index = world_index - self.world_origin;
    self.get_index(&index) == &BlockType::Empty
  }

  pub fn collides(&self, position: &Vec3F) -> bool {
    let is_colliding = self.contains(position) && !self.is_empty_block(position);
    is_colliding
  }
}

