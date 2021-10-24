use rand::{thread_rng, Rng};
use specs::prelude::*;
use cgmath::prelude::*;

use ecs::*;
use renderer::{Drawable, DrawableId, Renderer, Texture};
use utils::*;

use physics::TransformComponent;

use app::{Cube, AxisAlignedCubeCollision};

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

type BuildingStateData<'a> = ();

#[derive(Default, Debug)]
pub struct BuildingDelegate;
impl<'a> PrefabBuilder<'a> for BuildingDelegate {
  type State = BuildingState;
  type EntityResources = BuildingStateData<'a>;

  fn create<'b, F: Fn() -> MyBuilder<'a, 'b>>(
    &self,
    state: &Self::State,
    _resources: &mut Self::EntityResources,
    constructor: F,
  ) -> Vec<Entity> {
    // Entity 1

    // TODO: Set up some type of transform stack for hierarchical transforms
    let mut stack = TransformStack::default();
    stack.push_nonunif_scale(state.bounding_box.mul_element_wise(Vec3F::new(1f32,  state.ratio, 1f32)));
    stack.push_translate(state.position);
    let transform1 = stack.pop();
    stack.clear();
    let top_scale = state.ratio + (1f32 - state.ratio) / 2f32;
    stack.push_nonunif_scale(state.bounding_box.mul_element_wise(Vec3F::new(top_scale, 1f32 - state.ratio, top_scale)));
    stack.push_translate(state.position + Vec3F::unit_y() * state.bounding_box.y * state.ratio);
    let transform2 = stack.pop();

    let texture = Texture::from_file(&format!("resources/textures/building/building-{}.jpg", thread_rng().gen_range(1,6)));

    // Entity 1
    let builder = constructor();
    let mut material = Material::new();
    material.diffuse_texture(texture.clone());
    let model = Cube::new(material);
    let transform1 = TransformComponent::from(transform1);
    let collider = AxisAlignedCubeCollision::from_transform(&transform1);
    let ent1 = builder.with(transform1).with(collider).with_drawable(&model).build();

    // Entity 2
    let builder = constructor();
    let transform2 = TransformComponent::from(transform2);
    let collider = AxisAlignedCubeCollision::from_transform(&transform2);
    let ent2 = builder.with_drawable(&model).with(transform2).with(collider).build();
    vec![ent1, ent2]
  }
}
