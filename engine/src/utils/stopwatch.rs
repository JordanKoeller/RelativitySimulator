use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub trait StopwatchLike<T> {
    // Start the stopwatch
    fn start(&mut self);
    // Stop the stopwatch
    fn stop(&mut self);

    // Get the stopwatch's internal value
    fn get(&self) -> Option<T>;

    // Reset the stopwatch
    fn reset(&mut self);
}

#[derive(Default)]
pub struct Stopwatch {
    start_time: Option<SystemTime>,
    end_time: Option<SystemTime>,
}

impl StopwatchLike<Duration> for Stopwatch {
    fn start(&mut self) {
        self.start_time = Some(SystemTime::now());
        self.end_time = None;
    }

    fn stop(&mut self) {
        self.end_time = Some(SystemTime::now());
    }

    fn get(&self) -> Option<Duration> {
        self.end_time
            .zip(self.start_time)
            .map(|(e, s)| e.duration_since(s).unwrap())
    }

    fn reset(&mut self) {
        self.start_time = None;
        self.end_time = None;
    }
}

pub struct IntervalStopwatch {
    stopwatch: Stopwatch,

    count: u32,
    sum_elapsed: Option<Duration>,

    last_avg: Option<Duration>,
    measurement_interval: u32,
}

impl IntervalStopwatch {
    pub fn new(measurement_interval: u32) -> Self {
        Self {
            stopwatch: Stopwatch::default(),
            count: 0,
            sum_elapsed: None,
            last_avg: None,
            measurement_interval,
        }
    }
}

impl StopwatchLike<Duration> for IntervalStopwatch {
    /*
    A stopwatch that takes N many samples and reports the average duration.

    Note this is NOT a windowed stopwatch. The average only updates every N
    many samples, versus a windowed stopwatch where it would report the average
    of the last N many samples, updating with every new sample.
    */
    fn start(&mut self) {
        self.stopwatch.start();
    }

    fn stop(&mut self) {
        self.stopwatch.stop();
        self.sum_elapsed = self.sum_elapsed.map_or(self.stopwatch.get(), |partial| {
            self.stopwatch.get().map(|adding| partial + adding)
        });
        self.count += 1;
        if self.count >= self.measurement_interval {
            self.last_avg = self.sum_elapsed.map(|total_duration| total_duration / self.count);
            self.count = 0;
            self.sum_elapsed = None;
        }
        self.stopwatch.reset();
    }

    fn get(&self) -> Option<Duration> {
        self.last_avg
    }

    fn reset(&mut self) {
        self.stopwatch.reset();

        self.count = 0;
        self.sum_elapsed = None;

        self.last_avg = None;
    }
}

pub struct CompoundStopwatch {
    instantaneous_stopwatch: Stopwatch,
    interval_stopwatch: IntervalStopwatch,
}

impl StopwatchLike<(Duration, Duration)> for CompoundStopwatch {
    fn start(&mut self) {
        self.instantaneous_stopwatch.start();
        self.interval_stopwatch.start();
    }
    // Stop the stopwatch
    fn stop(&mut self) {
        self.instantaneous_stopwatch.stop();
        self.interval_stopwatch.stop();
    }

    // Get the stopwatch's internal value
    fn get(&self) -> Option<(Duration, Duration)> {
        self.instantaneous_stopwatch.get().zip(self.interval_stopwatch.get())
    }

    // Reset the stopwatch
    fn reset(&mut self) {
        self.instantaneous_stopwatch.reset();
        self.interval_stopwatch.reset();
    }
}

impl CompoundStopwatch {
    pub fn new(measurement_interval: u32) -> Self {
        Self {
            instantaneous_stopwatch: Stopwatch::default(),
            interval_stopwatch: IntervalStopwatch::new(measurement_interval),
        }
    }
}

#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::Duration;

    use super::*;

    #[test]
    fn stopwatch_returns_none_if_reset_or_unstarted() {
        let mut watch = Stopwatch::default();
        assert_eq!(watch.get(), None);
        watch.start();
        watch.reset();
        assert_eq!(watch.get(), None);
        watch.start();
        watch.stop();
        assert_eq!(watch.get().is_some(), true);
        watch.reset();
        assert_eq!(watch.get(), None);
    }

    #[test]
    fn intervalstopwatch_returns_none_if_reset_or_unstarted() {
        let mut watch = IntervalStopwatch::new(1);
        assert_eq!(watch.get(), None);
        watch.start();
        watch.reset();
        assert_eq!(watch.get(), None);
        watch.start();
        watch.stop();
        assert_eq!(watch.get().is_some(), true);
        watch.reset();
        assert_eq!(watch.get(), None);
    }

    #[test]
    fn stopwatch_measures_time() {
        let mut watch = Stopwatch::default();
        watch.start();
        sleep(Duration::from_millis(100));
        watch.stop();
        if let Some(t) = watch.get() {
            assert_eq!(t.as_millis(), 100);
        } else {
            assert_eq!(false, true);
        }
    }

    #[test]
    fn intervalstopwatch_averages_time() {
        let mut watch = IntervalStopwatch::new(10);
        for t in 1..10 {
            watch.start();
            sleep(Duration::from_millis(t));
            watch.stop();
            assert_eq!(watch.get().is_none(), true);
        }
        watch.start();
        sleep(Duration::from_millis(5));
        watch.stop();
        if let Some(t) = watch.get() {
            assert_eq!(t.as_millis() == 5 || t.as_millis() == 6, true);
        } else {
            assert_eq!(true, false);
        }

        for t in 1..10 {
            watch.start();
            sleep(Duration::from_millis(t * 2));
            watch.stop();
            assert_eq!(watch.get().is_some(), true);
            // Watch value shouldn't update until next 10 measures
            let tt = watch.get().unwrap();
            assert_eq!(tt.as_millis() == 5 || tt.as_millis() == 6, true);
        }
        watch.start();
        sleep(Duration::from_millis(10));
        watch.stop();
        if let Some(t) = watch.get() {
            assert_eq!(t.as_millis() == 10 || t.as_millis() == 11 || t.as_millis() == 12, true);
        } else {
            assert_eq!(true, false);
        }
    }
}
