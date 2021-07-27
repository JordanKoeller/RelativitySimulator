use cgmath::prelude::*;
use specs::prelude::*;
use std::time::Duration;

use app::flappy_bird::WallComponent;

use app::AxisAlignedCubeCollision;
use ecs::components::{DrawableId, EventReceiver, Material, Player};
use gui::{GuiInputPanel, LabeledText, LineBreak};
use physics::{CanCollide, Collision};
use renderer::{Renderer, Uniform};
use utils::{random, Mat4F, Timer, TimerLike, Timestep, Vec2F, Vec3F};

use physics::{Gravity, RigidBody, TransformComponent};

pub struct GameState {
  score: u32,
  game_over: bool,
}

impl Default for GameState {
  fn default() -> Self {
    Self {
      score: 0,
      game_over: false,
    }
  }
}

#[derive(SystemData)]
pub struct GameStateSystemData<'a> {
  player_storage: ReadStorage<'a, Player>,
  transform_storage: ReadStorage<'a, TransformComponent>,
  collide_storage: ReadStorage<'a, CanCollide>,
  wall_storage: ReadStorage<'a, WallComponent>,
  gui_storage: WriteStorage<'a, GuiInputPanel>,
  timestep: Read<'a, Timestep>
}

pub struct GameStateSystem {
  debugger: Option<Entity>,
  state: GameState,
  score_timer: Timer,
}

impl<'a> System<'a> for GameStateSystem {
  type SystemData = GameStateSystemData<'a>;

  fn run(
    &mut self,
    mut data: Self::SystemData,
  ) {
    for (_player, transform, collider) in
      (&data.player_storage, &data.transform_storage, &data.collide_storage).join()
    {
      for (wall_transform, _wall) in (
        &data.transform_storage,
        &data.wall_storage,
      )
        .join()
      {
        let mut collision_transform = wall_transform.clone();
        collision_transform.scale.y = collision_transform.scale.y.abs();
        let wall_collidable = AxisAlignedCubeCollision::from_transform(&collision_transform);
        let colliding = wall_collidable.sphere_collision(
          (&transform.translation, &collider.radius),
          &(Vec3F::unit_x() * -0.001f32),
        );
        if let Some(collision) = colliding {
          if collision.time < 160f32 {
            self.state.game_over = true;
          }
        }
      }
      if transform.translation.y < -8.25 || transform.translation.y > 8.25 {
        self.state.game_over = true;
      }
    }
    if !self.state.game_over {
      self.state.score += self.score_timer.start_poll_all(data.timestep.curr_time());
    }
    // self.score_timer.poll(time: Duration)
    self.draw_ui(&mut data.gui_storage);
  }

  fn setup(&mut self, world: &mut World) {
    let ett = world.create_entity().with(GuiInputPanel::new("Game State")).build();
    self.debugger = Some(ett);

  }
}

impl GameStateSystem {
  fn draw_ui<'a>(&self, gui_storage: &mut WriteStorage<'a, GuiInputPanel>) {
    if let Some(gui) = gui_storage.get_mut(self.debugger.unwrap()) {
      if gui.empty() {
        gui.push(Box::from(LabeledText::new("Game Status", if self.state.game_over { "GAME OVER" } else { "RUNNING" })));
        gui.push(Box::from(LabeledText::new("Score", &self.state.score.to_string())));
      } else {
        gui.lines[1] = Box::from(LabeledText::new("Game Status", if self.state.game_over { "GAME OVER" } else { "RUNNING" }));
        gui.lines[0] = Box::from(LabeledText::new("Score", &self.state.score.to_string()));
      }
    }
  }
}

impl Default for GameStateSystem {
  fn default() -> Self {
    Self {
      debugger: None,
      state: GameState::default(),
      score_timer: Timer::new(Duration::from_millis(3)),
    }
  }
}
