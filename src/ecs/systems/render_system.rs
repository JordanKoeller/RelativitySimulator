use specs::prelude::*;
use cgmath::prelude::*;

use utils::{Timestep, Mat4F, MutRef, Running};
use renderer::{Renderer, DrawableId, DrawCommand, Window, Camera};
use ecs::components::{Transform, Player, Position, Kinetics, Rotation};
use events::{EventChannel, WindowEvent, WindowEventChannel, ReceiverID, Event, KeyCode};

pub struct RenderSystem {
  pub window: MutRef<Window>,
}

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
        });
      } else {
        renderer.submit(DrawCommand {
          id: drawable.clone(),
          transform: Transform(Mat4F::one())
        });
      }
    }
    let mut window = self.window.borrow_mut();
    renderer.draw_scene(&mut window);
    renderer.end_frame(&mut window);
  }
}

pub struct StartFrameSystem {
  pub window: MutRef<Window>,
  pub last_time: f32,
}

impl <'a> System<'a> for StartFrameSystem {
  type SystemData = (
    Write<'a, Renderer>,
    Write<'a, EventChannel<WindowEvent>>,
    Write<'a, WindowEventChannel>,
    Read<'a, ReceiverID>,
    Write<'a, Timestep>,
    Write<'a, Running>,
    ReadStorage<'a, Player>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Kinetics>,
    ReadStorage<'a, Rotation>,
  );

  fn run(&mut self, 
    (mut renderer, mut events, mut window_events, receiver_id, mut timestep, mut running,
      s_player, s_pos, s_kinetics, s_rotation
    ): Self::SystemData) {
    let mut window = self.window.borrow_mut();
    let delta = window.glfw_token.get_time() as f32 - self.last_time;
    self.last_time = self.last_time + delta;
    timestep.set_value(delta);
    window.poll_events();
    window_events.process_events(&mut events, &mut window);
    events.read(&receiver_id).for_each(|window_evt| match window_evt.code {
      Event::KeyPressed(KeyCode::Control) => {
        window.toggle_cursor();
      },
      Event::KeyPressed(KeyCode::Esc) => {
        running.set_value(false);
      }
      _ => {}
    });
    renderer.init_frame(&mut window);
    // events.process_events(&mut )
    // window.clear_framebuffer();
    for (_player, pos, kinetics, rotation) in (&s_player, &s_pos, &s_kinetics, &s_rotation).join() {
      let cam = Camera::new(&pos.0, &kinetics.velocity, &rotation);
      renderer.start_scene(cam, timestep.0);
    }
  }
}