use engine::utils::RGB;

////////////////////////////////////
/// First define Structs and parsing
////////////////////////////////////
#[derive(Eq, PartialEq, Debug)]
pub(super) enum CityBlock {
  Road {
    elevation: u8,
  },
  Building {
    height: u8,
    shape: BuildingShape,
    elevation: u8,
  },
}

impl CityBlock {
  pub fn is_road(&self) -> bool {
    if let Self::Road { elevation: _ } = self {
      true
    } else {
      false
    }
  }
}

#[derive(Eq, PartialEq, Debug)]
pub(super) struct BuildingShape {
  pub north: WallInterpolation,
  pub east: WallInterpolation,
  pub south: WallInterpolation,
  pub west: WallInterpolation,
}

#[derive(Eq, PartialEq, Debug)]
pub(super) enum WallInterpolation {
  JAGGED,   // Encode 00
  LINEAR,   // Encode 01
  CIRCULAR, // Encode 10
  BEZIER,   // Encode 11
}

impl CityBlock {
  pub fn from_pixel(pixel: &RGB) -> Self {
    if pixel.x == 0u8 {
      Self::Road { elevation: pixel.z }
    } else {
      Self::Building {
        height: pixel.x,
        shape: BuildingShape::new(pixel.y),
        elevation: pixel.z,
      }
    }
  }

  pub fn default_building() -> Self {
    Self::Building {
      height: 1,
      shape: BuildingShape {
        north: WallInterpolation::LINEAR,
        east: WallInterpolation::LINEAR,
        south: WallInterpolation::LINEAR,
        west: WallInterpolation::LINEAR,
      },
      elevation: 1,
    }
  }

  pub fn default_road() -> Self {
    Self::Road { elevation: 1 }
  }
}

impl BuildingShape {
  // Encoded as NNEESSWW, 2 bits per wall
  pub fn new(spec: u8) -> Self {
    Self {
      north: WallInterpolation::decode(spec >> 6),
      east: WallInterpolation::decode((spec & 0b00110000) >> 4),
      south: WallInterpolation::decode((spec & 0b00001100) >> 2),
      west: WallInterpolation::decode(spec & 0b00000011),
    }
  }
}

impl WallInterpolation {
  fn decode(spec: u8) -> Self {
    match spec {
      0 => Self::JAGGED,
      1 => Self::LINEAR,
      2 => Self::CIRCULAR,
      3 => Self::BEZIER,
      _ => panic!("Invalid wall encoding"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn building_shape_decode() {
    let spec = 0b00011011; // Jagged, Linear, Circular, Bezier

    let expected = BuildingShape {
      north: WallInterpolation::JAGGED,
      east: WallInterpolation::LINEAR,
      south: WallInterpolation::CIRCULAR,
      west: WallInterpolation::BEZIER,
    };

    let actual = BuildingShape::new(spec);

    assert_eq!(expected, actual);
  }

  #[test]
  fn cityblock_road() {
    let spec = RGB::new(0u8, 1u8, 2u8);

    let expected = CityBlock::Road { elevation: 2 };

    let actual = CityBlock::from_pixel(&spec);

    assert_eq!(expected, actual);
  }

  #[test]
  fn cityblock_building() {
    let spec = RGB::new(12u8, 0b10101010, 128u8);

    let expected = CityBlock::Building {
      height: 12,
      shape: BuildingShape {
        north: WallInterpolation::CIRCULAR,
        east: WallInterpolation::CIRCULAR,
        south: WallInterpolation::CIRCULAR,
        west: WallInterpolation::CIRCULAR,
      },
      elevation: 128u8,
    };

    let actual = CityBlock::from_pixel(&spec);

    assert_eq!(expected, actual);
  }
}
