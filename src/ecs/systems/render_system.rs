use std::time::{Duration, Instant};

use cgmath::prelude::*;
use specs::prelude::SystemData;
use specs::prelude::*;

use ecs::SystemDelegate;

use ecs::components::{Camera, DrawableId, Material, MeshComponent, Player};
use events::{Event, EventChannel, KeyCode, ReceiverID, StatelessEventChannel, WindowEvent, WindowEventDispatcher};
use gui::*;
use renderer::render_pipeline::*;
use renderer::{AssetLibrary, DrawCall, Mesh, RenderCommand, RenderQueue, Renderer, Window};
use utils::{Mat4F, MutRef, RunningEnum, RunningState, Timestep};

use physics::TransformComponent;

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
    (mut renderer, mut events, mut window_events, mut timestep, mut running, camera_storage): Self::SystemData,
  ) {
    let mut window = self.window.borrow_mut();
    timestep.click_frame(Duration::from_secs_f64(window.glfw_token.get_time()));
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
      Event::KeyPressed(KeyCode::F) => match running.state {
        RunningEnum::StepFrameWait => {
          running.state = RunningEnum::StepFrame;
          println!("Stepping!");
        }
        _ => {}
      },
      _ => {}
    });
    renderer.init_frame(&mut window);
    for camera in (&camera_storage).join() {
      renderer.start_scene(&camera, &timestep);
    }
  }
}

pub struct EndFrameSystem {
  pub window: MutRef<Window>,
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

  fn run(&mut self, (mut renderer, mut drawables_storage, entities, updater): Self::SystemData) {
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
  draw_call_count: u32,
  render_time: Duration,
}

impl RenderPipelineSystem {
  pub fn new(window: MutRef<Window>, id: ReceiverID) -> Self {
    Self {
      window,
      event_receiver_id: id,
      draw_call_count: 0u32,
      render_time: Duration::new(0u64, 0u32),
    }
  }
}

impl<'a> SystemDelegate<'a> for RenderPipelineSystem {
  type SystemData = RenderSystemData<'a>;

  fn run(&mut self, mut system_data: Self::SystemData) {
    self.prepare_queue(
      &mut system_data.render_queue,
      &system_data.entities,
      &system_data.drawable_s,
    );
    // self.init_frame(&mut system_data.renderer);
    let start_time = self.window.borrow().glfw_token.get_time();
    let draw_call_count = self.render(&mut system_data);
    let end_time = self.window.borrow().glfw_token.get_time();
    self.render_time = Duration::from_secs_f64(end_time - start_time);
    self.draw_call_count = draw_call_count;
  }

  fn update_debugger(&mut self, data: &mut Self::SystemData, gui: &mut DebugPanel) {
    gui.panel.lines[1] = Box::from(LabeledText::new("Draw Calls", &self.draw_call_count.to_string()));
    gui.panel.lines[2] = Box::from(LabeledText::new(
      "GPU Render Time",
      &format!("{0:.3}", self.render_time.as_secs_f32() * 1000f32),
    ));
  }

  fn setup(&mut self, world: &mut World) {
  }

  fn setup_debug_panel(&mut self, _: &mut World) -> Option<DebugPanel> {
    let mut ui = DebugPanel::new("Renderer Debugger");
    ui.panel.push(Box::from(LineBreak));
    ui.panel.push(Box::from(LabeledText::new("Draw Calls", "")));
    ui.panel.push(Box::from(LabeledText::new("GPU Render Time", "")));
    Some(ui)
  }
}

impl RenderPipelineSystem {
  fn prepare_queue<'a>(
    &self,
    render_queue: &mut RenderQueue,
    entities: &Entities<'a>,
    drawables: &ReadStorage<'a, DrawableId>,
  ) {
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

  fn render<'a, 'b>(&mut self, system_data: &mut <Self as SystemDelegate<'a>>::SystemData) -> u32 {
    system_data.renderer.render_scene(
      system_data.render_queue.consume(),
      &system_data.material_s,
      &system_data.transform_s,
    )
  }



}
