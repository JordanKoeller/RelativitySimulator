use cgmath::prelude::*;
use specs::prelude::*;

use ecs::components::{Kinetics, Player, Position, Rotation, Transform, Camera};
use events::{Event, EventChannel, StatelessEventChannel, KeyCode, ReceiverID, WindowEvent, WindowEventDispatcher};
use renderer::{DrawCommand, DrawableId, Renderer, Window, DrawableMemo, DrawableState};
use utils::{Mat4F, MutRef, Running, Timestep};

pub struct RenderSystem {
  pub window: MutRef<Window>,
}

impl<'a> System<'a> for RenderSystem {
  type SystemData = (
    ReadStorage<'a, DrawableId>,
    ReadStorage<'a, Transform>,
    Write<'a, Timestep>,
    Write<'a, Renderer>,
  );

  fn run(&mut self, (drawables, transforms, mut timestep, mut renderer): Self::SystemData) {
    let window = self.window.borrow_mut();
    for (drawable, maybe_transform) in (&drawables, (&transforms).maybe()).join() {
      if let Some(transform) = maybe_transform {
        renderer.submit(DrawCommand {
          id: drawable.clone(),
          transform: transform.clone(),
        });
      } else {
        renderer.submit(DrawCommand {
          id: drawable.clone(),
          transform: Transform(Mat4F::one()),
        });
      }
    }
    // let mut window = self.window.borrow_mut();
    let start = window.glfw_token.get_time() as f32;
    renderer.draw_scene();
    timestep.set_render_time(window.glfw_token.get_time() as f32 - start);

  }
}

pub struct StartFrameSystem {
  pub window: MutRef<Window>,
  pub last_time: f32,
  pub receiver_id: ReceiverID,
}

impl<'a> System<'a> for StartFrameSystem {
  type SystemData = (
    Write<'a, Renderer>,
    Write<'a, StatelessEventChannel<WindowEvent>>,
    Write<'a, WindowEventDispatcher>,
    Write<'a, Timestep>,
    Write<'a, Running>,
    ReadStorage<'a, Camera>,
  );

  fn run(
    &mut self,
    (
      mut renderer,
      mut events,
      mut window_events,
      mut timestep,
      mut running,
      camera_storage
    ): Self::SystemData,
  ) {
    let mut window = self.window.borrow_mut();
    let delta = window.glfw_token.get_time() as f32 - self.last_time;
    self.last_time = self.last_time + delta;
    timestep.set_click(delta);
    window.poll_events();
    window_events.process_events(&mut events, &mut window);
    events.for_each(&self.receiver_id, |window_evt| match window_evt.code {
      Event::KeyPressed(KeyCode::Control) => {
        window.toggle_cursor();
      }
      Event::KeyPressed(KeyCode::Esc) => {
        running.set_value(false);
      }
      _ => {}
    });
    renderer.init_frame(&mut window);
    for camera in (&camera_storage).join() {
      renderer.start_scene(&camera, &timestep);

    }
    // events.process_events(&mut )
    // window.clear_framebuffer();
    // for (_player, pos, kinetics, rotation) in (&s_player, &s_pos, &s_kinetics, &s_rotation).join() {
    //   let cam = Camera::new(&pos.0, &kinetics.velocity, &rotation);
    //   renderer.start_scene(cam, &timestep);
    // }
  }
}

pub struct EndFrameSystem {
  pub window: MutRef<Window>
}

impl<'a> System<'a> for EndFrameSystem {

  type SystemData = Write<'a, Renderer>;

  fn run(&mut self, mut renderer: Self::SystemData) {
    let mut window = self.window.borrow_mut();
    renderer.end_frame(&mut window);
  }
}


pub struct RegisterDrawableSystem;

impl<'a> System<'a> for RegisterDrawableSystem {
  type SystemData = (
    Write<'a, Renderer>,
    WriteStorage<'a, DrawableState>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );

  fn run(&mut self,
    (
      mut renderer,
      mut drawables_storage,
      entities,
      updater
    ): Self::SystemData
  ) {
    for (entity, drawable_state) in (&entities, &mut drawables_storage).join() {
      drawable_state.refresh();
      let id = renderer.submit_model(drawable_state.clone());
      updater.remove::<DrawableState>(entity);
      updater.insert(entity, id);
    }
  }

  fn setup(&mut self, world: &mut World) {
    world.register::<DrawableState>();
  }
}