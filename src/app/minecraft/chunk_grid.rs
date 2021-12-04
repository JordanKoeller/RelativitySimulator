use std::collections::HashMap;

use specs::Entity;
use utils::{Vec2I, Vec3F};
use app::minecraft::CHUNK_DIMENSIONS;


#[derive(Default)]
pub struct ChunkGrid {
  data: HashMap<Vec2I, Entity>,
}

impl ChunkGrid {

  pub fn get_entity_from_coord(&self, coord: &Vec3F) -> Option<&Entity> {
    let idx = self.get_index_from_position(coord);
    self.data.get(&&idx)
  }

  pub fn add_chunk(&mut self, coord: Vec2I, chunk: Entity) {
    self.data.insert(coord, chunk);
  }

  pub fn has_chunk(&mut self, coord: &Vec2I) -> bool {
    self.data.contains_key(&self.get_index_pair(coord))
  }

  pub fn remove_chunk(&mut self, coord: &Vec2I) {
    self.data.remove(&self.get_index_pair(coord));
  }

  fn get_index_pair(&self, coord: &Vec2I) -> Vec2I {
    Vec2I::new(
      coord.x / CHUNK_DIMENSIONS.x as i32,
      coord.y / CHUNK_DIMENSIONS.z as i32
    )
  }

  fn get_index_from_position(&self, coord: &Vec3F) -> Vec2I {
    Vec2I::new(
      coord.x.floor() as i32 / CHUNK_DIMENSIONS.x as i32,
      coord.z.floor() as i32 / CHUNK_DIMENSIONS.z as i32,
    )
  }
}