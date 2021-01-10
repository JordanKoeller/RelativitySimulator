use specs::prelude::*;
use cgmath::prelude::*;

use utils::{Timestep, Mat4F};
use renderer::{Renderer, Camera, DrawableId, DrawCommand};
use ecs::components::{Transform, Player, Position, Kinetics, Rotation};
pub struct RenderSystem;

impl <'a> System<'a> for RenderSystem {
  type SystemData = (
    ReadStorage<'a, DrawableId>,
    ReadStorage<'a, Transform>,
    Write<'a, Renderer>
  );

  fn run(&mut self, (drawables, transforms, mut renderer): Self::SystemData) {
    for (drawable, maybe_transform) in (&drawables, (&transforms).maybe()).join() {
      if let Some(transform) = maybe_transform {
        renderer.submit(DrawCommand {
          id: drawable.clone(),
          transform: transform.clone()
        })
      } else {
        renderer.submit(DrawCommand {
          id: drawable.clone(),
          transform: Transform(Mat4F::one())
        })

      }
    }
  }
}

pub struct StartFrameSystem;

impl <'a> System<'a> for StartFrameSystem {
  type SystemData = (
    ReadStorage<'a, Player>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Kinetics>,
    ReadStorage<'a, Rotation>,
    Write<'a, Renderer>,
    Read<'a, Timestep>,
  );

  fn run(&mut self, (s_player, s_position, s_kinetics, s_rotation, mut renderer, timestep): Self::SystemData) {
    for (_player, pos, kinetics, rotation) in (&s_player, &s_position, &s_kinetics, &s_rotation).join() {
      let cam = Camera::new(&pos.0, &kinetics.velocity, &rotation);
      renderer.start_scene(cam, timestep.0);
    }
  }
}