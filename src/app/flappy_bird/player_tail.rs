use rand::{thread_rng, Rng};
use specs::prelude::*;
use cgmath::prelude::*;

use ecs::*;
use renderer::{Drawable, DrawableId, Material, Renderer, Texture};
use utils::*;

use app::{Cube, AxisAlignedCubeCollision};


#[derive(Clone, Debug)]
pub struct PlayerTailParticleState {
  position: Vec3F,
  lifetime: f32,
  impulse: Vec3F,
}

impl Default for PlayerTailParticleState {
  fn default() -> Self {
    Self {
      position: Vec3F::unit_x(),
      lifetime: 1500f32,
      impulse: -Vec3F::unit_x(),
    }
  }
}

impl PlayerTailParticleState {
  pub fn new(position: Vec3F, lifetime: f32, impulse: Vec3F) -> Self {
    Self {
      position,
      lifetime,
      impulse,
    }
  }
}

type PlayerTailStateData<'a> = ();

#[derive(Default, Debug)]
pub struct PlayerTailDelegate;
impl<'a> EntityDelegate<'a> for PlayerTailDelegate {
  type State = PlayerTailParticleState;
  type EntityResources = PlayerTailStateData<'a>;

  fn create<'b, F: Fn() -> MyBuilder<'a, 'b>>(
    &self,
    _state: &Self::State,
    _resources: &mut Self::EntityResources,
    _constructor: F,
  ) -> Vec<Entity> {
    vec![]
  }
}