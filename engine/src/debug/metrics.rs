use crate::specs::*;

use crate::ecs::{MonoBehavior, SystemUtilities, WorldProxy};
use crate::gui::{widgets::*, ControlPanel, ControlPanelBuilder, SystemDebugger};
use crate::utils::{Averager, CompoundStopwatch, Counter, StopwatchLike, Timestep};

pub struct DebugMetrics {
    pub fps_counter: CompoundStopwatch,
    pub draw_calls: Counter,
    pub frame_time: CompoundStopwatch,
    pub render_time: CompoundStopwatch,
    pub poly_count: Counter,
}

impl Default for DebugMetrics {
    fn default() -> Self {
        Self {
            fps_counter: CompoundStopwatch::new(120u32),
            draw_calls: Counter::default(),
            frame_time: CompoundStopwatch::new(120u32),
            render_time: CompoundStopwatch::new(120u32),
            poly_count: Counter::default(),
        }
    }
}

pub struct DebugMetricsSystem {
    timestep_averager: Averager,
}

impl Default for DebugMetricsSystem {
    fn default() -> Self {
        Self {
            timestep_averager: Averager::new(100),
        }
    }
}

impl<'a> MonoBehavior<'a> for DebugMetricsSystem {
    type SystemData = (Write<'a, DebugMetrics>, Read<'a, Timestep>);

    fn run(&mut self, api: SystemUtilities<'a>, (mut debugger, timestep): Self::SystemData) {
        debugger.frame_time.stop();
        self.timestep_averager.push_value(timestep.dt_f64() * 1000f64);
        let mut panel = self.get_write_panel(&api);
        if let Some((_instant, avg)) = debugger.frame_time.get() {
            panel.set_str("FrameTime", format!("{:#?}", avg.as_micros() as f64 / 1000f64));
        }
        if let Some((instant, avg)) = debugger.render_time.get() {
            panel.set_str(
                "RenderTime",
                format!(
                    "{:.3} ({:.3} Avg)",
                    instant.as_micros() as f64 / 1000f64,
                    avg.as_micros() as f64 / 1000f64
                ),
            );
        }
        panel.set_str("DrawCalls", format!("{}", debugger.draw_calls.get()));
        panel.set_str("TimestepValue", format!("{:.6} Avg", self.timestep_averager.get_avg()));
        panel.set_str("PolyCount", format!("{} ", debugger.poly_count.get()));
        debugger.frame_time.start();
    }

    fn setup(&mut self, mut world: WorldProxy) {
        world.insert(DebugMetrics::default());
        self.register_debugger(&world);
    }
}

impl<'a> SystemDebugger<'a> for DebugMetricsSystem {
    fn create_panel(&self) -> ControlPanelBuilder {
        ControlPanelBuilder::default()
            .with_title("Debug Metrics")
            .push_line("FrameTime", LabeledText::new("NaN", "Frame Time (avg)"))
            .push_line("RenderTime", LabeledText::new("NaN", "Render Time"))
            .push_line("DrawCalls", LabeledText::new("NaN", "Draw Calls"))
            .push_line("PolyCount", LabeledText::new("NaN", "Poly Count"))
            .push_line("TimestepValue", LabeledText::new("NaN", "Timestep Value"))
    }
}
