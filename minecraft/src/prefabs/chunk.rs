use engine::ecs::{PrefabBuilder, SystemUtilities};
use engine::prefab::{Cube, CubeState};
use engine::utils::Vec3F;

pub struct Chunk;

#[derive(Default)]
pub struct ChunkBuilder;

impl PrefabBuilder for ChunkBuilder {
    type PrefabState = Chunk;

    fn build<'a>(&mut self, api: &SystemUtilities<'a>, _state: Self::PrefabState) {
        let mut cube_builder = Cube::default();
        let (x_dim, y_dim, z_dim) = (32usize, 4usize, 32usize);
        for x in 0..x_dim {
            for y in 0..y_dim {
                for z in 0..z_dim {
                    let cube_state = CubeState::new(
                        "resources/minecraft/grass_block.png",
                        Vec3F::new(x as f32, 1f32 - y as f32, z as f32),
                    );
                    cube_builder.build(api, cube_state);
                }
            }
        }
    }
}
