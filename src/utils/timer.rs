use std::time::Duration;
use utils::{rand_choice, Timestep};

use debug::*;

pub trait TimerLike {
    fn poll(&mut self, time: Duration) -> bool;
    fn start(&mut self, time: Duration);
    fn started(&self) -> bool;

    // fn interval_expectation_value(&self) -> u32;

    fn start_poll(&mut self, time: Duration) -> bool {
        if self.started() {
            self.poll(time)
        } else {
            self.start(time);
            false
        }
    }

    fn start_poll_all(&mut self, time: Duration) -> u32 {
        if self.started() {
            self.poll_all(time)
        } else {
            self.start(time);
            0
        }
    }
    fn poll_all(&mut self, time: Duration) -> u32 {
        let mut c = 0;
        while self.poll(time) {
            c += 1;
        }
        c
    }
}

pub struct Timer {
    interval: Duration, // Interval, in miliseconds
    next_timestamp: Option<Duration>,
}

impl Timer {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            next_timestamp: None,
        }
    }
    pub fn reset_interval(&mut self, time: Duration) {
        self.interval = time;
    }
}

impl TimerLike for Timer {
    fn poll(&mut self, time: Duration) -> bool {
        if let Some(next_time) = self.next_timestamp {
            if time >= next_time {
                self.next_timestamp = self.next_timestamp.map(|nt| nt + self.interval);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn start(&mut self, time: Duration) {
        self.next_timestamp = Some(self.interval + time);
    }

    fn started(&self) -> bool {
        self.next_timestamp.is_some()
    }
}

pub struct WindowedTimer {
    min_interval: Duration,
    max_interval: Duration,
    last_emitted: Option<Duration>,
}

impl TimerLike for WindowedTimer {
    fn poll(&mut self, time: Duration) -> bool {
        if let Some(last_time) = self.last_emitted {
            let time_since = time - last_time;
            if time_since > self.max_interval {
                self.last_emitted = self
                    .last_emitted
                    .map(|le| le + (self.max_interval - self.min_interval) / 2u32); // Add the expectation value
                true
            } else if time_since > self.min_interval {
                let probability = (time_since - self.min_interval).as_secs_f32()
                    / (self.max_interval - self.min_interval).as_secs_f32();
                if rand_choice(probability) {
                    self.last_emitted = Some(time);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    fn start(&mut self, time: Duration) {
        self.last_emitted = Some(time);
    }
    fn started(&self) -> bool {
        self.last_emitted.is_some()
    }
}

impl WindowedTimer {
    pub fn new(min_time: Duration, max_time: Duration) -> Self {
        Self {
            min_interval: min_time,
            max_interval: max_time,
            last_emitted: None,
        }
    }
}
