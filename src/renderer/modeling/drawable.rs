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
      shader_name: self.shader_name()
    }
  }
}

pub struct DrawableState {
  pub shader_name: String,
  pub vertex_array: VertexArray,
  pub material: Material,
}

impl DrawableState {
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
}

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct DrawableId(pub usize);