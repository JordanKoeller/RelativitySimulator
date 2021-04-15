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
    .with_thread_local(RegisterDrawableSystem)
    .with(EntityManager::<DistrictDelegate>::default(), "district_mgr", &[])
    .with(EntityManager::<StreetDelegate>::default(), "street_mgr", &["district_mgr"])
    .with(EntityManager::<BuildingDelegate>::default(), "building_mgr", &["district_mgr"])
    .with(EventProcessingSystem::default(), "event processing", &[])
    .with(PlayerEvents, "plyer_events", &["event processing"])
    // .with(CollisionSystem::default(), "collision_system", &["plyer_events"])
    .with(Motion, "motion_system", &["plyer_events"])
    .with_thread_local(RenderSystem { window: window_handle })
    .build()
}
