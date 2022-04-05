use crate::debug::*;
use crate::ecs::systems::*;
use crate::ecs::{SystemDelegate, SystemManager};
use crate::events::ReceiverID;
use crate::gui::GuiRenderer;
use crate::renderer::Window;
use crate::utils::{GetMutRef, MutRef, RunningEnum, RunningState, Timestep};
use specs::prelude::*;
use std::time::Duration;

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
        self.world.maintain();
        ret
    }
}
