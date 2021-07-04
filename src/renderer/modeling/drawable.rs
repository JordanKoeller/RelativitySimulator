use specs::{Component, VecStorage};

use renderer::{VertexArray, Mesh};
use ecs::{Material, MeshComponent};
use ecs::MyBuilder;

pub trait Drawable {
  fn shader_name(&self) -> String {
    "default".to_string()
  }
  fn vertex_array(&self) -> VertexArray;
  fn material(&self) -> Material;

  fn mesh(&self) -> Mesh {
    Mesh::new(self.vertex_array(), self.shader_name())
  }

  fn mesh_component(&self) -> MeshComponent {
    MeshComponent::new(self.vertex_array(), self.shader_name())
  }

}