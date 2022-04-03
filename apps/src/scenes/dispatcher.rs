use specs::prelude::*;

use ecs::systems::*;
use events::ReceiverID;
use renderer::Window;
use utils::{MutRef, GetMutRef, Mat4F, Vec3F};
use ecs::entity::EntityManager;
use debug::DiagnosticsPanel;
use ecs::components::{Camera};

use app::{StreetDelegate, BuildingDelegate, DistrictDelegate};
use gui::GuiRenderer;

pub fn setup_dispatcher<'a, 'b>(window: MutRef<Window>, receiver_id: ReceiverID) -> Dispatcher<'a, 'b> {
  let window_handle = MutRef::clone(&window);
  let window_handle2 = MutRef::clone(&window);
  let window_handle3 = MutRef::clone(&window);
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
    // .with(PlayerEvents, "plyer_events", &["event processing"])
    .with(MotionSystem, "motion_system", &["plyer_events"])
    .with(ParticleUpdater, "particles", &["motion_system"])
    .with(DiagnosticsPanel, "diagnostics", &["plyer_events"])
    .with_thread_local(RenderSystem { window: window_handle })
    .with_thread_local(GuiRenderer {window: window_handle2})
    .with_thread_local(EndFrameSystem {window: window_handle3})
    .build()
}

