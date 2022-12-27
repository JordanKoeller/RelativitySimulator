use engine::ecs::EntityTree;
use engine::prefab::{Cube, CubeState, PrefabBuilder};
use engine::{ecs::MonoBehavior, graphics::MeshComponent};
use engine::utils::Vec3F;

use specs::prelude::*;
use specs::SystemData;

use crate::city::City;

#[derive(SystemData)]
pub struct CityManagerStorage<'a> {
  pub meshes: WriteStorage<'a, MeshComponent>,
}

#[derive(Default)]
pub struct CityManager;

impl<'a> MonoBehavior<'a> for CityManager {
  type SystemData = CityManagerStorage<'a>;
}

impl PrefabBuilder for CityManager {
  type PrefabState = City;

  fn build<'a>(&mut self, api: &engine::ecs::SystemUtilities<'a>, state: Self::PrefabState) -> Entity {
    let mut builder = api.entity_builder();

    let mut cube_spawner = Cube::default();
    let mut entity_set = EntityTree::default();

    for road in state.roads.into_iter() {
      let road_block = CubeState::new(road.get_texture_name(), Vec3F::new(road.center.x, 0f32, road.center.y));

      let cube = cube_spawner.build(api, road_block);

      entity_set.add(cube);
    }
    builder.with(entity_set);
    builder.consume()
  }
}
