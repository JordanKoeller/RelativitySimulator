use engine::datastructures::{Graph, GraphVisitor, GraphWalker};
use engine::graphics::Texture;
use engine::utils::{Vec2F, Vec2U, RGB};

use super::city_block::CityBlock;
use super::renderable_blocks::{BuildingBlock, RoadBlock};
use super::City;
use super::RoadType;

pub(super) struct CityBuilder {
  cells: Vec<CityBlock>,
  width: usize,
  height: usize,
}

impl From<Texture> for CityBuilder {
  fn from(texture: Texture) -> Self {
    let pixels = texture.pixels();
    Self {
      width: pixels.width(),
      height: pixels.height(),
      cells: (0..pixels.len())
        .map(|i| {
          let elem = &pixels[i];
          let rgb = RGB::new(elem[0], elem[1], elem[2]);
          CityBlock::from_pixel(&rgb)
        })
        .collect(),
    }
  }
}

impl From<Vec<Vec<CityBlock>>> for CityBuilder {
  fn from(blocks: Vec<Vec<CityBlock>>) -> Self {
    let height = blocks.len();
    let width = blocks[0].len();
    let mut buffer = Vec::new();
    for row in blocks.into_iter() {
      if row.len() != width {
        panic!("Invalid blocks vector. It was not square!");
      }
      for block in row.into_iter() {
        buffer.push(block)
      }
    }
    Self {
      cells: buffer,
      width,
      height,
    }
  }
}

impl CityBuilder {
  /**
   * Analyze the city image and confirm it is a valid configuration.
   *
   * This method validates:
   *     a. elevation changes gradually. (TODO)
   *     b. All roads are accessible.
   *     c. The city is completely enclosed.
   */
  pub fn is_valid(&self) -> bool {
    let visitor = GraphVisitor::new(self);
    self.is_enclosed() && !visitor.is_disjoint()
  }

  fn is_enclosed(&self) -> bool {
    for i in 0..self.width {
      if self[&[i, 0]].is_road() {
        return false;
      }
      if self[&[i, self.height - 1]].is_road() {
        return false;
      }
    }

    for i in 0..self.height {
      if self[&[0, i]].is_road() {
        return false;
      }
      if self[&[self.width - 1, 0]].is_road() {
        return false;
      }
    }

    true
  }

  fn if_road(&self, center: &Vec2U, offset_x: i32, offset_y: i32) -> Option<Vec2U> {
    if (center.x == 0 && offset_x < 0) || (center.y == 0 && offset_y < 0) {
      None
    } else {
      let coords = Vec2U::new(
        (center.x as i32 + offset_x) as usize,
        (center.y as i32 + offset_y) as usize,
      );
      if self[&coords].is_road() {
        Some(coords)
      } else {
        None
      }
    }
  }

  fn road_type(&self, node: &Vec2U) -> RoadType {
    let corners = [
      self.if_road(node, 0, -1),
      self.if_road(node, 1, 0),
      self.if_road(node, 0, 1),
      self.if_road(node, -1, 0),
    ];
    match corners {
      [Some(_), None, Some(_), None] => RoadType::Vertical,
      [Some(_), Some(_), None, None] => RoadType::NECorner,
      [Some(_), None, None, Some(_)] => RoadType::NWCorner,
      [None, Some(_), None, Some(_)] => RoadType::Horizontal,
      [None, Some(_), Some(_), None] => RoadType::SECorner,
      [None, None, Some(_), Some(_)] => RoadType::SWCorner,
      [Some(_), Some(_), Some(_), None] => RoadType::NESTee,
      [None, Some(_), Some(_), Some(_)] => RoadType::ESWTee,
      [Some(_), None, Some(_), Some(_)] => RoadType::SWNTee,
      [Some(_), Some(_), None, Some(_)] => RoadType::WNETee,
      [Some(_), Some(_), Some(_), Some(_)] => RoadType::Intersection,
      _ => panic!("Invalid road_type!"),
    }
  }

  /**
   * Builds a City object. If a City could not be built, returns None
   */
  pub fn build(mut self) -> Option<City> {
    if !self.is_valid() {
      return None;
    }

    let mut roads: Vec<RoadBlock> = Vec::new();
    let mut buildings: Vec<BuildingBlock> = Vec::new();

    let mut walker = GraphWalker::new(&self, |node| {
      let road_type = self.road_type(node);
      roads.push(RoadBlock::new(Vec2F::new(node.x as f32, node.y as f32), road_type));
      true
    });

    if let Some(start) = self.nodes().next() {
      walker.walk_bfs(start);
    }

    Some(City::new(roads, buildings))
  }
}

impl std::ops::Index<&[usize; 2]> for CityBuilder {
  type Output = CityBlock;

