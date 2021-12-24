use cgmath::prelude::*;
use std::collections::HashMap;

use app::minecraft::{ChunkComponent, CHUNK_DIMENSIONS};
use app::AxisAlignedCubeCollision;
use specs::{Entity, ReadStorage};
use utils::{Vec2I, Vec3F, Vec3I};

use physics::{Collision, CollisionSummary};

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

  pub fn get_nearby_collidables<'a>(&self, pos: &Vec3F, chunk_storage: &ReadStorage<'a, ChunkComponent>) -> Vec<AxisAlignedCubeCollision> {
    let mut ret: Vec<AxisAlignedCubeCollision> = Vec::new();
    let floor_vec = Vec3F::new(pos.x.floor() + 0.5, pos.y.floor() + 0.5, pos.z.floor() + 0.5);
    let shifts: [f32; 3] = [-1f32, 0f32, 1f32];
    for xx in shifts.iter() {
      for yy in shifts.iter() {
        for zz in shifts.iter() {
          if xx != &0f32 || yy != &0f32 || zz != &0f32 {
            if !self.is_occupied(&pos, chunk_storage) {
              ret.push(AxisAlignedCubeCollision::from_vecs(
                floor_vec + Vec3F::new(*xx, *yy, *zz),
                Vec3F::new(1f32, 1f32, 1f32)
              ));
            }
          }
        }
      }
    }
    ret
  }

  // pub fn get_next_position<'a>(
  //   &self,
  //   position: &Vec3F,
  //   velocity: &Vec3F,
  //   dt: &f32,
  //   chunk_storage: &ReadStorage<'a, ChunkComponent>,
  // ) -> Vec3F {
  //   if velocity.magnitude2() < 0.001 {
  //     return position.clone();
  //   }
  //   let mut flag = true;
  //   let mut remaining_time = dt.clone();
  //   let mut pos = position.clone();
  //   let mut direction = velocity.clone();
  //   while flag {
  //     let next_pos = pos + remaining_time * direction;
  //     if !self.is_occupied(&next_pos, chunk_storage) {
  //       pos = next_pos;
  //       flag = false;
  //     } else {
  //       let mut colliders: Vec<AxisAlignedCubeCollision> = Vec::new();
  //       for x in &[pos.x.floor() - 1f32, pos.x.floor(), pos.x.floor() + 1f32] {
  //         for y in &[pos.y.floor() - 1f32, pos.y.floor(), pos.y.floor() + 1f32] {
  //           for z in &[pos.z.floor() - 1f32, pos.z.floor(), pos.z.floor() + 1f32] {
  //             let cube_pos = Vec3F::new(x + 0.5f32, y + 0.5f32, z + 0.5f32);
  //             if self.is_occupied(&cube_pos, chunk_storage) {
  //               colliders.push(AxisAlignedCubeCollision::from_vecs(cube_pos, Vec3F::new(1f32, 1f32, 1f32)))
  //             }
  //           }
  //         }
  //       }
  //       let first_collider: Option<CollisionSummary> = colliders.iter().fold(None, |best, collider| {
  //         let curr_collider = collider.sphere_collision((&pos, &1f32), &direction);
  //         if let Some(prev_best) = best {
  //           curr_collider.map_or(Some(prev_best), |summary| if summary.time < prev_best.time {Some(summary)} else {Some(prev_best)})
  //         } else {
  //           curr_collider
  //         }
  //       });
  //       let collider = first_collider.expect("Could not find valid collider!");
  //       remaining_time -= collider.time;
  //       pos = collider.position;
  //       direction = direction * direction.dot(collider.surface_normal.normalize());
  //     }
  //   }
  //   pos
  // }

  pub fn is_occupied<'a>(&self, position: &Vec3F, chunk_storage: &ReadStorage<'a, ChunkComponent>) -> bool {
    if let Some(ret) = self
      .get_entity_from_coord(position)
      .map(|ett| chunk_storage.get(*ett).map_or(false, |chunk| chunk.collides(position)))
    {
      ret
    } else {
      false
    }
  }

  fn get_index_pair(&self, coord: &Vec2I) -> Vec2I {
    Vec2I::new(coord.x / CHUNK_DIMENSIONS.x as i32, coord.y / CHUNK_DIMENSIONS.z as i32)
  }

  fn get_index_from_position(&self, coord: &Vec3F) -> Vec2I {
    Vec2I::new(
      coord.x.floor() as i32 / CHUNK_DIMENSIONS.x as i32,
      coord.z.floor() as i32 / CHUNK_DIMENSIONS.z as i32,
    )
  }
}
