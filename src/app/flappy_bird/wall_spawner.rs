use specs::prelude::*;
use cgmath::prelude::*;

use ecs::*;
use renderer::{Drawable, Renderer, Texture};
use shapes::Sprite;

use utils::*;
use utils::random;

use physics::{TransformComponent, RigidBody};

use app::AxisAlignedCubeCollision;

struct WallSpec {
 gap_start: f32,
 gap_end: f32,
 speed: f32,
}


pub struct WallSpawner {
  time_since_spawn: f32,
  min_spawn_gap: f32,
  spawn_window_length: f32,
  gap_length: f32,
  id: Option<DrawableId>,
  material: Option<Material>,
}

impl <'a> System<'a> for WallSpawner {
  type SystemData = (
    Entities<'a>,
    Read<'a, LazyUpdate>,
    Read<'a, Timestep>,
  );

  fn run(&mut self, (entities, lazy_update, dt): Self::SystemData) {
    self.time_since_spawn += dt.0;
    if self.spawn_ready() {
      let wall_data = self.generate_wall();
      let ent1 = lazy_update.create_entity(&entities);
      let ent2 = lazy_update.create_entity(&entities);
      self.spawn_pipe(0f32, wall_data.gap_start, wall_data.speed, 1f32, ent1);
      self.spawn_pipe(wall_data.gap_end,  1f32, wall_data.speed, -1f32, ent2);
      self.time_since_spawn = 0f32;
    }
  }

  fn setup(&mut self, world: &mut World) {
    let mut renderer = world.write_resource::<Renderer>();
    let state = Sprite::new("resources/flappy_bird/pipe.png", false);
    let d_id = renderer.submit_model(state.mesh());
    self.id = Some(d_id);
    self.material = Some(state.material())
  }
}

impl WallSpawner {
  fn spawn_ready(&self) -> bool {
    if self.time_since_spawn > self.min_spawn_gap {
      let probability = (self.time_since_spawn - self.min_spawn_gap) / self.spawn_window_length;
      random::rand_choice(probability)
    } else {
      false
    }
  }

  fn generate_wall(&self) -> WallSpec {
    let top_wall_length = random::rand_float(0.05f32, 0.95f32 - self.gap_length);
    WallSpec {
      gap_start: top_wall_length,
      gap_end: top_wall_length + self.gap_length,
      speed: 5f32
    }
  }

  fn spawn_pipe<'a>(&self, start: f32, end: f32, speed: f32, invert: f32, ent: specs::world::LazyBuilder<'a>) {
    let half_height = 8.25f32;
    let scaled_start = lerp(0f32, 1f32, -half_height, half_height, start);
    let scaled_end = lerp(0f32, 1f32, -half_height, half_height, end);
    let position = Vec3F::new(-12f32, avg(scaled_start, scaled_end), 0f32);
    let scaled = Vec3F::new(1f32, (scaled_end - scaled_start) * invert, 1f32);
    let transform = TransformComponent::new(position, scaled, QuatF::zero());
    // let position = Vec3F::unit_x();
    if let Some(d_id) = &self.id {
      ent.with(
        Particle {
          lifetime: 10f32
        }
      )
      .with(RigidBody::new(Vec3F::unit_x() * speed, Vec3F::zero()))
      .with(d_id.clone())
      .with(self.material.clone().expect("Material was NONE on the wall spawner"))
      .with(AxisAlignedCubeCollision::from_transform(&transform))
      .with(transform)
      .build();
    }
  }
}

impl Default for WallSpawner {
  fn default() -> Self {
    Self {
      time_since_spawn: 1e6f32,
      min_spawn_gap: 1.8f32,
      spawn_window_length: 1f32,
      gap_length: 0.15f32,
      id: None,
      material: None,
    }
  }
}