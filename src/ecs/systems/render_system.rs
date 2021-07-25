use std::time::{Instant, Duration};

use cgmath::prelude::*;
use specs::prelude::*;
use specs::prelude::SystemData;

use ecs::components::{Player, Camera, DrawableId, MeshComponent, Material};
use events::{Event, EventChannel, StatelessEventChannel, KeyCode, ReceiverID, WindowEvent, WindowEventDispatcher};
use renderer::{RenderCommand, Renderer, Window, Mesh, RenderQueue, DrawCall, AssetLibrary};
use renderer::render_pipeline::*;
use utils::{Mat4F, MutRef, RunningState, RunningEnum, Timestep};

use physics::{TransformComponent};

pub struct RenderSystem {
  pub window: MutRef<Window>,
  event_receiver_id: ReceiverID,
}

impl RenderSystem {
  pub fn new(window: MutRef<Window>, id: ReceiverID) -> Self {
    Self {
      window,
      event_receiver_id: id,
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
    Write<'a, RenderQueue>,
  );

  fn run(&mut self, (entities, drawables, transforms, materials, mut timestep, mut renderer, mut render_queue): Self::SystemData) {
    let mut window = self.window.borrow_mut();
    for (entity, drawable) in (&entities, &drawables).join() {
      let cmd = DrawCall {
        drawable: drawable.clone(),
        entity,
        cmd: RenderCommand::Draw,
      };
      render_queue.push(cmd);
    }
    // let mut window = self.window.borrow_mut();
    renderer.init_frame(&mut window);
    let start = window.glfw_token.get_time() as f32;
    // renderer.draw_scene(render_queue.consume(), &materials, &transforms);
    timestep.set_render_time(window.glfw_token.get_time() as f32 - start);
  }

  fn setup(&mut self, world: &mut World) {
    world.insert(RenderQueue::default());
  }
}

pub struct StartFrameSystem {
  pub window: MutRef<Window>,
  pub receiver_id: ReceiverID,
}

impl<'a> System<'a> for StartFrameSystem {
  type SystemData = (
    Write<'a, Renderer>,
    Write<'a, StatelessEventChannel<WindowEvent>>,
    Write<'a, WindowEventDispatcher>,
    Write<'a, Timestep>,
    Write<'a, RunningState>,
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
    timestep.click_frame(window.glfw_token.get_time() as f32);
    // let delta = window.glfw_token.get_time() as f32 - self.last_time;
    // self.last_time = self.last_time + delta;
    // timestep.set_click(delta);
    window.poll_events();
    window_events.process_events(&mut events, &mut window);
    events.for_each(&self.receiver_id, |window_evt| match window_evt.code {
      Event::KeyPressed(KeyCode::Control) => {
        window.toggle_cursor();
      }
      Event::KeyPressed(KeyCode::Esc) => {

        running.state = RunningEnum::Stopped;
      }
      Event::KeyPressed(KeyCode::Alt) => {
        match running.state {
          RunningEnum::Running => running.state = RunningEnum::StepFrameWait,
          RunningEnum::StepFrameWait => running.state = RunningEnum::Running,
          _ => {}
        }
        println!("Updated running state to {:?}", running.state);
      }
      Event::KeyPressed(KeyCode::F) => {

        match running.state {
          RunningEnum::StepFrameWait => {
            running.state = RunningEnum::StepFrame;
            println!("Stepping!");
          },
          _ => {}
        }
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

#[derive(SystemData)]
pub struct RenderSystemData<'a> {
  entities: Entities<'a>,
  drawable_s: ReadStorage<'a, DrawableId>,
  transform_s: ReadStorage<'a, TransformComponent>,
  material_s: ReadStorage<'a, Material>,
  renderer: Write<'a, Renderer>, 
  timestep: Write<'a, Timestep>,
  render_queue: Write<'a, RenderQueue>,
}

pub struct RenderPipelineSystem {
  window: MutRef<Window>,
  event_receiver_id: ReceiverID,
}

impl RenderPipelineSystem {
  pub fn new(window: MutRef<Window>, id: ReceiverID) -> Self {
    Self {
      window,
      event_receiver_id: id,
    }
  }
}

impl<'a> System<'a> for RenderPipelineSystem {

  type SystemData = RenderSystemData<'a>;

  fn run(&mut self, mut system_data: Self::SystemData) {
    self.prepare_queue(&mut system_data.render_queue, &system_data.entities, &system_data.drawable_s);
    self.init_frame(&mut system_data.renderer);
    let start_time = self.window.borrow().glfw_token.get_time() as f32;
    self.render(&mut system_data);
    // self.end_frame(&mut system_data);
    let end_time = self.window.borrow().glfw_token.get_time() as f32;
    system_data.timestep.set_render_time(end_time - start_time);
  }
}

impl RenderPipelineSystem {
  fn prepare_queue<'a>(
    &self,
    render_queue: &mut RenderQueue,
    entities: &Entities<'a>,
    drawables: &ReadStorage<'a, DrawableId>) {
      for (entity, drawable) in (entities, drawables).join() {
        let cmd = DrawCall {
          drawable: drawable.clone(),
          entity,
          cmd: RenderCommand::Draw,
        };
        render_queue.push(cmd);
      }
  }

  fn init_frame(&self, renderer: &mut Renderer) {
    renderer.init_frame(&mut self.window.borrow_mut());
  }

  fn render<'a, 'b>(&mut self, system_data: &mut <Self as System<'a>>::SystemData) {
    system_data.renderer.render_scene(system_data.render_queue.consume(), &system_data.material_s, &system_data.transform_s);
  }

  fn end_frame<'a>(&mut self, system_data: &mut <Self as System<'a>>::SystemData) {
    system_data.renderer.end_frame(&mut self.window.borrow_mut());
  }
}