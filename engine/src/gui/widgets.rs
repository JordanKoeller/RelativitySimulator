use crate::utils::{getSyncMutRef, Color, SyncMutRef, Vec2F, Vec3F};
use imgui::{ImString, SliderFloat, Ui};
use specs::prelude::*;
use specs::{Component, VecStorage};

/******************************
 * Widget Interface
 ******************************/
pub trait Widget {
    fn render<'ui>(&mut self, ui: &imgui::Ui<'ui>);

    fn get_float(&self) -> f32 {
        panic!(
            "Widget::get_float not implemented for {}",
            std::any::type_name::<Self>()
        );
    }
    fn get_int(&self) -> i32 {
        panic!("Widget::get_int not implemented for {}", std::any::type_name::<Self>());
    }
    fn get_bool(&self) -> bool {
        panic!("Widget::get_bool not implemented for {}", std::any::type_name::<Self>());
    }
    fn get_vec2(&self) -> Vec2F {
        panic!("Widget::get_vec2 not implemented for {}", std::any::type_name::<Self>());
    }
    fn get_vec3(&self) -> Vec3F {
        panic!("Widget::get_vec3 not implemented for {}", std::any::type_name::<Self>());
    }
    fn get_string(&self) -> String {
        panic!(
            "Widget::get_string not implemented for {}",
            std::any::type_name::<Self>()
        );
    }
    fn set_float(&mut self, _value: f32) {
        panic!(
            "Widget::set_float not implemented for {}",
            std::any::type_name::<Self>()
        );
    }
    fn set_int(&mut self, _value: i32) {
        panic!("Widget::set_int not implemented for {}", std::any::type_name::<Self>());
    }
    fn set_bool(&mut self, _value: bool) {
        panic!("Widget::set_bool not implemented for {}", std::any::type_name::<Self>());
    }
    fn set_vec2(&mut self, _value: Vec2F) {
        panic!("Widget::set_vec2 not implemented for {}", std::any::type_name::<Self>());
    }
    fn set_vec3(&mut self, _value: Vec3F) {
        panic!("Widget::set_vec3 not implemented for {}", std::any::type_name::<Self>());
    }
    fn set_string(&mut self, _value: String) {
        panic!(
            "Widget::set_string not implemented for {}",
            std::any::type_name::<Self>()
        );
    }
}

/******************************
 * Widget Implementors
 ******************************/
pub struct LineBreak;
impl Widget for LineBreak {
    fn render<'ui>(&mut self, ui: &imgui::Ui<'ui>) {
        ui.separator();
    }
}

pub struct InputText {
    text: ImString,
    label: ImString,
}
impl InputText {
    pub fn new(text: &str, label: &str) -> Self {
        Self {
            text: ImString::from(text.to_string()),
            label: ImString::from(label.to_string()),
        }
    }
}
impl Widget for InputText {
    fn render<'ui>(&mut self, ui: &Ui<'ui>) {
        ui.input_text(&self.label, &mut self.text).build();
    }
}

pub struct LabeledText {
    text: ImString,
    label: ImString,
}
impl LabeledText {
    pub fn new(text: &str, label: &str) -> Self {
        Self {
            text: ImString::from(text.to_string()),
            label: ImString::from(label.to_string()),
        }
    }
}
impl Widget for LabeledText {
    fn render<'ui>(&mut self, ui: &Ui<'ui>) {
        ui.label_text(&self.text, &self.label);
    }

    fn set_string(&mut self, text: String) {
        self.text = ImString::from(text.to_string());
    }
}

pub struct LabeledInputTextBox {
    label: ImString,
    buffer: ImString,
    size: [f32; 2],
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
            size,
        }
    }
}
impl Widget for LabeledInputTextBox {
    fn render<'ui>(&mut self, ui: &Ui<'ui>) {
        ui.input_text_multiline(&self.label, &mut self.buffer, self.size.clone())
            .build();
    }

    fn get_string(&self) -> String {
        self.buffer.to_str().to_string()
    }
}

pub struct InputFloat {
    value: f32,
    label: ImString,
    low_value: f32,
    high_value: f32,
}
impl InputFloat {
    pub fn new(label: &str, value: f32) -> Self {
        Self {
            value: value as f32,
            label: ImString::from(label.to_string()),
            low_value: 0f32,
            high_value: 1f32,
        }
    }

    pub fn new_with_limits(label: &str, value: f32, low_value: f32, high_value: f32) -> Self {
        Self {
            value: value as f32,
            label: ImString::from(label.to_string()),
            low_value: low_value as f32,
            high_value: high_value as f32,
        }
    }
}
impl Widget for InputFloat {
    fn render<'ui>(&mut self, ui: &Ui<'ui>) {
        let slider = SliderFloat::new(ui, &self.label, &mut self.value, self.low_value, self.high_value);
        slider.build();
        // ui.input_float(&self.label, &mut self.value).build();
    }
    fn get_float(&self) -> f32 {
        self.value as f32
    }
}
// OverlayLine::IntInput(label, value) => {
//   let v = ImString::from(label.clone());
//   let slider = SliderInt::new(ui, &v, value, 0, 100);
//   slider.build();
// }

pub struct InputColor {
    label: ImString,
    value: [f32; 3],
}
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

    fn get_vec3(&self) -> Vec3F {
        Vec3F::new(self.value[0] as f32, self.value[1] as f32, self.value[2] as f32)
    }
}

pub struct Button {
    label: ImString,
    clicked: bool,
}
impl Widget for Button {
    fn render<'ui>(&mut self, ui: &Ui<'ui>) {
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
