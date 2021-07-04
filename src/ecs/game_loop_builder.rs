use specs::prelude::*;

use utils::{MutRef, GetMutRef};
use renderer::Window;
use events::ReceiverID;
use ecs::systems::*;
use gui::GuiRenderer;
use debug::DiagnosticsPanel;


pub struct GameLoopBuilder<'a, 'b> {
  builder: DispatcherBuilder<'a, 'b>,
  window: MutRef<Window>,
}

impl<'a, 'b> GameLoopBuilder<'a, 'b> {
  pub fn new(window: MutRef<Window>, receiver_id: ReceiverID) -> Self {
    let window_handle = MutRef::clone(&window);
    Self {
      builder: DispatcherBuilder::new()
        .with_thread_local(StartFrameSystem {
          window, last_time: 0f32, receiver_id
        })
        .with_thread_local(RegisterDrawableSystem)
        .with(EventProcessingSystem::default(), "event processing", &[])
        .with(ParticleUpdater, "particle_updater", &[])
        .with_barrier(),
      window: window_handle,
    }
  }

  pub fn with<T>(self, system: T, name: &str, dep: &[&str]) -> Self
  where
    T: for<'c> System<'c> + Send + 'a,
  {
    Self {
      builder: self.builder.with(system, name, dep),
      window: self.window,
    }
  }

  pub fn with_player_controller<T>(self, system: T) -> Self
  where
    T: for<'c> System<'c> + Send + 'a,
  {
    self.with(system, "player_controller", &["event_processing"])
      .with(MotionSystem, "motion", &["player_controller"])
  }


  pub fn build(self) -> Dispatcher<'a, 'b> {
    let window_handle = MutRef::clone(&self.window);
    let window_handle2 = MutRef::clone(&self.window);
    let window_handle3 = MutRef::clone(&self.window);
    self.builder
    .with_thread_local(DiagnosticsPanel)
    .with_thread_local(RenderSystem::new(window_handle))
    .with_thread_local(GuiRenderer {window: window_handle2})
    .with_thread_local(EndFrameSystem {window: window_handle3})
    .build()
  }
}
