use cgmath::prelude::*;
use specs::prelude::*;

use app::AxisAlignedCubeCollision;
use ecs::components::{EventReceiver, Player};
use ecs::CanCollide;
use ecs::Collision;
use gui::{GuiInputPanel, LabeledText, LineBreak};
use renderer::{DrawableId, Renderer, Uniform};
use utils::{random, Mat4F, Timestep, Vec2F, Vec3F};

use physics::{Gravity, RigidBody, TransformComponent};

pub struct GameState {
  debugger: Option<Entity>,
}

impl<'a> System<'a> for GameState {
  type SystemData = (
    ReadStorage<'a, Player>,
    ReadStorage<'a, RigidBody>,
    ReadStorage<'a, CanCollide>,
    ReadStorage<'a, DrawableId>,
    ReadStorage<'a, TransformComponent>,
    WriteStorage<'a, GuiInputPanel>,
    ReadStorage<'a, AxisAlignedCubeCollision>,
    Write<'a, Renderer>,
  );

  fn run(
    &mut self,
    (
      player_storage,
      rigid_storage,
      collide_storage,
      drawable_storage,
      transform_storage,
      mut gui_storage,
      aacc_storage,
      mut renderer,
    ): Self::SystemData,
  ) {
    let mut is_colliding = false;
    for (_player, rigid_body, transform, collider) in
      (&player_storage, &rigid_storage, &transform_storage, &collide_storage).join()
    {
      for (_wall_collision, wall_transform, d_id) in (&aacc_storage, &transform_storage, &drawable_storage).join() {
        let mut collision_transform = wall_transform.clone();
        collision_transform.scale.y = collision_transform.scale.y.abs();
        let wall_collidable = AxisAlignedCubeCollision::from_transform(&collision_transform);
        let colliding = wall_collidable.sphere_collision(
          (&transform.translation, &collider.radius),
          &(Vec3F::unit_x() * -0.001f32),
        );
        // let dist = wall_collidable.distance_to(&position.0);
        if let Some(collision) = colliding {
          if collision.time < 160f32 {
            renderer.submit_uniform(d_id, "ambient", Uniform::Vec3(Vec3F::new(1f32, 0.5f32, 0.5f32)));
            break;
          }
        }
      }
      if transform.translation.y < -8.25 || transform.translation.y > 8.25 {
        is_colliding = true;
      }
    }
    self.set_colliding(&mut gui_storage, is_colliding);
  }

  fn setup(&mut self, world: &mut World) {
    let ett = world.create_entity().with(GuiInputPanel::new("Game State")).build();
    self.debugger = Some(ett);
  }
}

impl GameState {
  fn set_colliding<'a>(
    &self,
    gui_storage: &mut WriteStorage<'a, GuiInputPanel>,
    is_colliding: bool,
  ) {
    if let Some(gui) = gui_storage.get_mut(self.debugger.unwrap()) {
      if gui.empty() {
        gui.push(Box::from(LabeledText::new(
          "Collision Status",
          if is_colliding { "COLLIDING" } else { "NOT COLLIDING" },
        )));
        // gui.push(Box::from(LabeledText::new("DISTANCE", &format!("{}", dist))));
      } else {
        gui.lines[0] = Box::from(LabeledText::new(
          "Collision Status",
          if is_colliding { "COLLIDING" } else { "NOT COLLIDING" },
        ));
        // gui.lines[1] = Box::from(LabeledText::new("DISTANCE", &format!("{}", dist)));
      }
    }
  }
}

impl Default for GameState {
  fn default() -> Self {
    Self { debugger: None }
  }
}
