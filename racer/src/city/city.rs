use super::RoadBlock;
use engine::utils::Vec2U;

pub struct City {
  pub roads: Vec<RoadBlock>,
  pub buildings: Vec<Vec<Vec2U>>,
}

impl City {
  pub fn new(roads: Vec<RoadBlock>, buildings: Vec<Vec<Vec2U>>) -> Self {
    Self { roads, buildings }
  }
}
