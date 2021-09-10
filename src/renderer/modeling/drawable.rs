use specs::{Component, VecStorage};

use renderer::{VertexArray, Mesh, AttributeType};
use ecs::{Material, MeshComponent};
use ecs::MyBuilder;

pub trait Drawable {
  fn shader_name(&self) -> String {
    "default".to_string()
  }
  fn vertex_array(&self) -> VertexArray;
  fn material(&self) -> Material;

  fn instance_attributes(&self) -> Option<Vec<(String, AttributeType)>> {None}

  fn mesh(&self) -> Mesh {
    if let Some(instance_attributes) = self.instance_attributes() {
      Mesh::new_instanced(self.vertex_array(), self.shader_name(), instance_attributes, 4096)
    } else {
      Mesh::new(self.vertex_array(), self.shader_name())
    }
  }

  fn mesh_component(&self) -> MeshComponent {
    MeshComponent::from(self.mesh())
  }

}