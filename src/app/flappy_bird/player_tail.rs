use cgmath::prelude::*;
use rand::{thread_rng, Rng};
use specs::prelude::*;

use ecs::*;
use renderer::{Drawable, Mesh, Renderer, Texture};
use shapes::Sprite;
use utils::*;

use app::{AxisAlignedCubeCollision, Cube};
use physics::{TransformComponent, Gravity, RigidBody};

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
  material: Option<Material>,
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
        .with(TransformComponent::new(state.position, Vec3F::new(0.5f32, 0.5f32, 0.5f32), QuatF::zero()))
        // .with(Position(state.position))
        .with(Particle {
          lifetime: state.lifetime,
        })
        .with(RigidBody::new(state.impulse, -Vec3F::unit_y()))
        .with(d_id.clone())
        .with(self.material.clone().expect("Player Tail Material was NONE"))
        .with(Gravity)
        .build()]
    } else {
      panic!("Tried to make a sprite but it hasn't been pre-initialized!");
    }
  }

  fn setup_delegate(&mut self, world: &mut World) {
    let mut renderer = world.write_resource::<Renderer>();
    let state = Sprite::new("resources/flappy_bird/spark.png", true);
    let d_id = renderer.submit_model(state.mesh());
    println!("Registered player tail! {:?}", d_id);
    self.id = Some(d_id);
    self.material = Some(state.material())
  }
}
