use std::time::{Duration, SystemTime, UNIX_EPOCH};

trait StopwatchLike {
    // Start the stopwatch
    fn start(&mut self);
    // Stop the stopwatch
    fn stop(&mut self);

    // Get the stopwatch's internal value
    fn get(&self) -> Option<Duration>;

    // Reset the stopwatch
    fn reset(&mut self);
}

pub struct Stopwatch {
    start_time: Option<SystemTime>
    end_time: Option<SystemTime>
}


impl StopwatchLike for Stopwatch {

    fn start(&mut self) {
        self.start_time = Some(SystemTime::now());
        self.end_time = None;
    }

    fn stop(&mut self) {
        if let Some(self.start_time) {
            self.end_time = Some(SystemTime::now());
        } else {
            panic!("Tried to stop a stopwatch that was never started!");
        }
    }

    fn get(&self) -> Option<Duration> {

    }
}