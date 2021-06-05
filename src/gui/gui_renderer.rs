use imgui::*;
use specs::prelude::*;

use renderer::Window;
use utils::MutRef;

use super::{GuiInputPanel, Widget};

pub struct GuiRenderer {
  pub window: MutRef<Window>,
  // pub mut_refs: Vec<MutRef<Widget>>,
  // pub opened: Vec<MutRef<bool>>,
}

impl<'a> System<'a> for GuiRenderer {
  type SystemData = (WriteStorage<'a, GuiInputPanel>,);

  fn run(&mut self, (mut overlay_store,): Self::SystemData) {
    let mut y = 0f32;
    for (overlay,) in (&mut overlay_store,).join() {
      // let pid = self.get_registered_panel(overlay);
      let mut window = self.window.borrow_mut();
      self.render_panel(&mut window, y, overlay);
      y += overlay.height() + 10f32;
    }
  }
}

impl GuiRenderer {
  fn render_panel(&self, window: &mut Window, height: f32, panel: &mut GuiInputPanel) {
    let ui = window.imgui_glfw.frame(&mut window.window, &mut window.im_context);
    let window_pos = [10f32, height];
    // let ui = window.get_ui();
    ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.3]);
    let title = panel.title.clone();
    GuiRenderer::render_panel_helper(&ui, title, window_pos, &mut panel.lines, &mut panel.active);
    window.imgui_glfw.draw(ui, &mut window.window);
  }

  fn render_panel_helper<'ui>(ui: &imgui::Ui<'ui>, title: String, pos: [f32; 2], lines: &mut Vec<Box<dyn Widget + Sync + Send>>, active: &mut bool) {
    imgui::Window::new(&ui, &ImString::from(title))
    .opened(active)
    .position(pos, Condition::Always)
    .title_bar(true)
    .resizable(true)
    .always_auto_resize(true)
    .movable(false)
    .save_settings(false)
    .build(|| {
      for line in lines.iter_mut() {
        line.render(&ui);
      }
    });
  }

}
