use specs::prelude::*;

use ecs::*;
use renderer::*;
use utils::*;

use physics::TransformComponent;

static EXTENT: f32 = 1e4f32;

pub struct Floor {
  cube_scale: f32,
}

impl Floor {
  pub fn new(scale: f32) -> Self {
    Self { cube_scale: scale }
  }
}

impl Drawable for Floor {
  fn shader_name(&self) -> String {
    "default_texture".to_string()
  }

  fn material(&self) -> Material {
    let mut material = Material::new();
    material.diffuse_texture(Texture::from_file("resources/textures/checkerboard.png"));
    material
  }

  fn vertex_array(&self) -> VertexArray {
    let layout = BufferLayout::new(vec![
      AttributeType::Float3,
      AttributeType::Float3,
      AttributeType::Float2,
    ]);
    let extent = EXTENT;
    let num_tiles = extent / self.cube_scale;
    let uv_mult = num_tiles / 4f32;
    let cube_verts = cube_verts(uv_mult);
    let inds = vec![2, 1, 0, 0, 3, 2];
    let vert_buff = VertexBuffer::create(cube_verts, layout);
    let ind_buff = IndexBuffer::create(inds);
    VertexArray::new(vec![vert_buff], ind_buff)
  }
}

fn cube_verts(uv_mult: f32) -> Vec<f32> {
  vec![
    -0.5f32,
    0f32,
    -0.5f32,
    0f32,
    1f32,
    0f32,
    0f32 * uv_mult,
    0f32 * uv_mult,
    0.5f32,
    0f32,
    -0.5f32,
    0f32,
    1f32,
    0f32,
    1f32 * uv_mult,
    0f32 * uv_mult,
    0.5f32,
    0f32,
    0.5f32,
    0f32,
    1f32,
    0f32,
    1f32 * uv_mult,
    1f32 * uv_mult,
    -0.5f32,
    0f32,
    0.5f32,
    0f32,
    1f32,
    0f32,
    0f32 * uv_mult,
    1f32 * uv_mult,
  ]
}

pub fn create_floor<'a>(height: f32, cube_scale: f32, world: &'a mut World) {
  let translation = translate(Vec3F::unit_y() * height);
  let scale = Mat4F::from_nonuniform_scale(EXTENT, 1f32, EXTENT);
  let floor = Floor::new(cube_scale);
  let transform = TransformComponent::from(scale * translation);
  // let drawable_id = {
  //   let mut renderer = world.write_resource::<Renderer>();
  //   renderer.submit_model(floor.state())
  // };
    world.create_entity()
    .with_drawable(&floor)
    .with(transform)
    .build();
}