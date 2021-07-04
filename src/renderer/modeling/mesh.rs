use renderer::platform::{VertexArray, ShaderId};
use ecs::DrawableId;


#[derive(Debug, Clone)]
pub struct Mesh {
  pub vao: VertexArray,
  pub shader_name: String,
  pub registry: Option<DrawableId>,
}

impl Mesh {
  pub fn new(vao: VertexArray, shader_name: String) -> Self {
    Self {
      vao, shader_name, registry: None,
    }
  }

  pub fn refresh(&mut self) {
    self.vao.refresh();
  }
}