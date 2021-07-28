use specs::prelude::*;
use specs::{Component, VecStorage};
use imgui::{ImString, Ui, SliderFloat};
use utils::{SyncMutRef, getSyncMutRef, Color};

/******************************
 * Widget Interface
 ******************************/
pub trait Widget {
  fn render<'ui>(&mut self, ui: &imgui::Ui<'ui>);

  fn get_float(&self) -> f32 {
    panic!("Widget::get_float not implemented for {}", std::any::type_name::<Self>());
  }
  fn get_bool(&self) -> bool {
    panic!("Widget::get_bool not implemented for {}", std::any::type_name::<Self>());
  }
  fn get_color(&self) -> Color {
    panic!("Widget::get_color not implemented for {}", std::any::type_name::<Self>());
  }
  fn get_string(&self) -> String {
    panic!("Widget::get_string not implemented for {}", std::any::type_name::<Self>());
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

pub struct LabeledInputTextBox {
  label: ImString,
  buffer: ImString,
  size: [f32; 2]
}
impl LabeledInputTextBox {
  pub fn new(text: &str, default: &str) -> Self {
    let mut buffer = ImString::from(default.to_string());
    buffer.reserve(200);
    let im_label = ImString::from(text.to_string());
    let size = [300f32, 300f32];
    Self {
      label: im_label,
      buffer,
      size
    }
  }
}
impl Widget for LabeledInputTextBox {
  fn render<'ui> (&mut self, ui: &Ui<'ui>) {
    ui.input_text_multiline(&self.label, &mut self.buffer, self.size.clone()).build();
  }

  fn get_string(&self) -> String {
    self.buffer.to_str().to_string()
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
    let slider = SliderFloat::new(ui, &self.label, &mut self.value, 0f32, 10f32);
    slider.build();
      // ui.input_float(&self.label, &mut self.value).build();
  }
  fn get_float(&self) -> f32 {
    self.value
  }
}
// OverlayLine::IntInput(label, value) => {
//   let v = ImString::from(label.clone());
//   let slider = SliderInt::new(ui, &v, value, 0, 100);
//   slider.build();
// }

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

  fn get_color(&self) -> Color {
    Color::new(self.value[0], self.value[1], self.value[2])
  }
}

pub struct Button {label: ImString, clicked: bool}
impl Widget for Button {
  fn render<'ui> (&mut self, ui: &Ui<'ui>) {
    let clicked = ui.button(&self.label, [80f32, 30f32]);
    self.clicked = clicked;
  }

  fn get_bool(&self) -> bool {
    self.clicked
  }
}

impl Button {
  pub fn new(label: &str) -> Self {
    Self {
      label: ImString::from(label.to_string()),
      clicked: false,
    }
  }
}