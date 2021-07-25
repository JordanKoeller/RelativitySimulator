use lazy_static::lazy_static;
use std::sync::Mutex;
use utils::{RunningState, RunningEnum};
use debug::gl_debug::*;

lazy_static! {
  static ref RUNNING_STATE: Mutex<RunningState> = Mutex::new(RunningState::default());
}

pub fn sync_running_state(state: &RunningEnum) {
  RUNNING_STATE.lock().unwrap().state = state.clone();
}

#[inline]
pub fn _print_if_stepping(msg: String) {
  match RUNNING_STATE.lock().unwrap().state {
    RunningEnum::StepFrame => {
      println!("{}", msg);
    }
    _ => {}
  }
}

#[allow(unused_macros)]
macro_rules! step_debug {
  ($literal:expr) => {
    #[cfg(feature = "debug")]
    _print_if_stepping($literal);
  }
}