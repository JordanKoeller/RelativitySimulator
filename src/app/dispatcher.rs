use specs::prelude::*;

use ecs::systems::*;
use super::Motion;
use renderer::Window;
use utils::MutRef;

pub fn setup_dispatcher<'a, 'b>(window: MutRef<Window>) -> Dispatcher<'a, 'b> {
  let window_handle = MutRef::clone(&window);
  DispatcherBuilder::new()
  .with_thread_local(StartFrameSystem {window, last_time: 0f32})
    .with(EventProcessingSystem, "event processing", &[])
    .with(PlayerEvents, "plyer_events", &["event processing"])
    .with(Motion, "motion_system", &["plyer_events"])
    .with_thread_local(RenderSystem {window: window_handle})
    .build()
}