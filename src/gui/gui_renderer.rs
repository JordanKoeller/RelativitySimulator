use imgui::*;
use specs::prelude::*;

use physics::TransformComponent;

use renderer::Window;
use utils::MutRef;

use super::{GuiInputPanel, Widget};

pub struct GuiRenderer {
  pub window: MutRef<Window>,
  // pub mut_refs: Vec<MutRef<Widget>>,
  // pub opened: Vec<MutRef<bool>>,
}

impl<'a> System<'a> for GuiRenderer {
  type SystemData = (
    WriteStorage<'a, GuiInputPanel>,
    ReadStorage<'a, TransformComponent>,
  );

  fn run(&mut self, (mut overlay_store, transform_storage): Self::SystemData) {
    let mut window = self.window.borrow_mut();
    let mut auto_pos = [10f32, 0f32];
    for (overlay, transform_opt) in (&mut overlay_store, transform_storage.maybe()).join() {
      if !overlay.empty() {
        // if let Some(transform) = transform_opt {
        //   let xy: [f32; 2] = [transform.translation.x, transform.translation.y];
        //   self.render_panel(&mut window, &xy, overlay)
        // } else {
          self.render_panel(&mut window, &auto_pos, overlay);
          auto_pos[1] += overlay.height() * 2f32 + 10f32;
        // }
      }
    }
  }

  fn setup(&mut self, _world: &mut World) {

  }
}

impl GuiRenderer {
  fn render_panel(&self, window: &mut Window, window_pos: &[f32; 2], panel: &mut GuiInputPanel) {
    let ui = window.imgui_glfw.frame(&mut window.window, &mut window.im_context);
    // let ui = window.get_ui();
    {
      let _token = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.3]);
    }
    let title = panel.title.clone();
    GuiRenderer::render_panel_helper(&ui, title, window_pos.clone(), &mut panel.lines, &mut panel.active);
    window.imgui_glfw.draw(ui, &mut window.window);
  }

  fn render_panel_helper<'ui>(ui: &imgui::Ui<'ui>, title: String, pos: [f32; 2], lines: &mut Vec<Box<dyn Widget + Sync + Send>>, active: &mut bool) {
    imgui::Window::new(&ui, &ImString::from(title))
    .opened(active)
    .position(pos, Condition::Always)
    .title_bar(true)
    .resizable(true)
    .always_auto_resize(true)
    .movable(true)
    .inputs(true)
    .save_settings(false)
    .build(|| {
      for line in lines.iter_mut() {
        line.render(&ui);
      }
    });
  }

}
