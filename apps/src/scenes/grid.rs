use app::{
  entities::{create_player, create_floor},
  Cube,
};
use ecs::components::*;
use ecs::*;
use specs::prelude::*;
use utils::*;

use physics::TransformComponent;

use renderer::{Drawable, Renderer};

pub fn build_grid_scene(center: Vec3F, world: &mut World) {
  let player_pos = Vec3F::new(0f64, 0f64, 0f64);
  create_player(player_pos, world);
  create_floor(-6f64, 3f64, world);

  let mut grid_material = Material::new();
  grid_material.diffuse(Vec3F::new(0.8f64, 0.4f64, 0.1f64));

  let wire_box = WireBox::new(0.01, 60f64, center, 15);
  wire_box.build(world, &grid_material);
}

struct WireBox {
  spoke_frac: f64, // fraction of the linear dimension of the box that is taken up by the spokes.
  scale: f64,
  center_pos: Vec3F,
  num_cubes: u32,
}

impl WireBox {
  pub fn new(frac: f64, scale: f64, center: Vec3F, num_cubes: u32) -> WireBox {
    WireBox {
      spoke_frac: frac,
      scale,
      center_pos: center,
      num_cubes: num_cubes,
    }
  }

  fn build<'a>(self, world: &'a mut World, material: &Material) {
    let d = self.scale / 2f64;
    let seed_corner = self.center_pos - Vec3F::new(d, d, d);
    let cube = Cube::new(material.clone());
    let cube_id = {
      let mut renderer = world.write_resource::<Renderer>();
      renderer.submit_model(cube.state())
    };
    let spoke_width = self.spoke_frac * self.scale / self.num_cubes as f64;
    for d in 0..3 {
      for e in 0..self.num_cubes + 1 {
        for f in 0..self.num_cubes + 1 {
        let mut scaling_vec = Vec3F::new(spoke_width, spoke_width, spoke_width);
          scaling_vec[d] = self.scale;
          let increment = self.scale / self.num_cubes as f64;
          let add_transl = if d == 0 {
            // yz plane
            Vec3F::new(self.scale / 2f64, e as f64 * increment, f as f64 * increment)
          } else if d == 1 {
            // xz plane
            Vec3F::new(e as f64 * increment, self.scale / 2f64, f as f64 * increment)
          } else {
            // xy plane
            Vec3F::new(e as f64 * increment, f as f64 * increment, self.scale / 2f64)
          };
          world.create_entity()
            .with(TransformComponent::from(translate(add_transl + seed_corner) * nonunif_scale(scaling_vec)))
            .with(cube_id.clone())
            .build();
        }
      }
    }
  }
}
