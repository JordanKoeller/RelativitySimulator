use super::Widget;
use specs::prelude::*;
use specs::{Component, HashMapStorage, NullStorage, VecStorage};

pub struct GuiInputPanel {
    pub title: String,
    pub lines: Vec<Box<dyn Widget + Sync + Send>>,
    pub active: bool,
    pub renderer_id: Option<usize>,
}

impl GuiInputPanel {
    pub fn height(&self) -> f64 {
        self.lines.len() as f64 * 20f64
    }

    pub fn empty(&self) -> bool {
        self.lines.len() == 0
    }

    pub fn push(&mut self, state: Box<dyn Widget + Sync + Send>) {
        self.lines.push(state);
    }

    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            lines: Vec::new(),
            active: true,
            renderer_id: None,
        }
    }
}

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct UiComponent {
    pub panel: GuiInputPanel,
}

impl UiComponent {
    pub fn new(title: &str) -> Self {
        Self {
            panel: GuiInputPanel::new(title),
        }
    }
}

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct DebugPanel {
    pub panel: GuiInputPanel,
}

impl DebugPanel {
    pub fn new(title: &str) -> Self {
        Self {
            panel: GuiInputPanel::new(title),
        }
    }
}
