use debug::DiagnosticsPanel;
use ecs::systems::*;
use events::ReceiverID;
use gui::GuiRenderer;
use renderer::Window;
use specs::prelude::*;
use utils::{GetMutRef, MutRef, RunningState, RunningEnum, Timestep};

pub type SystemsRegistration<'a, 'b> = dyn Fn(DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b>;

pub struct GameLoop<'a, 'b> {
  window: MutRef<Window>,
  world: World,
  r_id: ReceiverID,
  app_system: Option<Box<SystemsRegistration<'a, 'b>>>,
}

impl<'a, 'b> GameLoop<'a, 'b> {
  pub fn new(window: Window, world: World, r_id: ReceiverID) -> Self {
    Self {
      window: GetMutRef(window),
      world,
      app_system: None,
      r_id,
    }
  }

  pub fn with_systems(&mut self, func: Box<SystemsRegistration<'a, 'b>>) {
    self.app_system = Some(func);
  }

  // pub fn with_resources(&mut self, )

  pub fn run(&mut self) {
    // let mut last_time = window.glfw_token.get_time() as f32;
    let mut running = true;
    let mut dispatcher = self.initialize();
    let mut stepper = self.init_frame_stepper();
    dispatcher.setup(&mut self.world);
    while running {
      // dispatcher.dispatch_seq(&self.world);
      // dispatcher.dispatch_thread_local(&self.world);
      running = self.step_frame(&mut dispatcher, &mut stepper);
    }
  }

  fn step_frame(&mut self, dispatcher: &mut Dispatcher, step_frame_dispatcher: &mut Dispatcher) -> bool {
    let running_state = {
      self.world.read_resource::<RunningState>().state.clone()
    };
    let window_open = self.window.borrow().is_open().clone();
    self.world.maintain();
    match running_state {
      RunningEnum::Running => {
        dispatcher.dispatch(&self.world);
        window_open
      }
      RunningEnum::Stopped => {
        false
      }
      RunningEnum::StepFrame => {
        dispatcher.dispatch(&self.world);
        let mut running = self.world.write_resource::<RunningState>();
        running.state = RunningEnum::StepFrameWait;
        window_open
      }
      RunningEnum::StepFrameWait => {
        step_frame_dispatcher.dispatch(&self.world);
        window_open
      }
      // _ => {window_open}
    }
    // self.world.maintain();
    // let window_ref = self.window.borrow();
    // self.world.read_resource::<Running>().0 && window_ref.is_open()
  }

  fn initialize(&mut self) -> Dispatcher<'a, 'b> {
    if let Some(func) = &self.app_system {
      let window_handle = MutRef::clone(&self.window);
      let window_handle1 = MutRef::clone(&self.window);
      let window_handle2 = MutRef::clone(&self.window);
      let window_handle3 = MutRef::clone(&self.window);
      let dispatcher = DispatcherBuilder::new()
        .with_thread_local(RegisterDrawableSystem)
        .with(EventProcessingSystem::default(), "event processing", &[])
        .with(ParticleUpdater, "particle_updater", &[])
        .with_barrier();
      let dispatcher = func(dispatcher);
      dispatcher
        .with(MotionSystem, "motion", &["player_controller"])
        .with_thread_local(StartFrameSystem {
          window: window_handle1,
          receiver_id: self.r_id,
        })
        .with_thread_local(RenderSystem::new(window_handle))
        .with_thread_local(GuiRenderer { window: window_handle2 })
        .with_thread_local(EndFrameSystem { window: window_handle3 })
        .build()
    } else {
      panic!("No dispatcher constructor provided");
    }
  }

  fn init_frame_stepper(&mut self) -> Dispatcher<'a, 'b> {
    let window_handle = MutRef::clone(&self.window);
    let window_handle1 = MutRef::clone(&self.window);
    let window_handle2 = MutRef::clone(&self.window);
    let window_handle3 = MutRef::clone(&self.window);
    DispatcherBuilder::new()
      .with_thread_local(EventProcessingSystem::default())
      .with_thread_local(StartFrameSystem {
        window: window_handle,
        receiver_id: self.r_id
      })
      .with_thread_local(RenderSystem::new(window_handle1))
      .with_thread_local(GuiRenderer { window: window_handle2 })
      .with_thread_local(EndFrameSystem { window: window_handle3 })
      .build()
  }
}