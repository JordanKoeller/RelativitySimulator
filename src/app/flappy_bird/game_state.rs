use cgmath::prelude::*;
use specs::prelude::*;

use ecs::components::{EventReceiver, Gravity, Kinetics, Player, Position, Rotation, Transform};
use ecs::CanCollide;
use ecs::Collision;
use gui::{GuiInputPanel, LabeledText, LineBreak};
use app::AxisAlignedCubeCollision;
use utils::{Vec2F, Vec3F, Timestep, Mat4F, random};

pub struct GameState {
  debugger: Option<Entity>,
}

impl <'a> System<'a> for GameState {
  type SystemData = (
    ReadStorage<'a, Player>,
    ReadStorage<'a, Kinetics>,
    ReadStorage<'a, CanCollide>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Transform>,
    WriteStorage<'a, GuiInputPanel>,
    ReadStorage<'a, AxisAlignedCubeCollision>
  );

  fn run(&mut self, (player_storage, kin_storage, collide_storage, pos_storage, transform_storage, mut gui_storage, aacc_storage): Self::SystemData) {
    let mut is_colliding = false;
    for (_player, kinetics, position, collider) in (&player_storage, &kin_storage, &pos_storage, &collide_storage).join() {
      for (_wall_collision, wall_transform) in (&aacc_storage, &transform_storage).join() {
        let wall_collidable = AxisAlignedCubeCollision::from_transform(wall_transform);
        let colliding = wall_collidable.sphere_collision((&position.0, &collider.radius), &kinetics.velocity);
        // let dist = wall_collidable.distance_to(&position.0);
        if colliding.is_some() {
          is_colliding = true;
          break;
        }
      }
      if position.0.y < -8.25 || position.0.y > 8.25 {
        is_colliding = true;
      }
    }
    self.set_colliding(&mut gui_storage, is_colliding);
  }

  fn setup(&mut self, world: &mut World) {
    let ett = world.create_entity()
      .with(GuiInputPanel::new("Game State"))
      .build();
    self.debugger = Some(ett);
  }
}

impl GameState {
  fn set_colliding<'a>(&self, gui_storage: &mut WriteStorage<'a, GuiInputPanel>, is_colliding: bool) {
    if let Some(gui) = gui_storage.get_mut(self.debugger.unwrap()) {
      if gui.empty() {
        gui.push(Box::from(LabeledText::new("Collision Status", if is_colliding {"COLLIDING"} else {"NOT COLLIDING"})));
        // gui.push(Box::from(LabeledText::new("DISTANCE", &format!("{}", dist))));
      } else {
        gui.lines[0] = Box::from(LabeledText::new("Collision Status", if is_colliding {"COLLIDING"} else {"NOT COLLIDING"}));
        // gui.lines[1] = Box::from(LabeledText::new("DISTANCE", &format!("{}", dist)));
      }
    }
  }
}

impl Default for GameState {
  fn default() -> Self {
    Self {
      debugger: None,
    }
  }
}