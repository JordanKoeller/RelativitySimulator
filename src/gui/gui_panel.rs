use specs::prelude::*;
use super::Widget;

pub struct GuiInputPanel {
  pub title: String,
  pub lines: Vec<Box<dyn Widget + Sync + Send>>,
  pub active: bool,
  pub renderer_id: Option<usize>,
}

impl Component for GuiInputPanel {
  type Storage = VecStorage<Self>;
}


impl GuiInputPanel {
  pub fn height(&self) -> f32 {
    self.lines.len() as f32 * 20f32
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
      renderer_id: None
    }
  }
}