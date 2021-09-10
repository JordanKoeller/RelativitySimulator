use cgmath::prelude::*;
use specs::prelude::*;
use specs::{Component, NullStorage};
use std::time::Duration;

use ecs::*;
use renderer::{Drawable, Renderer, Texture};
use shapes::{Sprite, Block};

use utils::random;
use utils::*;

use physics::{RigidBody, TransformComponent, Gravity};

use app::AxisAlignedCubeCollision;
use app::flappy_bird::{GameState, GameStateEnum};

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct WallComponent;

struct WallSpec {
  gap_start: f32,
  gap_end: f32,
  speed: f32,
}

pub struct WallSpawner {
  spawn_timer: WindowedTimer,
  gap_length: f32,
  id: Option<DrawableId>,
  material: Option<Material>,
}

impl<'a> System<'a> for WallSpawner {
  type SystemData = (
    Entities<'a>,
    Read<'a, LazyUpdate>,
    Read<'a, Timestep>,
    Read<'a, GameState>
  );

  fn run(&mut self, (entities, lazy_update, dt, game_state): Self::SystemData) {
    if self.spawn_timer.start_poll(dt.curr_time()) {
      let wall_data = self.generate_wall();
      let ent1 = lazy_update.create_entity(&entities);
      let ent2 = lazy_update.create_entity(&entities);
      match game_state.state {
        GameStateEnum::Playing => {
          self.spawn_pipe(0f32, wall_data.gap_start, wall_data.speed, 1f32, ent1, &game_state.state);
          self.spawn_pipe(wall_data.gap_end, 1f32, wall_data.speed, -1f32, ent2, &game_state.state);
        },
        _ => {}
      }
    }
  }

  fn setup(&mut self, world: &mut World) {
    world.register::<WallComponent>();
    let mut renderer = world.write_resource::<Renderer>();
    let state = Block::new("resources/flappy_bird/pipe.png");
    let d_id = renderer.submit_model(state.mesh());
    self.id = Some(d_id);
    self.material = Some(state.material());
  }
}

impl WallSpawner {
  fn generate_wall(&self) -> WallSpec {
    let top_wall_length = random::rand_float(0.05f32, 0.95f32 - self.gap_length);
    WallSpec {
      gap_start: top_wall_length,
      gap_end: top_wall_length + self.gap_length,
      speed: 5f32,
    }
  }

  fn spawn_pipe<'a>(&self, start: f32, end: f32, speed: f32, invert: f32, ent: specs::world::LazyBuilder<'a>, running_state: &GameStateEnum) {
    let half_height = 8.25f32;
    let scaled_start = lerp(0f32, 1f32, -half_height, half_height, start);
    let scaled_end = lerp(0f32, 1f32, -half_height, half_height, end);
    let position = Vec3F::new(-12f32, avg(scaled_start, scaled_end), 0f32);
    let scaled = Vec3F::new(1f32, (scaled_end - scaled_start) * invert, 1f32);
    let transform = TransformComponent::new(position, scaled, QuatF::zero());
    // let position = Vec3F::unit_x();
    if let Some(d_id) = &self.id {
      let b = ent
        .with(Particle {
          lifetime: Duration::from_secs(10),
        })
        .with(RigidBody::new(Vec3F::unit_x() * speed, Vec3F::zero()))
        .with(d_id.clone())
        .with(self.material.clone().expect("Material was NONE on the wall spawner"))
        .with(AxisAlignedCubeCollision::from_transform(&transform))
        .with(transform)
        .with(WallComponent);
      let b = match running_state {
        GameStateEnum::GameOver => {b.with(Gravity)},
        GameStateEnum::Playing => {b}
      };
      b.build();
    }
  }
}

impl Default for WallSpawner {
  fn default() -> Self {
    Self {
      spawn_timer: WindowedTimer::new(Duration::from_millis(1600), Duration::from_millis(4200)),
      gap_length: 0.15f32,
      id: None,
      material: None,
    }
  }
}
