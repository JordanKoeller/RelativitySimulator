use std::ffi::{CStr, CString};

use imgui::*;

pub enum OverlayLine {
  LabelText(String, String),
  Text(String),
  HLine,
}

pub struct Overlay {
  lines: Vec<OverlayLine>,
  title: String,
}

impl Overlay {
  pub fn new(title: &str) -> Overlay {
    Overlay {
      lines: Vec::new(),
      title: title.to_string(),
    }
  }

  pub fn push(&mut self, line: OverlayLine) {
    self.lines.push(line);
  }

  pub fn height(&self) -> f32 {
    self.lines.len() as f32 * 20f32
  }

  pub fn render(&self, ui: &Ui, y: f32) {
    const DISTANCE: f32 = 10.0;
    let window_pos = [DISTANCE, y];
    let _style = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.3]);
    // let flags = ui.io().config_flags;
    // flags.push(imgui::ConfigFlags::NO_CURSOR_CHANGE);
    Window::new(ui, &ImString::from(self.title.clone()))
      .opened(&mut true)
      // .flags(imgui::ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
      .position(window_pos, Condition::Always)
      .title_bar(false)
      .resizable(true)
      .always_auto_resize(true)
      .movable(false)
      .save_settings(false)
      .build(|| {
        for line in self.lines.iter() {
          match line {
            OverlayLine::HLine => ui.separator(),
            OverlayLine::LabelText(l, t) => ui.label_text(&ImString::from(t.clone()), &ImString::from(l.clone())),
            OverlayLine::Text(t) => ui.text(t),
          }
        }
      });
  }
}