  fn index(&self, index: &[usize; 2]) -> &Self::Output {
    let i = index[0] + self.width * index[1];
    &self.cells[i]
  }
}

impl std::ops::Index<&Vec2U> for CityBuilder {
  type Output = CityBlock;

  fn index(&self, index: &Vec2U) -> &Self::Output {
    let i = index.x + self.width * index.y;
    &self.cells[i]
  }
}

impl Graph for CityBuilder {
  type Node = Vec2U;

  fn id(&self, node: &Self::Node) -> usize {
    node.x + self.width * node.y
  }

  fn adjacent(&self, node: &Self::Node) -> Box<dyn Iterator<Item = Self::Node> + '_> {
    Box::new(CityBuilderIterator::new_adjacent(node.x, node.y, &self))
  }

  fn nodes(&self) -> Box<dyn Iterator<Item = Self::Node> + '_> {
    Box::new(CityBuilderIterator::new(&self))
  }

  fn len(&self) -> usize {
    self.nodes().count()
  }
}

// Helper Structs

/**
 * Helper for iterating through the road cells in a city.
 *
 * Scheme:
 *
 *   1. If adjacent_only == 0, it should iterate through all city blocks in the map.
 *   2. If adjacent_only > 0, it iterates adjacent blocks to the current specified block
 *
 *   3. i, j start as width, height to indicate the current element has not been visited,
 *      afterwards which the current (i, j) has not been iterated yet.
 */
struct CityBuilderIterator<'a> {
  city_builder: &'a CityBuilder,
  ij: Vec2U,
  adjacent_only: u8,
}

impl<'a> CityBuilderIterator<'a> {
  fn new(city: &'a CityBuilder) -> Self {
    let mut ret = Self {
      city_builder: city,
      ij: Vec2U::new(0, 0),
      adjacent_only: 0,
    };
    ret.next_road(false);
    ret
  }

  fn new_adjacent(i: usize, j: usize, city_builder: &'a CityBuilder) -> Self {
    Self {
      city_builder,
      ij: Vec2U::new(i, j),
      adjacent_only: 1,
    }
  }

  fn if_road(&self, i: usize, j: usize) -> Option<Vec2U> {
    if self.is_road(i, j) {
      Some(Vec2U::new(i, j))
    } else {
      None
    }
  }

  fn adjacent_road(&mut self) -> Option<Vec2U> {
    while self.adjacent_only < 5 {
      let ret = match self.adjacent_only {
        1 => self.if_road(self.ij.x + 1, self.ij.y),
        2 => self.if_road(self.ij.x, self.ij.y + 1),
        3 => {
          if self.ij.x > 0 {
            self.if_road(self.ij.x - 1, self.ij.y)
          } else {
            None
          }
        }
        4 => {
          if self.ij.y > 0 {
            self.if_road(self.ij.x, self.ij.y - 1)
          } else {
            None
          }
        }
        _ => None,
      };
      self.adjacent_only += 1;
      if ret.is_some() {
        return ret;
      }
    }
    None
  }

  fn is_road(&self, i: usize, j: usize) -> bool {
    i < self.city_builder.width && j < self.city_builder.height && self.city_builder[&[i, j]].is_road()
  }

  fn terminated(&self) -> bool {
    self.ij.y == self.city_builder.height
  }

  fn next_road(&mut self, mut force_move: bool) {
    while !self.terminated() && (force_move || !self.is_road(self.ij.x, self.ij.y)) {
      force_move = false;
      self.ij.x += 1;
      if self.ij.x == self.city_builder.width {
        self.ij.y += 1;
        self.ij.x = 0;
      }
    }
  }
}

impl<'a> Iterator for CityBuilderIterator<'a> {
  type Item = Vec2U;

