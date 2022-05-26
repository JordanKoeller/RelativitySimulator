use crate::specs::*;

use crate::ecs::{MonoBehavior, SystemUtilities, WorldProxy};
use crate::gui::{ControlPanel, ControlPanelBuilder, LabeledText, SystemDebugger, Widget};
use crate::utils::{CompoundStopwatch, Counter, StopwatchLike};

pub struct DebugMetrics {
    pub fps_counter: CompoundStopwatch,
    pub draw_calls: Counter,
    pub frame_time: CompoundStopwatch,
    pub render_time: CompoundStopwatch,
}

impl Default for DebugMetrics {
    fn default() -> Self {
        Self {
            fps_counter: CompoundStopwatch::new(120u32),
            draw_calls: Counter::default(),
            frame_time: CompoundStopwatch::new(120u32),
            render_time: CompoundStopwatch::new(120u32),
        }
    }
}

#[derive(Default)]
pub struct DebugMetricsSystem;

impl<'a> MonoBehavior<'a> for DebugMetricsSystem {
    type SystemData = Read<'a, DebugMetrics>;

    fn run(&mut self, api: SystemUtilities<'a>, debugger: Self::SystemData) {
        let mut panel = self.get_write_panel(&api);
        if let Some((_instant, avg)) = debugger.frame_time.get() {
            panel.set_str("FrameTime", format!("{:#?}", avg));
        }
        if let Some((_instant, avg)) = debugger.render_time.get() {
            panel.set_str("RenderTime", format!("{:#?}", avg));
        }
        panel.set_str("DrawCalls", format!("{}", debugger.draw_calls.get()));
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
            .push_line("RenderTime", LabeledText::new("NaN", "Render Time (avg)"))
            .push_line("DrawCalls", LabeledText::new("NaN", "Draw Calls"))
    }
}
