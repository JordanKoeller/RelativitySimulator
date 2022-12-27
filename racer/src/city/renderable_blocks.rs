use engine::utils::Vec2F;


pub struct RoadBlock {
  pub center: Vec2F,
  pub road_type: RoadType,
}

impl RoadBlock {
  pub fn new(center: Vec2F, road_type: RoadType) -> Self {
    Self {
      center,
      road_type,
    }
  }

  pub fn get_texture_name(&self) -> &str {
    ""
  }
}

pub struct BuildingBlock {
  center: Vec2F,
}

pub enum RoadType {
  Horizontal,
  Vertical,
  NESTee,
  ESWTee,
  SWNTee,
  WNETee,
  NECorner,
  NWCorner,
  SECorner,
  SWCorner,
  Intersection,
}