use std::time::{Instant, Duration};

use cgmath::prelude::*;
use specs::prelude::*;

use ecs::components::{Player, Camera, DrawableId, MeshComponent, Material};
use events::{Event, EventChannel, StatelessEventChannel, KeyCode, ReceiverID, WindowEvent, WindowEventDispatcher};
use renderer::{DrawCommand, Renderer, Window, Mesh, RenderQueue, DrawCall};
use utils::{Mat4F, MutRef, Running, Timestep};

use physics::{TransformComponent};

pub struct RenderSystem {
  pub window: MutRef<Window>,
  render_queue: RenderQueue,
}

impl RenderSystem {
  pub fn new(window: MutRef<Window>) -> Self {
    Self {
      window,
      render_queue: RenderQueue::default(),
    }
  }
}

impl<'a> System<'a> for RenderSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, DrawableId>,
    ReadStorage<'a, TransformComponent>,
    ReadStorage<'a, Material>,
    Write<'a, Timestep>,
    Write<'a, Renderer>,
  );

  fn run(&mut self, (entities, drawables, transforms, materials, mut timestep, mut renderer): Self::SystemData) {
    let mut window = self.window.borrow_mut();
    for (entity, drawable) in (&entities, &drawables, ).join() {
      let cmd = DrawCall {
        drawable: drawable.clone(),
        entity,
      };
      self.render_queue.push(cmd);
    }
    // let mut window = self.window.borrow_mut();
    renderer.init_frame(&mut window);
    let start = window.glfw_token.get_time() as f32;
    renderer.draw_scene(self.render_queue.consume(), &materials, &transforms);
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
    WriteStorage<'a, MeshComponent>,
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
    for (entity, mesh) in (&entities, &mut drawables_storage).join() {
      let id = renderer.submit_model(mesh.mesh.clone());
      updater.remove::<MeshComponent>(entity);
      updater.insert(entity, id);
    }
  }

  fn setup(&mut self, world: &mut World) {
    world.register::<MeshComponent>();
  }
}