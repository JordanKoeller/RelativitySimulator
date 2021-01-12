use specs::prelude::*;

use renderer::{Renderer};
use events::{EventChannel, WindowEvent, ImguiUiEvent};

pub struct EventProcessingSystem;

impl <'a> System<'a> for EventProcessingSystem {
  type SystemData = (
    Write<'a, Renderer>,
    Write<'a, EventChannel<WindowEvent>>,
    Write<'a, EventChannel<ImguiUiEvent>>,
  );

  fn run(&mut self, (mut renderer, mut window_evt, mut imgui_evt): Self::SystemData) {
    // Load component data into the UI
    renderer.process_events(&mut window_evt);
    // Finished processing events. Need to send the results of any user input back out

  }
}