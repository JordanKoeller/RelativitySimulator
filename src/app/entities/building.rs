use specs::prelude::*;

use ecs::*;
use renderer::{Drawable, DrawableId, Material, Renderer, Texture};
use utils::*;

use app::Cube;

#[derive(Clone, Debug)]
pub struct BuildingState {
  bounding_box: Vec3F,
  position: Vec3F,
  ratio: f32,
}

impl Default for BuildingState {
  fn default() -> Self {
    Self {
      position: Vec3F::unit_x(),
      bounding_box: Vec3F::unit_x(),
      ratio: 0.5,
    }
  }
}

impl BuildingState {
  pub fn new(position: Vec3F, bounding_box: Vec3F, ratio: f32) -> Self {
    Self {
      position,
      bounding_box,
      ratio,
    }
  }
}

type BuildingStateData<'a> = (ReadStorage<'a, DrawableId>, Write<'a, Renderer>);

#[derive(Default, Debug)]
pub struct BuildingDelegate;
impl<'a> EntityDelegate<'a> for BuildingDelegate {
  type State = BuildingState;
  type EntityResources = BuildingStateData<'a>;

  fn create<'b, F: Fn() -> MyBuilder<'a, 'b>>(
    &self,
    state: Self::State,
    resources: &mut Self::EntityResources,
    constructor: F,
  ) -> Vec<Entity> {
    // Entity 1

    // TODO: Set up some type of transform stack for hierarchical transforms
    let scale_low_block = Mat4F::from_nonuniform_scale(
      state.bounding_box.x,
      state.bounding_box.y * state.ratio,
      state.bounding_box.z,
    );
    let big_scale = state.ratio + (1f32 - state.ratio) / 2f32;
    let scale_high_block = Mat4F::from_nonuniform_scale(
      state.bounding_box.x * big_scale,
      state.bounding_box.y * (1f32 - state.ratio),
      state.bounding_box.z * big_scale,
    );
    let translate1 = translate(state.position);
    let transform1 = Transform(translate1 * scale_low_block);
    let translate2 = translate(Vec3F::new(
      state.position.x,
      state.position.y + state.bounding_box.y * state.ratio,
      state.position.z,
    ));
    let transform2 = Transform(translate2 * scale_high_block);

    let texture = Texture::from_file(&format!("resources/textures/building/building-{}.jpg", 1));

    // Entity 1
    let builder = constructor();
    let mut material = Material::new();
    material.diffuse_texture(texture.clone());
    let model = Cube::new(material).state();
    let id = resources.1.submit_model(model);
    let ent1 = builder.with(id).with(transform1).build();

    // Entity 2
    let builder = constructor();
    let mut material = Material::new();
    material.diffuse_texture(texture.clone());
    let model = Cube::new(material).state();
    let id = resources.1.submit_model(model);
    let ent2 = builder.with(id).with(transform2).build();
    vec![ent1, ent2]
  }
}
