use specs::prelude::*;

use events::{Event, EventChannel, KeyCode, ReceiverID, WindowEvent, WindowEventChannel};
use renderer::{Renderer, Window};
use utils::Timestep;

pub struct GameLoop {
  receiver_id: ReceiverID,
  running: bool,
}

impl GameLoop {
  pub fn new(receiver_id: ReceiverID) -> GameLoop {
    GameLoop {
      receiver_id,
      running: false,
    }
  }

  pub fn run(
    &mut self,
    dispatcher: &mut Dispatcher,
    world: &mut World,
    window: &mut Window,
    window_event_receiver: &mut WindowEventChannel,
  ) {
    let mut last_time = window.glfw_token.get_time() as f32;
    self.running = true;
    while self.running && window.is_open() {
      let curr_time = window.glfw_token.get_time() as f32;
      let delta_time = curr_time - last_time;
      last_time = curr_time;
      {
        window.poll_events();
        let mut listener = world.write_resource::<EventChannel<WindowEvent>>();
        window_event_receiver.process_events(&mut listener, window);
        let mut renderer = world.write_resource::<Renderer>();
        renderer.process_events(&mut listener);
        self.handle_loop_events(&mut listener);
      }
      world.insert(Timestep(delta_time));
      {
        let mut renderer = world.write_resource::<Renderer>();
        renderer.init_frame(window);
      }
      dispatcher.dispatch(&world);
      {
        let mut renderer = world.write_resource::<Renderer>();
        renderer.draw_scene(window);
        renderer.end_frame(window);
      }
      // window.frame_end();
    }
  }

  pub fn stop(&mut self) {
    self.running = false;
  }

  pub fn handle_loop_events(&mut self, channel: &mut EventChannel<WindowEvent>) {
    channel
      .read(&self.receiver_id)
      .for_each(|window_evt| match window_evt.code {
        Event::KeyPressed(KeyCode::Control) => {
          self.stop();
        },
        Event::KeyPressed(KeyCode::Esc) => {
          self.stop();
        }
        _ => {}
      });
  }
}
