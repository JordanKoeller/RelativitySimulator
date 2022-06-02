use imgui::*;
use specs::prelude::*;

use crate::physics::TransformComponent;

use crate::platform::Window;
use crate::utils::MutRef;

use super::{ControlPanels, Widget};

pub struct GuiRenderer {
    pub window: MutRef<Window>,
}

impl<'a> System<'a> for GuiRenderer {
    type SystemData = Read<'a, ControlPanels>;

    fn run(&mut self, data: Self::SystemData) {
        let mut window = self.window.borrow_mut();
        self.run_helper(&mut window, data)
    }
}

impl GuiRenderer {
    fn run_helper(&self, window: &mut Window, panels: <Self as System>::SystemData) {
        let mut auto_pos = [10f64, 10f64];
        let ui = window.imgui_glfw.frame(&mut window.window, &mut window.im_context);
        {
            let _ = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.3]);
        }
        for panel in panels.iter() {
            panel
                .write()
                .map(|mut p| {
                    p.render(&ui, &auto_pos);
                    auto_pos[1] += p.height() as f64;
                })
                .expect("Could not unlock control panel mutex");
        }
        window.imgui_glfw.draw(ui, &mut window.window);
    }
}
