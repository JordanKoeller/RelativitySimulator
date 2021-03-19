use specs::prelude::{ReadStorage, Entity, Write};
use ecs::components::Transform;
use ecs::{EntityDelegate, MyBuilder};
use renderer::{DrawableId, Material, Texture, Drawable, Renderer};
use utils::*;

use app::{Cube, FaceCube};

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

type StreetStateData<'a> = (ReadStorage<'a, DrawableId>, ReadStorage<'a, Transform>);

#[derive(Default)]
pub struct StreetDelegate;

impl<'a> EntityDelegate<'a> for StreetDelegate {
  type State = StreetState;
  type EntityResources = StreetStateData<'a>;

  fn create<'b, F: Fn() -> MyBuilder<'a, 'b>>(
    &self,
    state: Self::State,
    resources: &mut Self::EntityResources,
    constructor: F,
  ) -> Vec<Entity> {
    let scaler = nonunif_scale(Vec3F::new(state.footprint.x, 1f32, state.footprint.y));
    let translate_matrix = translate(state.position);
    let rotation_matrix = Mat4F::from_angle_y(state.rotation);
    let transform = Transform(translate_matrix * scaler * rotation_matrix);
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
    let ent = builder.with(model).with(transform).build();
    vec![ent]
  }

}