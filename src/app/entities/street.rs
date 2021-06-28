use specs::prelude::{ReadStorage, Entity, Write};
use ecs::{EntityDelegate, MyBuilder};
use renderer::{DrawableId, Material, Texture, Drawable, Renderer};
use utils::*;
use physics::TransformComponent;
use app::{Cube, FaceCube, AxisAlignedCubeCollision};

#[derive(Clone, Debug)]
pub enum StreetPiece {
  Straightaway,
  Tee,
  Intersection,
  Turn,
}

#[derive(Clone, Debug)]
pub struct StreetState {
  position: Vec3F,
  footprint: Vec2F,
  rotation: DegF,
  piece: StreetPiece,
}

impl Default for StreetState {
  fn default() -> Self {
    Self {
      position: Vec3F::new(0f32, 0f32, 0f32),
      footprint: Vec2F::new(1f32, 1f32),
      rotation: cgmath::Deg(0f32),
      piece: StreetPiece::Straightaway,
    }
  }
}

impl StreetState {
  pub fn new(position: Vec3F, footprint: Vec2F, rotation: DegF, piece: StreetPiece) -> Self {
    Self {
      position,
      footprint,
      rotation,
      piece,
    }
  }
}

type StreetStateData<'a> = ();

#[derive(Default)]
pub struct StreetDelegate;

impl<'a> EntityDelegate<'a> for StreetDelegate {
  type State = StreetState;
  type EntityResources = StreetStateData<'a>;

  fn create<'b, F: Fn() -> MyBuilder<'a, 'b>>(
    &self,
    state: &Self::State,
    _resources: &mut Self::EntityResources,
    constructor: F,
  ) -> Vec<Entity> {
    let mut stack = TransformStack::default();
    stack.push_euler(state.rotation, Vec3F::unit_y());
    stack.push_nonunif_scale(Vec3F::new(state.footprint.x, 1f32, state.footprint.y));
    stack.push_translate(state.position);
    let mut material = Material::new();
    let texture = match state.piece {
      StreetPiece::Straightaway => Texture::from_file("resources/textures/roads/straightaway-1.jpg"),
      StreetPiece::Intersection => Texture::from_file("resources/textures/roads/intersection-1.jpg"),
      StreetPiece::Tee => Texture::from_file("resources/textures/roads/tee.jpg"),
      StreetPiece::Turn => Texture::from_file("resources/textures/roads/bend-1.jpg")
    };
    material.diffuse_texture(texture);
    let model = Cube::new(material).state();
    let builder = constructor();
    let matrix = stack.pop();
    let transform = TransformComponent::from(matrix);
    let ent = builder.with(model).with(AxisAlignedCubeCollision::from_transform(&transform)).with(transform).build();
    vec![ent]
  }

}