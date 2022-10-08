use imgui::*;

use specs::Read;
use std::collections::HashMap;
use std::sync::RwLock;

use super::widgets::Widget;
use crate::datastructures::{GenericRegistry, Registry, RegistryItem};
use crate::utils::{Vec2F, Vec3F};
use std::any::TypeId;

pub struct ControlPanel {
  lines: Vec<(String, Box<dyn Widget>)>,
  title: ImString,
}

impl ControlPanel {
  pub fn get_float(&self, name: &str) -> f32 {
    self.get_by_name(name).get_float()
  }

  pub fn get_int(&self, name: &str) -> i32 {
    self.get_by_name(name).get_int()
  }

  pub fn get_vec2(&self, name: &str) -> Vec2F {
    self.get_by_name(name).get_vec2()
  }

  pub fn get_vec3(&self, name: &str) -> Vec3F {
    self.get_by_name(name).get_vec3()
  }

  pub fn get_str(&self, name: &str) -> String {
    self.get_by_name(name).get_string()
  }

  pub fn set_float(&mut self, name: &str, value: f32) {
    self.get_by_name_mut(name).set_float(value);
  }

  pub fn set_int(&mut self, name: &str, value: i32) {
    self.get_by_name_mut(name).set_int(value);
  }

  pub fn set_vec2(&mut self, name: &str, value: Vec2F) {
    self.get_by_name_mut(name).set_vec2(value);
  }

  pub fn set_vec3(&mut self, name: &str, value: Vec3F) {
    self.get_by_name_mut(name).set_vec3(value);
  }

  pub fn set_str(&mut self, name: &str, value: String) {
    self.get_by_name_mut(name).set_string(value);
  }

  pub fn render<'ui>(&mut self, ui: &imgui::Ui<'ui>, pos: &[f32; 2]) {
    let pos_f32 = [pos[0] as f32, pos[1] as f32];
    imgui::Window::new(ui, &self.title)
      .opened(&mut true)
      .position(pos_f32, Condition::Always)
      .title_bar(true)
      .resizable(true)
      .always_auto_resize(true)
      .movable(false)
      .save_settings(false)
      .build(|| {
        for (_name, line) in self.lines.iter_mut() {
          line.render(&ui)
        }
      });
  }

  pub fn height(&self) -> u32 {
    10u32 + self.lines.len() as u32 * 40u32
  }

  fn get_by_name(&self, name: &str) -> &Box<dyn Widget> {
    for i in 0..self.lines.len() {
      if self.lines[i].0 == name {
        return &self.lines[i].1;
      }
    }
    panic!("Could not find widget with name {}", name);
  }

  fn get_by_name_mut(&mut self, name: &str) -> &mut Box<dyn Widget> {
    for i in 0..self.lines.len() {
      if self.lines[i].0 == name {
        return &mut self.lines[i].1;
      }
    }
    panic!("Could not find widget with name {}", name);
  }
}

unsafe impl Send for ControlPanel {}
unsafe impl Sync for ControlPanel {}

#[derive(Default)]
pub struct ControlPanelBuilder {
  lines: Vec<(String, Box<dyn Widget>)>,
  title: String,
}

impl ControlPanelBuilder {
  pub fn push_line<W: Widget + 'static>(mut self, name: &str, line: W) -> Self {
    self.lines.push((name.to_string(), Box::from(line)));
    self
  }

  pub fn with_title(mut self, title: &str) -> Self {
    self.title = title.to_string();
    self
  }

  pub fn build(self) -> ControlPanel {
    ControlPanel {
      title: ImString::from(self.title),
      lines: self.lines,
    }
  }
}

#[derive(Default)]
pub struct ControlPanels {
  lookup: HashMap<TypeId, (usize, RwLock<ControlPanel>)>,
  ordering: Vec<TypeId>,
}

impl ControlPanels {
  pub fn add(&mut self, id: TypeId, builder: ControlPanelBuilder) {
    if self.lookup.contains_key(&id) {
      // TypeId Already present, so rebuild panel and insert.
      self
        .lookup
        .insert(id.clone(), (self.ordering.len(), RwLock::from(builder.build())));
    } else {
      self
        .lookup
        .insert(id.clone(), (self.ordering.len(), RwLock::from(builder.build())));
      self.ordering.push(id)
    }
  }

  pub fn get(&self, id: &TypeId) -> Option<&RwLock<ControlPanel>> {
    self.lookup.get(id).map(|tup| &tup.1)
  }

  pub fn iter(&self) -> impl Iterator<Item = &RwLock<ControlPanel>> {
    self
      .ordering
      .iter()
      .filter_map(|k| self.lookup.get(k).map(|sz_lock| &sz_lock.1))
  }
}

// pub type ControlPanels<'a> = Read<'a, HashMap<TypeId, RwLock<ControlPanel>>>;
