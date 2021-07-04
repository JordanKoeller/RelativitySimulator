use specs::prelude::*;
use events::ReceiverID;
use renderer::Window;
use ecs::systems::*;
use utils::{Running, MutRef, GetMutRef, Timestep};
use debug::DiagnosticsPanel;
use gui::GuiRenderer;

pub type SystemsRegistration<'a, 'b> = dyn Fn(DispatcherBuilder<'a,'b>) -> DispatcherBuilder<'a, 'b>;

pub struct GameLoop<'a, 'b> {
  window: MutRef<Window>,
  world: World,
  r_id: ReceiverID,
  app_system: Option<Box<SystemsRegistration<'a, 'b>>>
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

  pub fn run(
    &mut self,
  ) {
    // let mut last_time = window.glfw_token.get_time() as f32;
    let mut running = true;
    let mut window_open = true;
    let mut dispatcher = self.initialize();
    dispatcher.setup(&mut self.world);
    while running && window_open {
      dispatcher.dispatch(&self.world);
      self.world.maintain();
      {
        let window_ref = self.window.borrow();
        window_open = window_ref.is_open();
        let running_v = self.world.read_resource::<Running>();
        running = running_v.0;
      }
    }
  }

  fn initialize(&mut self) -> Dispatcher<'a, 'b> {
    if let Some(func) = &self.app_system {
      let window_handle = MutRef::clone(&self.window);
      let window_handle1 = MutRef::clone(&self.window);
      let window_handle2 = MutRef::clone(&self.window);
      let window_handle3 = MutRef::clone(&self.window);
      let dispatcher = DispatcherBuilder::new()
      .with_thread_local(StartFrameSystem {
        window: window_handle1,
        last_time: 0f32,
        receiver_id: self.r_id,
      })
      .with_thread_local(RegisterDrawableSystem)
      .with(EventProcessingSystem::default(), "event processing", &[])
      .with(ParticleUpdater, "particle_updater", &[])
      .with_barrier();
      
      let dispatcher = func(dispatcher);
      dispatcher
      .with(MotionSystem, "motion", &["player_controller"])
      .with_thread_local(RenderSystem::new(window_handle))
      .with_thread_local(GuiRenderer {window: window_handle2})
      .with_thread_local(EndFrameSystem {window: window_handle3})
      .build()
    } else {
      panic!("No dispatcher constructor provided");
    }
  }
}
