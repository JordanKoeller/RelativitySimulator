use engine::ecs::CubeState;
use engine::prefab::{CubeFace, CubeStateBuilder};
use engine::utils::{Vec2F, Vec3F};

pub struct RoadBlock {
  pub center: Vec2F,
  pub road_type: RoadType,
}

impl RoadBlock {
  pub fn new(center: Vec2F, road_type: RoadType) -> Self {
    Self { center, road_type }
  }

  pub fn get_cube_state(&self) -> CubeState {
    CubeStateBuilder::default()
      .position(Vec3F::new(self.center.x * 5f32, 0f32, self.center.y * 5f32))
      .scale(Vec3F::new(5f32, 0.1f32, 5f32))
      .rotation(self.get_rotation())
      .texture_filename(self.get_filename())
      .faces(vec![CubeFace::Top])
      .build()
      .unwrap()
  }

  fn get_rotation(&self) -> Vec3F {
    let angle_f32 = match self.road_type {
      RoadType::Horizontal => 0f32,
      RoadType::Vertical => 90f32,
      RoadType::Intersection => 0f32,
      RoadType::ESWTee => 0f32,
      RoadType::NESTee => 90f32,
      RoadType::WNETee => 180f32,
      RoadType::SWNTee => 270f32,
      RoadType::SECorner => 0f32,
      RoadType::NECorner => 90f32,
      RoadType::NWCorner => 180f32,
      RoadType::SWCorner => 270f32,
    };
    Vec3F::new(0f32, angle_f32, 0f32)
  }

  fn get_filename(&self) -> String {
    let name: &str = match self.road_type {
      RoadType::Horizontal => "straightaway-1.jpg",
      RoadType::Vertical => "straightaway-1.jpg",
      RoadType::Intersection => "intersection-1.jpg",
      RoadType::ESWTee => "tee.jpg",
      RoadType::NESTee => "tee.jpg",
      RoadType::WNETee => "tee.jpg",
      RoadType::SWNTee => "tee.jpg",
      RoadType::SECorner => "bend-1.jpg",
      RoadType::NECorner => "bend-1.jpg",
      RoadType::NWCorner => "bend-1.jpg",
      RoadType::SWCorner => "bend-1.jpg",
    };
    let filename = format!("resources/roads/{}", name);
    filename.into()
  }
}

#[derive(Debug)]
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
