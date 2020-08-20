
use std::ffi::CStr;

use gl;

use utils::{Rectangle, Vec2F, Vec3F};
use drawable::Drawable;
use shader_manager::ShaderManager;
// use drawable::Model;

pub struct SimpleBuilding {
    pub position: Vec3F,
    pub footprint: Rectangle,
    pub height: f32,
    drawables: Vec<Box<dyn Drawable>>,
    shader_name: String
}

impl Drawable for SimpleBuilding {
    fn draw(&self, shader_manager: &ShaderManager) {
        for d in self.drawables.iter() {
            d.draw(shader_manager);
        }
    }

    fn shader_name(&self) -> String {
        self.shader_name
    }
}