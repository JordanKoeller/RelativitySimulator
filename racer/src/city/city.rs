use engine::prefab::PrefabBuilder;

use super::{BuildingBlock, RoadBlock};

pub struct City {
  pub roads: Vec<RoadBlock>,
  pub buildings: Vec<BuildingBlock>,
}

impl City {
  pub fn new(roads: Vec<RoadBlock>, buildings: Vec<BuildingBlock>) -> Self {
    Self { roads, buildings }
  }
}