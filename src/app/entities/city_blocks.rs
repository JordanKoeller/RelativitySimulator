use cgmath::ElementWise;
use ecs::*;
use events::{StatefulEventChannel, EventChannel};
use specs::prelude::*;
use utils::*;

use app::{BuildingState, StreetPiece, StreetState};

const BUILDING: char = '#';
const STREET: char = '.';

#[derive(Clone, Debug)]
pub struct DistrictState {
  position: Vec3F,
  footprint: Vec2F,
  layout: String,
}

impl Default for DistrictState {
  fn default() -> Self {
    Self {
      position: Vec3F::new(0f32, 0f32, 0f32),
      footprint: Vec2F::new(0f32, 0f32),
      layout: "#@#@#\n#@#@#\n#@#@#\n#@#@#\n#@#@#".to_string(),
    }
  }
}

impl DistrictState {
  pub fn new(position: Vec3F, footprint: Vec2F, layout: String) -> Self {
    Self {
      position,
      footprint,
      layout,
    }
  }
}

type DistrictStateData<'a> = (
  Write<'a, StatefulEventChannel<EntityCrudEvent, StreetState>>,
  Write<'a, StatefulEventChannel<EntityCrudEvent, BuildingState>>,
);

#[derive(Default, Debug)]
pub struct DistrictDelegate;
impl<'a> EntityDelegate<'a> for DistrictDelegate {
  type State = DistrictState;
  type EntityResources = DistrictStateData<'a>;

  fn create<'b, F: Fn() -> MyBuilder<'a, 'b>>(
    &self,
    state: Self::State,
    resources: &mut Self::EntityResources,
    _constructor: F,
  ) -> Vec<Entity> {
    let mut lines: Vec<Vec<(char, bool)>> = state // tuples of charcter and a bool for the block has been processed already
      .layout
      .lines()
      .map(|l| l.to_string().chars().map(|c| (c, false)).collect())
      .collect();
    let num_rows = lines.len();
    let num_cols = lines.iter().map(|l| l.len()).max().unwrap();
    let block_scale = state
      .footprint
      .div_element_wise(Vec2F::new(num_cols as f32, num_rows as f32));
    for i in 0..num_rows {
      for j in 0..num_cols {
        let offset = state.position + Vec3F::new(i as f32 * block_scale.x, 0f32, j as f32 * block_scale.y);
        if !lines[i][j].1 && lines[i][j].0 == BUILDING {
          // A building
          self.make_building(&mut resources.1, &mut lines, i, j, &block_scale, state.position + offset);
        } else if !lines[i][j].1 && lines[i][j].0 == STREET {
          self.make_road(&mut resources.0, &mut lines, i, j, &block_scale, state.position + offset);
        }
      }
    }
    vec![]
  }  
}

fn get(lines: &Vec<Vec<(char, bool)>>, i: usize, j: usize) -> char {
  match lines.get(i) {
    Some(sub) => {
      match sub.get(j) {
        Some((c, _)) => c.clone(),
        None => '0'
      }
    },
    None => '0'
  }
}

impl DistrictDelegate {

  fn merge_buildings(&self, lines: &Vec<Vec<(char, bool)>>, origin: Vec2I, dims: Vec2I) -> Option<Vec2I> {
    // Note: I only need to grow in the positive direction because the negative direction
    // is implicitly covered since blocks at lower indices should already be consumed.
    let mut new_dims: Vec<Vec2I> = vec![];
    if self.check_valid_building(lines, Vec2I::new(origin.x + dims.x, origin.y), Vec2I::new(1, dims.y)) {
      let new_dim = Vec2I::new(dims.x + 1, dims.y);
      let grow_x = self.merge_buildings(lines, origin, new_dim);
      match grow_x {
        Some(grown) => new_dims.push(grown),
        None => {}
      }
    }
    if self.check_valid_building(lines, Vec2I::new(origin.x, origin.y + dims.y), Vec2I::new(dims.x, 1)) {
      let new_dim = Vec2I::new(dims.x, dims.y + 1);
      let grow_y = self.merge_buildings(lines, origin, new_dim);
      match grow_y {
        Some(grown) => new_dims.push(grown),
        None => {}
      }
    }
    if new_dims.len() == 0 {
      Some(dims)
    } else {
      let ret = new_dims.iter().fold(dims, |acc, &elem| {
        if elem.x * elem.y > acc.x * acc.y {
          elem
        } else {
          acc
        }
      });
      Some(ret)
    }
  }

