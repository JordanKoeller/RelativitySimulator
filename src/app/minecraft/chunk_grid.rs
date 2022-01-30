use cgmath::prelude::*;
use std::collections::HashMap;

use app::minecraft::{ChunkComponent, CHUNK_DIMENSIONS};
use app::AxisAlignedCubeCollision;
use shapes::{BlockFace, get_index_shift};
use specs::{Entity, ReadStorage};
use utils::{line_intersects_block, Vec2I, Vec3F, Vec3I};

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

    pub fn get_position(&self, chunk_index: &Vec2I, block_index: &Vec3I) -> Vec3F {
        let voxel_coord = Vec3I::new(chunk_index.x, 0, chunk_index.y).mul_element_wise(CHUNK_DIMENSIONS) + block_index;
        Vec3F::new(
            voxel_coord.x as f32 + 0.5f32,
            voxel_coord.y as f32 + 0.5f32,
            voxel_coord.z as f32 + 0.5f32,
        )
    }

    pub fn get_nearby_collidables<'a>(
        &self,
        pos: &Vec3F,
        chunk_storage: &ReadStorage<'a, ChunkComponent>,
    ) -> Vec<AxisAlignedCubeCollision> {
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
                                Vec3F::new(1f32, 1f32, 1f32),
                            ));
                        }
                    }
                }
            }
        }
        ret
    }

    // Given a starting position and an ending position, return the ID of the first block along that line, as well as which face was intersected.
    pub fn get_colliding_along_line<'a>(
        &self,
        start: &Vec3F,
        end: &Vec3F,
        chunk_storage: &ReadStorage<'a, ChunkComponent>,
    ) -> Option<(Vec2I, Vec3I, BlockFace)> {
        let mut query_pt = start.clone();
        let mut chunk_index = self.get_index_from_position(&query_pt);
        let mut flag = true;
        let mut c = 0i32;
        while flag && c < 5 {
            c += 1;
            println!("OUTER LOOP");
            let chunk = chunk_storage.get(*self.data.get(&chunk_index).unwrap()).unwrap();
            let (low, high) = chunk.chunk_dimensions();
            if let Some((intersection, face)) = line_intersects_block(&query_pt, end, &low, &high) {
                if let Some((block_id, face)) = chunk.get_first_on_line(&query_pt, end) {
                    return Some((chunk_index, block_id, face));
                } else {
                    let chunk_shift = get_index_shift(&face);
                    if chunk_shift.y != 0 {
                        flag = false;
                    } else {
                        chunk_index = chunk_index + Vec2I::new(chunk_shift.x, chunk_shift.z);
                        query_pt = intersection;
                        flag = self.data.contains_key(&chunk_index);
                    }
                }
            } else {
                flag = false;
            }
        }
        None
    }

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
