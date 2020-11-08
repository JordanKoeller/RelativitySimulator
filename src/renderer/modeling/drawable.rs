use renderer::{Material, Texture, Uniform, VertexArray};
use std::ffi::CString;
use utils::*;

use ecs::components::DrawableMemo;

pub trait Drawable {
  fn shader_name(&self) -> String {
    "default".to_string()
  }
  fn vertex_array(&self) -> &VertexArray;
  fn material(&self) -> &Material;

  fn renderable(&self) -> DrawableMemo {
    DrawableMemo {
      vertex_array: self.vertex_array().clone(),
      material: self.material().clone(),
      shader_id: self.shader_name(),
      transform: None,
    }
  }
}


pub struct DefaultDrawable {
  shader_name: String,
  vertex_array: VertexArray,
  material: Material,
}

impl DefaultDrawable {
  pub fn new(vao: VertexArray, material: Material) -> DefaultDrawable {
    DefaultDrawable {
      shader_name: "default".to_string(),
      vertex_array: vao,
      material: material,
    }
  }

  pub fn new_textured(vao: VertexArray, material: Material) -> DefaultDrawable {
    DefaultDrawable {
      shader_name: "default_texture".to_string(),
      vertex_array: vao,
      material: material,
    }
  }
}

impl Drawable for DefaultDrawable {
  fn shader_name(&self) -> String {
    self.shader_name.clone()
  }
  fn vertex_array(&self) -> &VertexArray {
    &self.vertex_array
  }
  fn material(&self) -> &Material {
    &self.material
  }
}
