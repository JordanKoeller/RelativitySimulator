use specs::prelude::*;

use crate::events::{EventChannel, ImguiUiEvent, ReceiverID, StatelessEventChannel, WindowEvent};
use crate::renderer::Renderer;

pub struct EventProcessingSystem {
    receiver_id: ReceiverID,
}

impl Default for EventProcessingSystem {
    fn default() -> Self {
        EventProcessingSystem {
            receiver_id: usize::MAX,
        }
    }
}

impl<'a> System<'a> for EventProcessingSystem {
    type SystemData = (
        Write<'a, Renderer>,
        Write<'a, StatelessEventChannel<WindowEvent>>,
        // Write<'a, EventChannel<ImguiUiEvent>>,
    );

    fn run(&mut self, (mut renderer, mut window_evt): Self::SystemData) {
        // Load component data into the UI
        renderer.process_events(&mut window_evt);
        // Finished processing events. Need to send the results of any user input back out
    }

    fn setup(&mut self, _world: &mut World) {
        // let mut imgui_channel = world.write_resource::<EventChannel<ImguiUiEvent>>();
        // self.receiver_id = imgui_channel.register_reader();
    }
}
