use specs::prelude::*;

use renderer::Window;
use utils::{Running, MutRef};

pub struct GameLoop;

impl GameLoop {

  pub fn run(
    &mut self,
    dispatcher: &mut Dispatcher,
    world: &mut World,
    window: MutRef<Window>,
  ) {
    // let mut last_time = window.glfw_token.get_time() as f32;
    let mut running = true;
    let mut window_open = true;
    while running && window_open {
      dispatcher.dispatch(&world);
      {
        let window_ref = window.borrow();
        window_open = window_ref.is_open();
        let running_v = world.read_resource::<Running>();
        running = running_v.0;
      }
    }
  }
}
