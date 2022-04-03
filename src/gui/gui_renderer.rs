use imgui::*;
use specs::prelude::*;

use physics::TransformComponent;

use renderer::Window;
use utils::MutRef;

use super::{DebugPanel, GuiInputPanel, UiComponent, Widget};

pub struct GuiRenderer {
    pub window: MutRef<Window>,
}

impl<'a> System<'a> for GuiRenderer {
    type SystemData = (
        WriteStorage<'a, DebugPanel>,
        WriteStorage<'a, UiComponent>,
        ReadStorage<'a, TransformComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let mut window = self.window.borrow_mut();
        self.run_helper(&mut window, data)
    }
    fn setup(&mut self, world: &mut World) {
        world.register::<DebugPanel>();
        world.register::<UiComponent>();
    }
}

impl GuiRenderer {
    fn render_panel<'ui>(&self, window_pos: &[f32; 2], panel: &mut GuiInputPanel, ui: &imgui::Ui<'ui>) {
        let title = panel.title.clone();
        GuiRenderer::render_panel_helper(&ui, title, window_pos.clone(), &mut panel.lines, &mut panel.active);
    }
    fn run_helper(
        &self,
        window: &mut Window,
        (mut debugger_storage, mut overlay_store, transform_storage): <Self as System>::SystemData,
    ) {
        let mut auto_pos = [10f32, 0f32];
        let ui = window.imgui_glfw.frame(&mut window.window, &mut window.im_context);
        {
            let _ = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.3]);
        }
        for (overlay, transform_opt) in (&mut overlay_store, transform_storage.maybe()).join() {
            if !overlay.panel.empty() {
                // if let Some(transform) = transform_opt {
                //   let xy: [f32; 2] = [transform.translation.x, transform.translation.y];
                //   self.render_panel(&xy, &mut overlay.panel, &ui);
                // } else {
                self.render_panel(&auto_pos, &mut overlay.panel, &ui);
                auto_pos[1] += overlay.panel.height() * 2f32 + 10f32;
                // }
            }
        }
        for (overlay, transform_opt) in (&mut debugger_storage, transform_storage.maybe()).join() {
            if !overlay.panel.empty() {
                // if let Some(transform) = transform_opt {
                //   let xy: [f32; 2] = [transform.translation.x, transform.translation.y];
                //   self.render_panel(&xy, &mut overlay.panel, &ui);
                // } else {
                self.render_panel(&auto_pos, &mut overlay.panel, &ui);
                auto_pos[1] += overlay.panel.height() * 2f32 + 10f32;
                // }
            }
        }
        window.imgui_glfw.draw(ui, &mut window.window);
    }

    fn render_panel_helper<'ui>(
        ui: &imgui::Ui<'ui>,
        title: String,
        pos: [f32; 2],
        lines: &mut Vec<Box<dyn Widget + Sync + Send>>,
        active: &mut bool,
    ) {
        imgui::Window::new(&ui, &ImString::from(title))
            .opened(active)
            .position(pos, Condition::Always)
            .title_bar(false)
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
