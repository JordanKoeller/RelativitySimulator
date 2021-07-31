use cgmath::prelude::*;
use specs::prelude::*;
use std::time::Duration;

use app::flappy_bird::WallComponent;

use app::AxisAlignedCubeCollision;
use ecs::components::{DrawableId, EventReceiver, Material, Player};
use ecs::SystemDelegate;
use gui::{Button, DebugPanel, GuiInputPanel, InputColor, InputFloat, LabeledText, LineBreak};
use physics::{CanCollide, Collision};
use renderer::{Renderer, Uniform};
use utils::{random, Mat4F, QuatF, Timer, TimerLike, Timestep, Vec2F, Vec3F};

use physics::{Gravity, RigidBody, TransformComponent};

pub enum GameStateEnum {
  Playing,
  GameOver,
}

impl Default for GameStateEnum {
  fn default() -> Self {
    Self::Playing
  }
}

pub struct GameState {
  pub score: u32,
  pub state: GameStateEnum,
}

impl Default for GameState {
  fn default() -> Self {
    Self {
      score: 0,
      state: GameStateEnum::default(),
    }
  }
}

#[derive(SystemData)]
pub struct GameStateSystemData<'a> {
  entities: Entities<'a>,
  updater: Read<'a, LazyUpdate>,
  player_storage: ReadStorage<'a, Player>,
  transform_storage: ReadStorage<'a, TransformComponent>,
  collide_storage: ReadStorage<'a, CanCollide>,
  wall_storage: ReadStorage<'a, WallComponent>,
  timestep: Read<'a, Timestep>,
  game_state: Write<'a, GameState>,
}

pub struct GameStateSystem {
  debugger: Option<Entity>,
  score_timer: Timer,
}

impl<'a> SystemDelegate<'a> for GameStateSystem {
  type SystemData = GameStateSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    match data.game_state.state {
      GameStateEnum::Playing => {
        let game_over = self.update(&mut data);
        if game_over {
          self.end_game(&mut data);
        } else {
          data.game_state.score = data.game_state.score + self.score_timer.start_poll_all(data.timestep.curr_time());
        }
      }
      _ => {}
    }
  }

  fn setup_debug_panel(&mut self, _world: &mut World) -> Option<DebugPanel> {
    let mut gui = DebugPanel::new("Game State");
    gui.panel.push(Box::from(LabeledText::new("Score", "")));
    Some(gui)
  }

  fn update_debugger(&mut self, data: &mut Self::SystemData, debugger: &mut DebugPanel) {
    if let Some(button) = debugger.panel.lines.get(1) {
      if button.get_bool() {
        data.game_state.score = 0;
        data.game_state.state = GameStateEnum::Playing;
        for (ent, _player) in (&data.entities, &data.player_storage).join() {
          let pos = Vec3F::unit_x() * 4f32;
          let mut tc = TransformComponent::new(pos, Vec3F::new(1f32, 1f32, 1f32), QuatF::zero());
          tc.rotation = Vec3F::unit_y() * 90f32;
          data.updater.insert(ent, tc);
          data.updater.insert(ent, RigidBody::new_stationary());
          debugger.panel.lines.pop();
        }
      }
    }
    match (*data.game_state).state {
      GameStateEnum::Playing => {
        debugger.panel.lines[0] = Box::from(LabeledText::new("Score", &data.game_state.score.to_string()));
      }
      GameStateEnum::GameOver => {
        debugger.panel.lines[0] = Box::from(LabeledText::new("Max Score:", &data.game_state.score.to_string()));
        if debugger.panel.lines.len() == 1 {
          debugger.panel.push(Box::from(Button::new("Try Again?")));
        }
        debugger.panel.lines[1] = Box::from(Button::new("Try Again?"));
      }
    }
  }

  fn setup(&mut self, world: &mut World) {
    world.insert::<GameState>(GameState::default());
  }
}

impl GameStateSystem {
  /*
  Runs one frame of the system. Returns a tuple of T/F the game is still running and the new score
  */
  fn update(&self, data: &mut <Self as SystemDelegate>::SystemData) -> bool {
    for (_player, transform, collider) in (&data.player_storage, &data.transform_storage, &data.collide_storage).join()
    {
      for (wall_transform, _wall) in (&data.transform_storage, &data.wall_storage).join() {
        let mut collision_transform = wall_transform.clone();
        collision_transform.scale.y = collision_transform.scale.y.abs();
        let wall_collidable = AxisAlignedCubeCollision::from_transform(&collision_transform);
        let colliding = wall_collidable.sphere_collision(
          (&transform.translation, &collider.radius),
          &(Vec3F::unit_x() * -0.001f32),
        );
        if let Some(collision) = colliding {
          if collision.time < 160f32 {
            data.game_state.state = GameStateEnum::GameOver;
          }
        }
      }
      if transform.translation.y < -8.25 || transform.translation.y > 8.25 {
        data.game_state.state = GameStateEnum::GameOver;
      }
    }
    match data.game_state.state {
      GameStateEnum::GameOver => true,
      GameStateEnum::Playing => false,
    }
  }

  fn end_game(&mut self, data: &mut <Self as SystemDelegate>::SystemData) {
    for (ent, _wall) in (&data.entities, &data.wall_storage).join() {
      data.updater.insert(ent, Gravity);
    }
    data.game_state.state = GameStateEnum::GameOver;
  }
}

impl Default for GameStateSystem {
  fn default() -> Self {
    Self {
      debugger: None,
      score_timer: Timer::new(Duration::from_millis(3)),
    }
  }
}
