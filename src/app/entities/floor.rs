use specs::prelude::*;

use ecs::*;
use utils::*;
use app::{TEXTURE_CUBE_VERTICES};
use renderer::*;


static EXTENT: f32 = 1e4f32;

pub struct Floor {
  height: f32,
  cube_scale: f32,
}

impl Floor {
  pub fn new(h: f32, cube_scale: f32) -> Floor {
    Floor {
      height: h,
      cube_scale,

    }
  }

  pub fn get_constructor<'a>(&self, world: &'a mut World) -> EntityConstructor<'a> {
    let constructor = EntityConstructor::new(world);
    let translation = translate(Vec3F::unit_y() * self.height);
    let scale = Mat4F::from_nonuniform_scale(EXTENT, 1f32, EXTENT);
    let cube = Floor::get_drawable(self.cube_scale);
    let constructor = constructor.add(cube.renderable());
    let constructor = constructor.add(Transform(scale * translation));
    constructor
  }

  fn get_drawable(cube_scale: f32) -> DefaultDrawable {
    let layout = BufferLayout::new(vec![AttributeType::Float3, AttributeType::Float3, AttributeType::Float2]);
    let extent = EXTENT;
    let num_tiles = extent / cube_scale;
    let uv_mult = num_tiles / 4f32;
    let cube_verts = vec![
      -0.5f32, 0f32, -0.5f32, 0f32, 1f32, 0f32, 0f32 * uv_mult, 0f32 * uv_mult,
      0.5f32,  0f32, -0.5f32, 0f32, 1f32, 0f32, 1f32 * uv_mult, 0f32 * uv_mult,
      0.5f32,  0f32,  0.5f32, 0f32, 1f32, 0f32, 1f32 * uv_mult, 1f32 * uv_mult,
      -0.5f32, 0f32,  0.5f32, 0f32, 1f32, 0f32, 0f32 * uv_mult, 1f32 * uv_mult,
    ];
    let inds = vec![2, 1, 0, 0, 3, 2];
    let vert_buff = VertexBuffer::create(cube_verts, layout);
    let ind_buff = IndexBuffer::create(inds);
    let vertex_array = VertexArray::new(vec![vert_buff], ind_buff);
    let mut material = Material::new();
    material.diffuse_texture(Texture::from_file("resources/textures/checkerboard.png"));
    DefaultDrawable::new_textured(
      vertex_array,
      material,
    )
  }
}