  fn next(&mut self) -> Option<Self::Item> {
    match self.adjacent_only {
      0 => {
        if !self.terminated() {
          let ret = self.ij.clone();
          self.next_road(true);
          Some(ret)
        } else {
          None
        }
      }
      _ => self.adjacent_road(),
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn nodes_iterator() {
    let grid_data = vec![
      vec![0, 0, 1, 0],
      vec![0, 1, 1, 1],
      vec![1, 1, 0, 1],
      vec![0, 1, 1, 1],
      vec![1, 1, 0, 0],
    ];

    assert_nodes(grid_data);
  }

  #[test]
  fn nodes_iterator_disjoint() {
    let grid_data = vec![
      vec![0, 0, 1, 0],
      vec![0, 1, 1, 1],
      vec![1, 1, 0, 0],
      vec![0, 0, 0, 0],
      vec![1, 1, 0, 0],
    ];

    assert_nodes(grid_data);
  }

  #[test]
  fn nodes_iterator_disjoint_2() {
    let grid_data = vec![
      vec![0, 0, 0, 0],
      vec![0, 1, 1, 1],
      vec![1, 1, 0, 0],
      vec![0, 0, 0, 0],
      vec![1, 1, 0, 0],
    ];

    assert_nodes(grid_data);
  }

  #[test]
  fn adjacent_success() {
    let grid_data = vec![
      vec![0, 0, 1, 0],
      vec![0, 1, 1, 1],
      vec![1, 1, 0, 0],
      vec![0, 1, 1, 0],
      vec![1, 1, 0, 0],
    ];

    let grid = get_grid(grid_data.clone());

    assert_set_eq(grid.adjacent(&Vec2U::new(2, 0)).collect(), &[Vec2U::new(2, 1)]);
    assert_set_eq(
      grid.adjacent(&Vec2U::new(1, 1)).collect(),
      &[Vec2U::new(2, 1), Vec2U::new(1, 2)],
    );
    assert_set_eq(
      grid.adjacent(&Vec2U::new(2, 1)).collect(),
      &[Vec2U::new(1, 1), Vec2U::new(3, 1), Vec2U::new(2, 0)],
    );
    assert_set_eq(grid.adjacent(&Vec2U::new(3, 1)).collect(), &[Vec2U::new(2, 1)]);
    assert_set_eq(grid.adjacent(&Vec2U::new(0, 2)).collect(), &[Vec2U::new(1, 2)]);
    assert_set_eq(
      grid.adjacent(&Vec2U::new(1, 2)).collect(),
      &[Vec2U::new(0, 2), Vec2U::new(1, 1), Vec2U::new(1, 3)],
    );
    assert_set_eq(
      grid.adjacent(&Vec2U::new(1, 3)).collect(),
      &[Vec2U::new(1, 2), Vec2U::new(2, 3), Vec2U::new(1, 4)],
    );
    assert_set_eq(grid.adjacent(&Vec2U::new(0, 4)).collect(), &[Vec2U::new(1, 4)]);
    assert_set_eq(
      grid.adjacent(&Vec2U::new(1, 4)).collect(),
      &[Vec2U::new(0, 4), Vec2U::new(1, 3)],
    );
  }

  #[test]
  fn validate_valid() {
    let grid = vec![
      vec![0, 0, 0, 0, 0, 0],
      vec![0, 0, 0, 1, 0, 0],
      vec![0, 0, 1, 1, 1, 0],
      vec![0, 1, 1, 0, 1, 0],
      vec![0, 0, 1, 1, 1, 0],
      vec![0, 1, 1, 0, 0, 0],
      vec![0, 0, 0, 0, 0, 0],
    ];
    assert_valid_grid(grid, true);
  }

  #[test]
  fn validate_disjoint_invalid() {
    let grid = vec![
      vec![0, 0, 1, 0],
      vec![0, 1, 1, 1],
      vec![1, 1, 0, 0],
      vec![0, 0, 0, 1],
      vec![1, 1, 1, 1],
    ];
    assert_valid_grid(grid, false);
  }

  #[test]
  fn validate_not_enclosed_invalid() {
    let grid = vec![
      vec![0, 0, 0, 1, 0, 0],
      vec![0, 0, 0, 1, 0, 0],
      vec![0, 0, 1, 1, 1, 0],
      vec![0, 1, 1, 0, 1, 0],
      vec![0, 0, 1, 1, 1, 0],
      vec![0, 1, 1, 0, 0, 0],
      vec![0, 0, 0, 0, 0, 0],
    ];
    assert_valid_grid(grid, false);
  }

  fn get_grid(grid: Vec<Vec<u8>>) -> CityBuilder {
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

    CityBuilder::from(grid)
  }

  fn assert_nodes(grid_data: Vec<Vec<u8>>) {
    let grid = get_grid(grid_data.clone());

    let expected_count: u8 = grid_data.iter().flat_map(|f| f.iter()).sum();

    let mut count = 0;
    for coord in grid.nodes() {
      assert_eq!(grid_data[coord.y][coord.x], 1);
      count += 1;
    }
    assert_eq!(count, expected_count);
  }

  fn assert_valid_grid(grid: Vec<Vec<u8>>, valid: bool) {
    let grid = get_grid(grid);
    assert_eq!(grid.is_valid(), valid);
  }

  fn assert_set_eq(actual: Vec<Vec2U>, expected: &[Vec2U]) {
    let set_1 = std::collections::HashSet::<Vec2U>::from_iter(actual.into_iter());
    let set_2 = std::collections::HashSet::<Vec2U>::from_iter(expected.iter().map(|e| e.clone()));

    assert_eq!(set_1, set_2);
  }
}
