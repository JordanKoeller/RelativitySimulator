use cgmath::prelude::*;
use rand::{thread_rng, Rng};
use specs::prelude::*;

use ecs::*;
use renderer::{Drawable, DrawableId, DrawableState, Material, Renderer, Texture};
use shapes::Sprite;
use utils::*;

use app::{AxisAlignedCubeCollision, Cube};


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

type PlayerTailStateData<'a> = ReadStorage<'a, DrawableId>;

#[derive(Default, Debug)]
pub struct PlayerTailDelegate {
  id: Option<DrawableId>,
}
impl<'a> EntityDelegate<'a> for PlayerTailDelegate {
  type State = PlayerTailParticleState;
  type EntityResources = PlayerTailStateData<'a>;

  fn create<'b, F: Fn() -> MyBuilder<'a, 'b>>(
    &self,
    state: &Self::State,
    _resources: &mut Self::EntityResources,
    constructor: F,
  ) -> Vec<Entity> {
    if let Some(d_id) = &self.id {
      vec![constructor()
        .with(Position(state.position))
        .with(Particle {
          lifetime: state.lifetime,
        })
        .with(Kinetics {
          velocity: state.impulse,
          acceleration: -Vec3F::unit_y(),
        })
        .with(d_id.clone())
        .with(Gravity)
        .with(Transform(
          Mat4F::from_translation(state.position) * Mat4F::from_scale(0.5f32),
        ))
        .build()]
    } else {
      panic!("Tried to make a sprite but it hasn't been pre-initialized!");
    }
  }

  fn setup_delegate(&mut self, world: &mut World) {
    let mut renderer = world.write_resource::<Renderer>();
    let mut state = Sprite::new("resources/flappy_bird/spark.png").state();
    state.refresh();
    let id = renderer.submit_model(state);
    println!("Registered! {:?}", id);
    self.id = Some(id);
  }
}
