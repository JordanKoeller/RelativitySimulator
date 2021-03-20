use super::{Overlay, OverlayLine};
use renderer::{Camera, Window, RendererConfig};
use utils::Timestep;

pub struct UiRenderer {
  overlays: Vec<Overlay>,
}

impl Default for UiRenderer {
  fn default() -> Self {
    Self {
      overlays: Vec::with_capacity(10),
    }
  }
}

impl UiRenderer {

  pub fn submit_2d(&mut self, overlay: Overlay) {
    self.overlays.push(overlay);
  }

  pub fn new_overlay(&self, title: &str)  -> Overlay {
    Overlay::new(title)
  }

  pub fn clear(&mut self) {
    self.overlays.clear();
  }

  pub fn draw(&mut self, window: &mut Window) {
    let mut y = 10f32;
    for i in 0..self.overlays.len() {
      let overlay = &mut self.overlays[i];
      let ui = window.imgui_glfw.frame(&mut window.window, &mut window.im_context);
      overlay.render(&ui, y.clone());
      window.imgui_glfw.draw(ui, &mut window.window);
      y += overlay.height() + 10f32;
    }
  }

  pub fn add_diagnostics_pannel<'a>(&mut self, camera: Camera<'a>, timestep: &Timestep, config: &RendererConfig) {

    let mut overlay = self.new_overlay("Camera Uniforms");
    overlay.push(OverlayLine::HLine);
    overlay.push(OverlayLine::LabelText(
      "Position".to_string(),
      to_string!(camera.position),
    ));
    // overlay.push(OverlayLine::LabelText("Front".to_string(), to_string!(camera.front())));
    overlay.push(OverlayLine::LabelText(
      "Beta:".to_string(),
      format!("{0:.3}", camera.beta()),
    ));
    overlay.push(OverlayLine::LabelText(
      "Gamma:".to_string(),
      format!("{0:.3}", camera.gamma()),
    ));
    overlay.push(OverlayLine::LabelText(
      "Render Mode".to_string(),
      format!("{:?}", config.mode),
    ));
    overlay.push(OverlayLine::LabelText(
      "Debug On:".to_string(),
      format!("{:?}", config.debug),
    ));
    overlay.push(OverlayLine::IntInput(
      format!("Frame Time {:.4}", timestep.0).to_string(),
      (timestep.0 * 1000.0) as i32,
    ));
    overlay.push(OverlayLine::IntInput(
      format!("Render Time {:.4}", timestep.1).to_string(),
      (timestep.1 * 1000.0) as i32,
    ));
    self.submit_2d(overlay);
  }

}