  fn check_valid_building(&self, lines: &Vec<Vec<(char, bool)>>, origin: Vec2I, dims: Vec2I) -> bool {
    let last_opt = lines.get((origin.x + dims.x - 1) as usize).and_then(|ll| {ll.get((origin.y+dims.y - 1) as usize)});
    match last_opt {
      Some(_) => {
        for i in origin.x..origin.x + dims.x {
          for j in origin.y..origin.y + dims.y {
            if !(!lines[i as usize][j as usize].1 && lines[i as usize][j as usize].0 == BUILDING) {
              return false;
            }
          }
        }
        true
      },
      None => {
        false
      }
    }
  }

  fn make_building<'a>(&self,
    evts: &mut Write<'a, StatefulEventChannel<EntityCrudEvent, BuildingState>>,
    lines: &mut Vec<Vec<(char, bool)>>, i: usize, j: usize, block_scale: &Vec2F, position: Vec3F) {
      // Find the max dims of a contiguous building
      let dims = match self.merge_buildings(&lines, Vec2I::new(i as i32,j as i32), Vec2I::new(1,1)) {
        Some(new_dim) => {new_dim}
        None => Vec2I::new(1,1)
      };
      // Set booleans for all the consumed blocks to true
      for ii in i..i+dims.x as usize {
        for jj in j..j+dims.y as usize {
          lines[ii][jj].1 = true;
        }
      }
      // Publish the building state
      let my_dims = Vec3F::new(block_scale.x * dims.x as f32, 20f32, block_scale.y * dims.y as f32);
      let bl_corner = position - Vec3F::new(block_scale.x / 2f32, 0f32, block_scale.y / 2f32);
      let new_center = bl_corner + Vec3F::new(my_dims.x / 2f32, 0f32, my_dims.z / 2f32);
      evts.publish((EntityCrudEvent::Create, BuildingState::new(
        new_center,
        my_dims,
        0.7f32
      )));
  }

  fn make_road<'a>(&self,
    evts: &mut Write<'a, StatefulEventChannel<EntityCrudEvent, StreetState>>,
    l: &mut Vec<Vec<(char, bool)>>, i: usize, j: usize, block_scale: &Vec2F, position: Vec3F) {
      l[i][j].1 = true;
      let char_slice: [char; 9] = [
        get(&l, i - 1, j -1), get(&l, i -1, j), get(&l, i - 1, j + 1),
        get(&l, i, j -1), get(&l, i, j), get(&l, i, j + 1),
        get(&l, i + 1, j -1), get(&l, i +1, j), get(&l, i + 1, j + 1),];
      let found = STREET_PIECE_LOOKUP.iter().find_map(|(arr, piece, rotation)| {
        if arr.iter().all(|&ind| char_slice[ind] == STREET) {
          Some((piece, rotation))
        } else {
          None
        }
      });
      match found {
        Some((piece, rotation)) => {
          evts.publish((EntityCrudEvent::Create, StreetState::new(
            position, *block_scale, *rotation, piece.clone()
          )));
        }
        None => {
          println!("########## FAILED ##############");
          println!("{:?}", &char_slice[0..3]);
          println!("{:?}", &char_slice[3..6]);
          println!("{:?}", &char_slice[6..9]);
          println!("################################");
        }
      }
  }
}

const STREET_PIECE_LOOKUP: [(&[usize], StreetPiece, cgmath::Deg<f32>); 13] = [
  (&[1, 3, 4, 5, 7], StreetPiece::Intersection, cgmath::Deg(0f32)), // Intersection
  (&[3,4,5,7], StreetPiece::Tee, cgmath::Deg(90f32)), // Tee down
  (&[1,4,5,7], StreetPiece::Tee, cgmath::Deg(0f32)), // Tee right
  (&[1,3,4,5], StreetPiece::Tee, cgmath::Deg(270f32)), // Tee up
  (&[1,3,4,7], StreetPiece::Tee, cgmath::Deg(180f32)), // Tee left
  (&[4,5,7], StreetPiece::Turn, cgmath::Deg(0f32)), // Turn bottom - right
  (&[1,4,5], StreetPiece::Turn, cgmath::Deg(270f32)), // Turn top - right
  (&[1,3,4], StreetPiece::Turn, cgmath::Deg(180f32)), // Turn top - left
  (&[3,4,7], StreetPiece::Turn, cgmath::Deg(90f32)), // Turn bottom - left
  (&[1,4], StreetPiece::Straightaway, cgmath::Deg(0f32)), // Vertical straight
  (&[4,7], StreetPiece::Straightaway, cgmath::Deg(0f32)), // Vertical Straight
  (&[3,4], StreetPiece::Straightaway, cgmath::Deg(90f32)), // Horizontal Straight
  (&[4,5], StreetPiece::Straightaway, cgmath::Deg(90f32)), // Horizontal Straight
];