use specs::prelude::*;

use ecs::systems::*;
use super::Motion;

pub fn setup_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
  DispatcherBuilder::new()
    .with(PlayerEvents, "event_system", &[])
    .with(Motion, "motion_system", &["event_system"])
    .with(StartFrameSystem, "frame_init", &["motion_system"])
    .with_thread_local(RenderSystem)
    .build()
}