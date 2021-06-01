use specs::prelude::*;
use specs::{Component, VecStorage};
use imgui::{ImString, Ui};
use utils::{SyncMutRef, GetSyncMutRef, Color};

/******************************
 * Widget Interface
 ******************************/
pub trait Widget {
  fn render<'ui>(&mut self, ui: &imgui::Ui<'ui>);

  fn get_float(&self) -> Option<f32> {
    None
  }
}


/******************************
 * Widget Implementors
 ******************************/
pub struct LineBreak;
impl Widget for LineBreak {
  fn render<'ui>(&mut self, ui: &imgui::Ui<'ui>) {ui.separator();}
}

pub struct InputText {text: ImString, label: ImString}
impl InputText {
  pub fn new(text: &str, label: &str) -> Self {
    Self {
      text: ImString::from(text.to_string()),
      label: ImString::from(label.to_string()),
    }
  }
}
impl Widget for InputText {
  fn render<'ui> (&mut self, ui: &Ui<'ui>) {
      ui.input_text(&self.label, &mut self.text).build();
  }
}

pub struct LabeledText {text: ImString, label: ImString}
impl LabeledText {
  pub fn new(text: &str, label: &str) -> Self {
    Self {
      text: ImString::from(text.to_string()),
      label: ImString::from(label.to_string()),
    }
  }
}
impl Widget for LabeledText {
  fn render<'ui> (&mut self, ui: &Ui<'ui>) {
      ui.label_text(&self.label, &self.text);
  }
}

pub struct InputFloat {value: f32, label: ImString}
impl InputFloat {
  pub fn new(label: &str, value: f32) -> Self {
    Self {
      value: value,
      label: ImString::from(label.to_string()),
    }
  }
}
impl Widget for InputFloat {
  fn render<'ui> (&mut self, ui: &Ui<'ui>) {
      ui.input_float(&self.label, &mut self.value).build();
  }
  fn get_float(&self) -> Option<f32> {
    Some(self.value)
  }
}

pub struct InputColor {label: ImString, value: [f32; 3]}
impl InputColor {
  pub fn new(label: &str) -> Self {
    Self {
      label: ImString::from(label.to_string()),
      value: [1f32, 0f32, 0f32],
    }
  }
}
impl Widget for InputColor {
  fn render<'ui>(&mut self, ui: &Ui<'ui>) {
    ui.color_picker(&self.label, &mut self.value).build();
  }
}