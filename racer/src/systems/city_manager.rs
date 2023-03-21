use engine::ecs::EntityTree;
use engine::prefab::{Cube, PrefabBuilder};
use engine::{ecs::MonoBehavior, graphics::MeshComponent};

use specs::prelude::*;
use specs::SystemData;

use crate::city::{BuildingBuilder, BuildingReducer, City, CityBlock, CityBuilder};

#[derive(SystemData)]
pub struct CityManagerStorage<'a> {
  pub meshes: WriteStorage<'a, MeshComponent>,
}

#[derive(Default)]
pub struct CityManager;

impl<'a> MonoBehavior<'a> for CityManager {
  type SystemData = CityManagerStorage<'a>;

  fn setup(&mut self, world: engine::ecs::WorldProxy) {
    let grid = vec![
      vec![0, 0, 0, 0, 0, 0, 0],
      vec![0, 1, 0, 1, 1, 1, 0],
      vec![0, 1, 1, 1, 0, 1, 0],
      vec![0, 1, 0, 1, 0, 1, 0],
      vec![0, 0, 1, 1, 0, 1, 0],
      vec![0, 1, 1, 0, 0, 1, 0],
      vec![0, 1, 0, 1, 0, 1, 0],
      vec![0, 1, 1, 1, 1, 1, 0],
      vec![0, 1, 0, 1, 0, 1, 0],
      vec![0, 0, 1, 1, 0, 1, 0],
      vec![0, 1, 1, 0, 0, 1, 0],
      vec![0, 1, 0, 1, 1, 1, 0],
      vec![0, 1, 1, 1, 0, 1, 0],
      vec![0, 1, 0, 1, 0, 1, 0],
      vec![0, 0, 1, 1, 0, 1, 0],
      vec![0, 1, 1, 0, 0, 1, 0],
      vec![0, 1, 0, 1, 0, 1, 0],
      vec![0, 1, 1, 1, 1, 1, 0],
      vec![0, 1, 0, 1, 0, 1, 0],
      vec![0, 0, 1, 1, 0, 1, 0],
      vec![0, 1, 1, 0, 0, 1, 0],
      vec![0, 1, 0, 1, 0, 1, 0],
      vec![0, 1, 1, 1, 1, 1, 0],
      vec![0, 1, 0, 1, 0, 1, 0],
      vec![0, 0, 1, 1, 0, 1, 0],
      vec![0, 1, 1, 0, 0, 1, 0],
      vec![0, 0, 0, 0, 0, 0, 0],
    ];

    let grid = vec![
      vec![0, 0, 0, 0, 0, 0, 0],
      vec![0, 1, 1, 1, 1, 1, 0],
      vec![0, 1, 0, 0, 0, 1, 0],
      vec![0, 1, 0, 0, 0, 1, 0],
      vec![0, 1, 0, 0, 0, 1, 0],
      vec![0, 1, 1, 1, 1, 1, 0],
      vec![0, 0, 0, 0, 0, 0, 0],
    ];

    let grid: Vec<Vec<CityBlock>> = grid
      .iter()
      .map(|row| {
        row
          .iter()
          .map(|c| {
            if c == &0 {
              CityBlock::default_building()
            } else {
              CityBlock::default_road()
            }
          })
          .collect()
      })
      .collect();

    let city_builder = CityBuilder::from(grid);

    if let Some(city) = city_builder.build() {
      self.build(&world.api(), city);
    }
  }
}

impl PrefabBuilder for CityManager {
  type PrefabState = City;

  fn build<'a>(&mut self, api: &engine::ecs::SystemUtilities<'a>, state: Self::PrefabState) -> Entity {
    let mut builder = api.entity_builder();

    let mut cube_spawner = Cube::default();
    let mut building_spawner = BuildingBuilder::default();
    let mut entity_set = EntityTree::default();

    for road in state.roads.into_iter() {
      let road_block = road.get_cube_state();

      let cube = cube_spawner.build(api, road_block);

      entity_set.add(cube);
    }

    for (index, building_coords) in state.buildings.into_iter().enumerate() {
      let building_reducer = BuildingReducer::new(building_coords, index);

      let building = building_spawner.build(api, building_reducer);

      entity_set.add(building);
    }

    builder.with(entity_set);
    builder.consume()
  }
}
