use std::collections::{HashMap, HashSet};

use engine::{
  graphics::{
    Assets, ColorSpace, MaterialComponent, MeshBuilder, MeshComponent, ShadingStrategy, TextureBuilder,
    VertexArrayBuilder,
  },
  physics::TransformComponent,
  prefab::{CubeFace, PrefabBuilder},
  utils::{Vec2F, Vec2U, Vec3F},
};

#[derive(Default)]
pub struct BuildingBuilder;
impl PrefabBuilder for BuildingBuilder {
  type PrefabState = BuildingReducer;

  fn build<'a>(&mut self, api: &engine::ecs::SystemUtilities<'a>, state: Self::PrefabState) -> specs::Entity {
    let mut mesh_builder = MeshBuilder::default()
      .with_shading_strategy(ShadingStrategy::PerFace)
      .next();

    // Build up the mesh coordinates of the perimeter
    for (start, end, direction) in state.perimeter() {
      let midpt = Vec2F::new((start.x + end.x) as f32 / 2f32, (start.y + end.y) as f32 / 2f32);
      let span = Vec2F::new(
        end.x as f32 - start.x as f32 + 1f32,
        end.y as f32 - start.y as f32 + 1f32,
      );
      let face_coords = direction.coords();
      for i in 0..6 {
        let ii = i * 5;
        mesh_builder.push_vertex_flat(
          face_coords[ii] * span.x + midpt.x,
          face_coords[ii + 1],
          face_coords[ii + 2] * span.y + midpt.y,
          face_coords[ii + 3] * span.x,
          face_coords[ii + 4] * span.y,
        );
      }
    }

    let mesh_builder: VertexArrayBuilder = mesh_builder.next().into();
    let vai = api.get_else(&format!("building-{}", state.building_id), mesh_builder);
    let mesh = MeshComponent::new(vai, api.get_shader("default_texture").unwrap());

    let mut material = MaterialComponent::default();
    material.diffuse_texture(
      api.get_else(
        "resources/debug/checkerboard.png",
        TextureBuilder::default()
          .with_color_space(ColorSpace::SRGB)
          .with_file("resources/debug/checkerboard.png"),
      ),
    );

    let mut t1 = TransformComponent::identity();
    t1.push_scale(Vec3F::new(5f32, 5f32, 5f32));
    t1.push_translation(Vec3F::new(0f32, 2.5f32, 0f32));

    let mut eb = api.entity_builder();

    let ett = eb.spawn_child();
    ett.with(material.clone()).with(t1).with(mesh.clone());

    eb.consume()
  }
}

pub struct BuildingReducer {
  coordinates: HashMap<Vec2U, ()>,
  building_id: usize,
}

impl BuildingReducer {
  pub fn new(coordinates: Vec<Vec2U>, building_id: usize) -> Self {
    Self {
      coordinates: HashMap::from_iter(coordinates.into_iter().map(|v| (v, ()))),
      building_id,
    }
  }

  fn perimeter(&self) -> Vec<(Vec2U, Vec2U, CubeFace)> {
    let mut perimeter_walls: HashSet<(Vec2U, CubeFace)> = self
      .coordinates
      .keys()
      .filter(|block| self.cardinality(block) < 4)
      .flat_map(|block| {
        CubeFace::cardinal_directions()
          .into_iter()
          .filter(|d| self.is_exposed(block, *d))
          .map(|d| (block.clone(), d))
      })
      .collect();

    let mut walls: Vec<(Vec2U, Vec2U, CubeFace)> = Vec::default();

    while !perimeter_walls.is_empty() {
      let wall_elem = perimeter_walls.iter().next().unwrap().clone();

      walls.push((wall_elem.0, wall_elem.0, wall_elem.1));

      let mut wall_stack = vec![wall_elem];
      while !wall_stack.is_empty() {
        let (seed, direction) = wall_stack.pop().unwrap();
        perimeter_walls.remove(&(seed, direction));

        let (mut start, mut end, _) = walls.pop().unwrap();
        if seed.x < start.x || seed.y < start.y {
          start = seed;
        }
        if seed.x > end.x || seed.y > end.y {
          end = seed;
        }
        walls.push((start, end, direction));

        for coord in self.get_adjacents(&seed, &direction) {
          let new_tuple = (coord, direction);
          if perimeter_walls.contains(&new_tuple) {
            wall_stack.push(new_tuple);
          }
        }
      }
    }

    walls
  }
}

// Helper Functions
impl BuildingReducer {
  fn is_exposed(&self, coord: &Vec2U, wall: CubeFace) -> bool {
    match wall {
      CubeFace::North => {
        if coord.y > 0 {
          !self.coordinates.contains_key(&Vec2U::new(coord.x, coord.y - 1))
        } else {
          true
        }
      }
      CubeFace::East => !self.coordinates.contains_key(&Vec2U::new(coord.x + 1, coord.y)),
      CubeFace::South => !self.coordinates.contains_key(&Vec2U::new(coord.x, coord.y + 1)),
      CubeFace::West => {
        if coord.x > 0 {
          !self.coordinates.contains_key(&Vec2U::new(coord.x - 1, coord.y))
        } else {
          true
        }
      }
      _ => panic!("Encountered top/bottom CubeFace"),
    }
  }

  fn cardinality(&self, coord: &Vec2U) -> usize {
    CubeFace::cardinal_directions()
      .iter()
      .map(|d| if self.is_exposed(coord, *d) { 0 } else { 1 })
      .sum()
  }

  fn get_adjacents(&self, coord: &Vec2U, direction: &CubeFace) -> Vec<Vec2U> {
    let mut ret = Vec::new();
    match direction {
      &CubeFace::North => {
        ret.push(Vec2U::new(coord.x + 1, coord.y));
        if coord.x > 0 {
          ret.push(Vec2U::new(coord.x - 1, coord.y));
        }
      }
      &CubeFace::South => {
        ret.push(Vec2U::new(coord.x + 1, coord.y));
        if coord.x > 0 {
          ret.push(Vec2U::new(coord.x - 1, coord.y));
        }
      }
      &CubeFace::East => {
        ret.push(Vec2U::new(coord.x, coord.y + 1));
        if coord.y > 0 {
          ret.push(Vec2U::new(coord.x, coord.y - 1));
        }
      }
      &CubeFace::West => {
        ret.push(Vec2U::new(coord.x, coord.y + 1));
        if coord.y > 0 {
          ret.push(Vec2U::new(coord.x, coord.y - 1));
        }
      }
      _ => panic!("Encountered top/bottom CubeFace"),
    }
    ret
  }
}
