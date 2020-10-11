use renderer::{Material, RenderCommand, Texture, Uniform, VertexArray};
use std::ffi::CString;
use utils::*;

use scene::{Entity, Renderable, Scene};
// pub type Material = Vec<(CString, Uniform)>;

#[derive(Clone, Debug)]
pub struct DrawableMemo {
  pub vertex_array: Ref<VertexArray>,
  pub material: Ref<Material>,
  pub shader_name: String,
  pub transform: Ref<Mat4F>,
}

pub trait Drawable: Renderable {
  fn shader_name(&self) -> String {
    "default".to_string()
  }
  fn vertex_array(&self) -> &Ref<VertexArray>;
  fn material(&self) -> &Ref<Material>;
  fn transform(&self) -> &Ref<Mat4F>;

  fn renderable(&self) -> DrawableMemo {
    DrawableMemo {
      vertex_array: Ref::clone(self.vertex_array()),
      material: Ref::clone(self.material()),
      shader_name: self.shader_name(),
      transform: Ref::clone(self.transform()),
    }
  }
}

impl<T: Drawable> Renderable for T {
  fn draw(&self) -> RenderCommand {
    RenderCommand::SingleDrawable(self.renderable())
  }
}


pub struct DefaultDrawable {
  shader_name: String,
  vertex_array: Ref<VertexArray>,
  material: Ref<Material>,
  transform: Ref<Mat4F>,
}

impl DefaultDrawable {
  pub fn new(vao: VertexArray, material: Ref<Material>, transform: Mat4F) -> DefaultDrawable {
    DefaultDrawable {
      shader_name: "default".to_string(),
      vertex_array: Ref::from(vao),
      material: material,
      transform: Ref::from(transform),
    }
  }

  pub fn new_textured(vao: VertexArray, material: Ref<Material>, transform: Mat4F) -> DefaultDrawable {
    DefaultDrawable {
      shader_name: "default_texture".to_string(),
      vertex_array: Ref::from(vao),
      material: material,
      transform: Ref::from(transform),
    }
  }
}

impl Drawable for DefaultDrawable {
  fn shader_name(&self) -> String {
    self.shader_name.clone()
  }
  fn vertex_array(&self) -> &Ref<VertexArray> {
    &self.vertex_array
  }
  fn material(&self) -> &Ref<Material> {
    &self.material
  }

  fn transform(&self) -> &Ref<Mat4F> {
    &self.transform
  }
}

impl Entity for DefaultDrawable {
  fn register(self: Box<Self>, scene: &mut Scene) {
    let rend = self as Box<dyn Renderable>;
    let mr = GetMutRef(rend);
    scene.register_renderable(mr);
  }
}