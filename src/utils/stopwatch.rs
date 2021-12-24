use std::fmt;

use stopwatch::Stopwatch;

pub struct AggregatingStopwatch {
  watch: Stopwatch,
  accumulator: i64,
  window_length: i64,
  value: f64,
  num_accumulated: i64,
}

impl AggregatingStopwatch {
  pub fn new(window_length: i32) -> Self {
    Self {
      watch: Stopwatch::new(),
      accumulator: 0,
      window_length: window_length as i64,
      value: 0f64,
      num_accumulated: 0,
    }
  }

  pub fn start(&mut self) {
    self.watch.start();
  }

  pub fn stop(&mut self) {
    if self.watch.is_running() {
      self.watch.stop();
      self.num_accumulated += 1;
      self.accumulator += self.watch.elapsed_ms();
      self.watch.reset();
      if self.num_accumulated == self.window_length {
        self.value = (self.accumulator as f64) / (self.window_length as f64);
        self.num_accumulated = 0;
        self.accumulator = 0;
      }
    }
  }

  pub fn get_value(&self) -> f64 {
    self.value
  }
}

impl fmt::Display for AggregatingStopwatch {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} AVG ({:.3} ms)", self.window_length, self.value)
  }
}
