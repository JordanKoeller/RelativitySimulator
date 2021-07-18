

#[derive(Clone, Debug)]
pub enum RunningEnum {
  Running,
  Stopped,
  StepFrame,
  StepFrameWait,
}

impl Default for RunningEnum {
  fn default() -> Self {
    RunningEnum::Running
  }
}

#[derive(Default, Clone, Debug)]
pub struct RunningState {
  pub state: RunningEnum
}

impl RunningState {

  pub fn stepping(&self) -> bool {
    match self.state {
      RunningEnum::StepFrame => {true},
      _ => {false}
    }
  }
}