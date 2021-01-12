
use imgui::*;

#[allow(dead_code)]
pub enum OverlayLine {
  // Labels and stuff
  LabelText(String, String),
  Text(String),
  HLine,

  // Input components
  FloatInput(String, f32),
  IntInput(String, i32),
  BoolInput(String, bool),
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

  pub fn render(&mut self, ui: &Ui, y: f32) {
    const DISTANCE: f32 = 10.0;
    let window_pos = [DISTANCE, y];
    let _style = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.3]);
    Window::new(ui, &ImString::from(self.title.clone()))
      .opened(&mut true)
      .position(window_pos, Condition::Always)
      .title_bar(false)
      .resizable(true)
      .always_auto_resize(true)
      .movable(false)
      .save_settings(false)
      .build(|| {
        for line in self.lines.iter_mut() {
          match line {
            OverlayLine::HLine => ui.separator(),
            OverlayLine::LabelText(l, t) => {
              ui.label_text(&ImString::from(t.clone()), &ImString::from(l.clone()));
            },
            OverlayLine::Text(t) => ui.text(t),
            // OverlayLine::Text(t) => {

            //   // let slider = imgui::internal::Slider::<u8>::new(&ImString::from(t.clone()));
            //   // slider.build(ui, &self.v)
            // }
            OverlayLine::IntInput(label, value) => {
              let v = ImString::from(label.clone());
              let slider = SliderInt::new(ui, &v, value, 0, 100);
              slider.build();
            }
            _ => {}
          }
        }
      });
  }
}