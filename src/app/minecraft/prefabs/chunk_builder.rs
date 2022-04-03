use cgmath::prelude::Zero;
use specs::{Entity, System, SystemData, World, WorldExt, Write};

use app::minecraft::{BlockGenerator, BlockSampler, ChunkComponent, ChunkGrid};
use ecs::{DrawableId, Material, MeshComponent, EntitySpawner, PrefabBuilder};
use physics::TransformComponent;
use renderer::{AttributeType, BufferLayout, DataBuffer, Drawable, IndexBuffer, Renderer, VertexArray};
use shapes::Block;
use utils::{QuatF, Vec2I, Vec3F};

#[derive(Clone, Debug)]
pub struct ChunkBuilderState {
  pub sampler: BlockGenerator,
  pub chunk_index: Vec2I,
}

impl ChunkBuilderState {
  pub fn new(seed: Vec2I, generator: BlockGenerator) -> Self {
    Self {
      sampler: generator,
      chunk_index: seed,
    }
  }
}

impl Default for ChunkBuilderState {
  fn default() -> Self {
    Self {
      sampler: BlockGenerator::default(),
      chunk_index: Vec2I::new(0, 0),
    }
  }
}

#[derive(Default)]
pub struct ChunkBuilder {
  block_drawable: Option<DrawableId>,
  block_material: Option<Material>,
}

impl<'a> PrefabBuilder<'a> for ChunkBuilder {
  type State = ChunkBuilderState;
  type EntityResources = Write<'a, ChunkGrid>;

  fn create<'b, F: Fn() -> EntitySpawner<'a, 'b>>(
    &self,
    state: &Self::State,
    chunk_grid: &mut Self::EntityResources,
    constructor: F,
  ) -> Vec<Entity> {
    let chunk_component = ChunkComponent::new(state.chunk_index, &state.sampler);
    let mut indices_buffer: Vec<u32> = Vec::new();
    let mut values_buffer: Vec<f32> = Vec::new();
    let buffer_layout = BufferLayout::new(vec![
      AttributeType::Float3,
      AttributeType::Float3,
      AttributeType::Float2,
    ]);
    let mut ret_vec = Vec::new();
    chunk_component.foreach_face(&mut |block, face| {
      let world_coordinate = block.world_coordinate();
      let (coords, inds) = face.buffer_info(world_coordinate, indices_buffer.len() as u32);
      indices_buffer.extend_from_slice(&inds);
      values_buffer.extend_from_slice(&coords);
    });
    let vao = VertexArray::new(
      DataBuffer::dynamic_buffer(&values_buffer, buffer_layout),
      IndexBuffer::create(indices_buffer),
    );
    let mesh = MeshComponent::new(vao, "default_texture".to_string());
    let chunk_entity = constructor()
      .with(chunk_component)
      .with(mesh)
      .with(self.block_material.clone().expect("Could not get material for block"))
      .with(TransformComponent::identity())
      .build();
    chunk_grid.add_chunk(state.chunk_index, chunk_entity);
    ret_vec.push(chunk_entity);
    ret_vec
  }

  fn setup_delegate(&mut self, world: &mut World) {
    world.register::<ChunkComponent>();
    world.insert::<ChunkGrid>(ChunkGrid::default());
    let template_block = Block::new("resources/minecraft/grass_block.png");
    self.block_material = Some(template_block.material());
    self.block_drawable = Some({
      let mut renderer = world.write_resource::<Renderer>();
      renderer.submit_model(template_block.mesh())
    })
  }
}
