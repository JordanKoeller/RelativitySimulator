use cgmath::prelude::*;
use specs::prelude::*;

use app::minecraft::ChunkGrid;
use ecs::{DrawableId, EntitySpawner, Material, MeshComponent, PrefabBuilder};
use physics::TransformComponent;
use renderer::{AttributeType, BufferLayout, DataBuffer, Drawable, IndexBuffer, Renderer, VertexArray};
use shapes::Block;
use utils::{Vec2I, Vec3F, Vec3I};

#[derive(Clone, Debug)]
pub struct BlockHighlightState {
    chunk_index: Vec2I,
    block_index: Vec3I,
}

impl Default for BlockHighlightState {
    fn default() -> Self {
        Self {
            chunk_index: Vec2I::zero(),
            block_index: Vec3I::zero(),
        }
    }
}

impl BlockHighlightState {
    pub fn new(chunk_index: Vec2I, block_index: Vec3I) -> Self {
        Self {
            chunk_index,
            block_index,
        }
    }
}

#[derive(Default)]
pub struct BlockHighlightBuilder {
    id: Option<DrawableId>,
    material: Option<Material>,
}

impl<'a> PrefabBuilder<'a> for BlockHighlightBuilder {
    type State = BlockHighlightState;
    type EntityResources = Read<'a, ChunkGrid>;

    fn create<'b, F: Fn() -> EntitySpawner<'a, 'b>>(
        &self,
        state: &Self::State,
        chunk_grid: &mut Self::EntityResources,
        constructor: F,
    ) -> Vec<Entity> {
        let mut transform = TransformComponent::identity();
        transform.push_scale(Vec3F::new(1.01f32, 1.01f32, 1.01f32));
        let position = chunk_grid.get_position(&state.chunk_index, &state.block_index);
        transform.push_translation(position);
        let entity = constructor()
            .with(self.id.clone().unwrap())
            .with(transform)
            .with(self.material.clone().unwrap())
            .build();
        vec![entity]
    }

    fn setup_delegate(&mut self, world: &mut World) {
        let block = Block::new("resources/minecraft/white.jpg");
        let mat = block.material();
        let d_id = {
            let mut renderer = world.write_resource::<Renderer>();
            renderer.submit_model(block.mesh())
        };
        self.id = Some(d_id);
        self.material = Some(mat);
    }
}
