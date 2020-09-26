use std::ffi::CString;

use renderer::{Uniform, VertexArray, Texture};

pub type Material = Vec<(CString, Uniform)>;
pub type Textures = Vec<(CString, Texture)>;

pub trait Drawable {
  fn shader_name(&self) -> &str {
    "default"
  }
  fn vertex_array(&self) -> &VertexArray;
  fn material(&self) -> &Material;
  fn textures(&self) -> Option<&Textures> {
    None
  }
}

pub struct DefaultDrawable {
  shader_name: String,
  vertex_array: VertexArray,
  material: Material,
  textures: Textures,
}

impl DefaultDrawable {
  pub fn new(vao: VertexArray, material: Material) -> DefaultDrawable {
    DefaultDrawable {
      shader_name: "default".to_string(),
      vertex_array: vao,
      material: material,
      textures: Vec::new(),
    }
  }

  pub fn new_textured(vao: VertexArray, material: Material, textures: Textures) -> DefaultDrawable {
    DefaultDrawable {
      shader_name: "default".to_string(),
      vertex_array: vao,
      material: material,
      textures: textures
    }
  }
}

impl Drawable for DefaultDrawable {
  fn shader_name(&self) -> &str {
    &self.shader_name
  }
  fn vertex_array(&self) -> &VertexArray {
    &self.vertex_array
  }
  fn material(&self) -> &Material {
    &self.material
  }

  fn textures(&self) -> Option<&Textures> {
    Some(&self.textures)
  }
}
