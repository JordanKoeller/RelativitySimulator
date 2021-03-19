use specs::{Component, VecStorage};

use renderer::{Material, VertexArray};

pub trait Drawable {
  fn shader_name(&self) -> String {
    "default".to_string()
  }
  fn vertex_array(&self) -> VertexArray;
  fn material(&self) -> Material;

  fn state(&self) -> DrawableState {
    DrawableState {
      vertex_array: self.vertex_array(),
      material: self.material(),
      shader_name: self.shader_name(),
    }
  }
}

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct DrawableState {
  pub shader_name: String,
  pub vertex_array: VertexArray,
  pub material: Material,
}

impl DrawableState {
  #[allow(dead_code)]
  pub fn new(vao: VertexArray, material: Material) -> DrawableState {
    DrawableState {
      shader_name: "default".to_string(),
      vertex_array: vao,
      material: material,
    }
  }

  pub fn new_textured(vao: VertexArray, material: Material) -> DrawableState {
    DrawableState {
      shader_name: "default_texture".to_string(),
      vertex_array: vao,
      material: material,
    }
  }

  pub fn refresh(&mut self) {
    self.vertex_array.refresh();
    self.material.refresh();
  }
}

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct DrawableId(pub usize);
