use std::time::Duration;
use std::time::Instant;
#[derive(Debug)]
pub struct Timestep {
  last_click_time: Instant,
  current_click_time: Instant,
}

impl Default for Timestep {
  fn default() -> Self {
    Self {
      last_click_time: Instant::now() - Duration::from_millis(16),
      current_click_time: Instant::now(),
    }
  }
}

impl Timestep {
  pub fn click(&mut self) {
    self.last_click_time = self.current_click_time;
    self.current_click_time = Instant::now();
  }

  pub fn dt(&self) -> Duration {
    self.current_click_time - self.last_click_time
  }

  pub fn dt_f32(&self) -> f32 {
    self.dt().as_secs_f32()
  }
}
