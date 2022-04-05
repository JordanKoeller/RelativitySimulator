use specs::prelude::*;
use cgmath::Zero;

use app::{entities::create_floor, entities::create_player, Cube, FaceCube};
use renderer::*;
use utils::*;

use ecs::*;

use physics::TransformComponent;

pub fn build_rotate_boxes(num_boxes: u32, scale: f32, start_pos: Vec3F, delta: Vec3F, world: &mut World) {
  create_player(Vec3F::new(-50f32, 38f32, 25f32), world);
  let mut mtl = Material::new();
  mtl.diffuse(Vec3F::new(1f32, 0.5f32, 0.5f32));
  let cube = FaceCube { c: Cube::new(mtl) };
  create_floor(start_pos.y - scale / 2f32, scale, world);
  let drawable_id = {
    let mut renderer = world.write_resource::<Renderer>();
    renderer.submit_model(cube.state())
  };
  for i in 0..num_boxes {
    let pos = start_pos + delta * i as f32;
    let scale = Vec3F::new(scale, scale, scale);
    let transform = TransformComponent::new(pos, scale, QuatF::zero());
    world.create_entity()
      .with(drawable_id.clone())
      .with(transform)
      .build();
  }
}

