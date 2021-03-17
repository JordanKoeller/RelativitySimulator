use specs::prelude::*;

use super::Motion;
use ecs::systems::*;
use events::ReceiverID;
use renderer::Window;
use utils::MutRef;
use ecs::entity::EntityManager;

use app::{StreetDelegate, BuildingDelegate, DistrictDelegate};

pub fn setup_dispatcher<'a, 'b>(window: MutRef<Window>, receiver_id: ReceiverID) -> Dispatcher<'a, 'b> {
  let window_handle = MutRef::clone(&window);
  DispatcherBuilder::new()
    .with_thread_local(StartFrameSystem {
      window,
      last_time: 0f32,
      receiver_id,
    })
    // .with(EntityManager::<StreetDelegate>::default(), "street_mgr", &[])
    .with_thread_local(EntityManager::<StreetDelegate>::default())
    .with_thread_local(EntityManager::<BuildingDelegate>::default())
    .with_thread_local(EntityManager::<DistrictDelegate>::default())
    .with(EventProcessingSystem::default(), "event processing", &[])
    .with(PlayerEvents, "plyer_events", &["event processing"])
    .with(Motion, "motion_system", &["plyer_events"])
    .with_thread_local(RenderSystem { window: window_handle })
    .build()
}
