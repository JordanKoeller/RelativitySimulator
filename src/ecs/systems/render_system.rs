use specs::{Builder, Join, Component, ReadStorage, Read, RunNow, System, VecStorage, World, WorldExt, WriteStorage, Write};

use utils::{Vec2F, Vec3F, Mat4F, Timestep};
use renderer::{Renderer, Camera, Window};
use ecs::components::{DrawableMemo, Transform, Player, Position, Kinetics, Rotation};
use events::{EventChannel, WindowEvent};
pub struct RenderSystem;

impl <'a> System<'a> for RenderSystem {
  type SystemData = (
    ReadStorage<'a, DrawableMemo>,
    ReadStorage<'a, Transform>,
    Write<'a, Renderer>
  );

  fn run(&mut self, (drawables, transforms, mut renderer): Self::SystemData) {
    for (drawable, maybe_transform) in (&drawables, (&transforms).maybe()).join() {
      if let Some(transform) = maybe_transform {
        let mut c = drawable.clone();
        c.with_transform(transform.0);
        renderer.submit(c);
      } else {
        renderer.submit(drawable.clone());

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
    for (player, pos, kinetics, rotation) in (&s_player, &s_position, &s_kinetics, &s_rotation).join() {
      let cam = Camera::new(&pos.0, &kinetics.velocity, &rotation);
      renderer.start_scene(cam, timestep.0);
    }
  }
}