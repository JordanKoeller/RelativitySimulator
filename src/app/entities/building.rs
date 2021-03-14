use specs::prelude::*;

use ecs::*;
use renderer::{Drawable, DrawableId, Material, Renderer, Texture};
use utils::*;

use app::Cube;

#[derive(Clone, Debug)]
struct BuildingState {
  bounding_box: Vec3F,
  position: Vec3F,
  ratio: f32,
  textures: (Texture, Texture),
}

impl Default for BuildingState {
  fn default() -> Self {
    Self {
      position: Vec3F::unit_x(),
      bounding_box: Vec3F::unit_x(),
      ratio: 0.5,
      textures: (Texture::pre_made(1, 200, 200), Texture::pre_made(2, 200, 200)),
    }
  }
}

type BuildingStateData<'a> = (ReadStorage<'a, DrawableId>, Write<'a, Renderer>);

struct BuildingDelegate;
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
      state.bounding_box.y,
      state.bounding_box.z * state.ratio,
    );
    let big_scale = state.ratio + (1f32 - state.ratio) / 2f32;
    let scale_high_block = Mat4F::from_nonuniform_scale(
      state.bounding_box.x * big_scale,
      state.bounding_box.y * big_scale,
      state.bounding_box.z * (1f32 - state.ratio),
    );
    let translate1 = translate(state.position);
    let transform1 = Transform(translate1 * scale_low_block);
    let translate2 = translate(Vec3F::new(
      state.position.x,
      state.position.y + state.bounding_box.z * state.ratio,
      state.position.z,
    ));
    let transform2 = Transform(translate2 * scale_high_block);

    // Entity 1
    let builder = constructor();
    let mut material = Material::new();
    material.ambient_texture(state.textures.0);
    let model = Cube::new(material).state();
    let id = resources.1.submit_model(model);
    let ent1 = builder.with(id).with(transform1).build();

    // Entity 2
    let builder = constructor();
    let mut material = Material::new();
    material.ambient_texture(state.textures.1);
    let model = Cube::new(material).state();
    let id = resources.1.submit_model(model);
    let ent2 = builder.with(id).with(transform2).build();
    vec![ent1, ent2]
  }

  fn update_entity(&self, entity: Entity, state: Self::State) {}
}
