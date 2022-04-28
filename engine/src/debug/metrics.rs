use crate::utils::{CompoundStopwatch, Counter, StopwatchLike};

struct DebugMetrics {
    pub fps_counter: CompoundStopwatch,
    pub draw_calls: Counter,
    pub frame_time: CompoundStopwatch,
    pub render_time: CompoundStopwatch,
}
