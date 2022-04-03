extern crate specs;

use events::*;
use specs::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Debugger(ReceiverID);
impl Component for Debugger {
    type Storage = VecStorage<Self>;
}

pub trait Debuggable {
    type ValueType;

    // fn evt_id(&self) -> usize {
    //   CALL_COUNT.fetch_add(1, Ordering::SeqCst)
    // }

    fn accept_value(&mut self, event: &ImguiUiEvent);

    fn emit_value(&self) -> Self::ValueType;

    fn get_events(&self) -> Vec<ImguiUiEvent>;
}

// #[derive(SystemData)]
// pub struct MySystemData<'a> {
//     positions: ReadStorage<'a, Position>,
//     velocities: ReadStorage<'a, Velocity>,
//     forces: ReadStorage<'a, Force>,

//     delta: Read<'a, DeltaTime>,
//     game_state: Write<'a, GameState>,
// }

// pub struct DebugSystem<T: Component + Sync + Send + Debuggable> {
//   data: std::collections::HashMap<ReceiverID, T::ValueType>,
// }

// impl<'a, T: Component + Sync + Send + Debuggable> System<'a> for DebugSystem<T> {
//   type SystemData = (
//     ReadStorage<'a, Debugger>,
//     WriteStorage<'a, T>,
//     Write<'a, EventChannel<ImguiUiEvent>>,
//   );

//   fn run(&mut self, (receiver_storage, mut debug_storage, evts): Self::SystemData) {
//     let mut events: std::collections::HashMap<ReceiverID, Vec<&ImguiUiEvent>> = std::collections::HashMap::new();
//     for (receiver, debugger) in (&receiver_storage, &mut debug_storage).join() {
//       if !events.contains_key(&receiver.0) {
//         let inbox = evts.read(&receiver.0);
//         events.insert(receiver.0, inbox.collect());
//       }
//       let receiver_events = events.get(&receiver.0);
//       match receiver_events {
//         Some(evts) => {
//           for evt in evts.iter() {
//             debugger.accept_value(evt);
//           }
//         }
//         None => {}
//       }
//     }
//   }
// }

// trait DebugSystem<'a> {
//   type DebugData: SystemData<'a>;
// }

// impl<'a, S: DebugSystem<'a> + 'static> System<'a> for S {
//   type SystemData = (
//     Write<'a, EventChannel<ImguiUiEvent>>,
//     ReadStorage<'a, Debugger>,
//     S::DebugData
//   );
// }
