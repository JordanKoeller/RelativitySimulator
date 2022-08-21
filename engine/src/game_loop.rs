use log::info;
use specs::prelude::*;
use std::time::Duration;

use crate::debug::*;
use crate::ecs::systems::*;
use crate::events::ReceiverID;
use crate::graphics::AssetLibrary;
use crate::gui::GuiRenderer;
use crate::platform::Window;
use crate::utils::{GetMutRef, MutRef, RunningEnum, RunningState, StopwatchLike, Timestep};

pub type SystemsRegistration<'a, 'b> = dyn Fn(DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b>;

pub struct GameLoop<'a, 'b> {
  world: World,
  dispatcher: Dispatcher<'a, 'b>,
  window: MutRef<Window>,
}

impl<'a, 'b> GameLoop<'a, 'b> {
  pub fn new(world: World, dispatcher: Dispatcher<'a, 'b>, window: MutRef<Window>) -> Self {
    Self {
      world,
      dispatcher,
      window,
    }
  }

  // pub fn with_resources(&mut self, )

  pub fn run(&mut self) {
    let mut running = true;
    self.dispatcher.setup(&mut self.world);
    self.maintain();
    {
      self.world.write_resource::<DebugMetrics>().frame_time.start();
    }
    while running {
      running = self.step_frame();
    }
  }

  fn step_frame(&mut self) -> bool {
    let running_state = { self.world.read_resource::<RunningState>().state.clone() };
    let window_open = self.window.borrow().is_open().clone();
    sync_running_state(&running_state);
    let ret = match running_state {
      RunningEnum::Running => {
        self.dispatcher.dispatch(&self.world);
        gl_check_error!("FRAME MESSAGE");
        window_open
      }
      RunningEnum::Stopped => false,
      RunningEnum::StepFrame => {
        self.dispatcher.dispatch(&self.world);
        let mut running = self.world.write_resource::<RunningState>();
        running.state = RunningEnum::StepFrameWait;
        gl_check_error!("FRAME MESSAGE");
        window_open
      }
      RunningEnum::StepFrameWait => {
        // step_frame_dispatcher.dispatch(&self.world);
        window_open
      } // _ => {window_open}
    };
    self.maintain();
    ret
  }

  fn maintain(&mut self) {
    {
      let mut asset_library = self.world.write_resource::<AssetLibrary>();
      asset_library.flush_all();
    }
    {
      self.world.write_resource::<Timestep>().click();
    }
    self.world.maintain();
  }
}
