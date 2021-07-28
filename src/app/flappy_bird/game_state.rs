use cgmath::prelude::*;
use specs::prelude::*;
use std::time::Duration;

use app::flappy_bird::WallComponent;

use app::AxisAlignedCubeCollision;
use ecs::components::{DrawableId, EventReceiver, Material, Player};
use gui::{Button, GuiInputPanel, LabeledText, LineBreak, InputColor};
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
  entities: Entities<'a>,
  updater: Read<'a, LazyUpdate>,
  player_storage: ReadStorage<'a, Player>,
  transform_storage: ReadStorage<'a, TransformComponent>,
  collide_storage: ReadStorage<'a, CanCollide>,
  wall_storage: ReadStorage<'a, WallComponent>,
  gui_storage: WriteStorage<'a, GuiInputPanel>,
  timestep: Read<'a, Timestep>,
}

pub struct GameStateSystem {
  debugger: Option<Entity>,
  state: GameState,
  score_timer: Timer,
}

impl<'a> System<'a> for GameStateSystem {
  type SystemData = GameStateSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    if !self.state.game_over {
      let game_over = self.update(&mut data);
      if game_over {
        self.end_game(&mut data);
      } else {
        self.draw_ui(&mut data.gui_storage);
        self.state.score += self.score_timer.start_poll_all(data.timestep.curr_time());
      }
    } else {
      if let Some(gui) = data.gui_storage.get_mut(self.debugger.unwrap()) {
        // if gui.lines[1].get_bool() {
        //   println!("They clicked the button! Need to restart!!!");
        // }
      }
    }
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
        gui.push(Box::from(LabeledText::new(
          "Game Status",
          if self.state.game_over { "GAME OVER" } else { "RUNNING" },
        )));
        gui.push(Box::from(LabeledText::new("Score", &self.state.score.to_string())));
      } else {
        gui.lines[1] = Box::from(LabeledText::new(
          "Game Status",
          if self.state.game_over { "GAME OVER" } else { "RUNNING" },
        ));
        gui.lines[0] = Box::from(LabeledText::new("Score", &self.state.score.to_string()));
      }
    }
  }

  /*
  Runs one frame of the system. Returns a tuple of T/F the game is still running and the new score
  */
  fn update(&self, data: &mut <Self as System>::SystemData) -> bool {
    let mut game_over = self.state.game_over;
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
            game_over = true;
          }
        }
      }
      if transform.translation.y < -8.25 || transform.translation.y > 8.25 {
        game_over = true;
      }
    }
    game_over
  }

  fn end_game(&mut self, data: &mut <Self as System>::SystemData) {
    for (ent, _wall) in (&data.entities, &data.wall_storage).join() {
      data.updater.insert(ent, Gravity);
    }
    let mut panel = GuiInputPanel::new("GAME OVER");
    panel.push(Box::from(LabeledText::new("Max Score:", &self.state.score.to_string())));
    panel.push(Box::from(InputColor::new("Color Picker")));
    // panel.push(Box::from(Button::new("Try Again?")));
    data.updater.insert(self.debugger.expect("No debugger? WAAAT"), panel);
    let mut transform = TransformComponent::default();
    transform.push_translation(Vec3F::new(0f32, 0f32, 0f32));
    data
      .updater
      .insert(self.debugger.expect("No debugger? WAAAT"), transform);
    self.state.game_over = true;
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
