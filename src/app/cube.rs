use cgmath::prelude::*;
use std::ffi::{CStr, CString};

use utils::*;

use renderer::{Drawable, Material, Texture, RenderCommand};
use renderer::{AttributeType, BufferLayout, IndexBuffer, Shader, Uniform, VertexArray, VertexBuffer};

use scene::{Entity, Renderable, Scene};

static TEXTURE_CUBE_VERTICES: [f32; 288] = [
    // positions       // normals        // texture coords
    -0.5f32, -0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32,
    0.5f32, -0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  1.0f32,  0.0f32,
    0.5f32,  0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  1.0f32,  1.0f32,
    0.5f32,  0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  1.0f32,  1.0f32,
   -0.5f32,  0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.0f32,  1.0f32,
   -0.5f32, -0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32,

   -0.5f32, -0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32,
    0.5f32, -0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  1.0f32,  0.0f32,
    0.5f32,  0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  1.0f32,  1.0f32,
    0.5f32,  0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  1.0f32,  1.0f32,
   -0.5f32,  0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,  1.0f32,
   -0.5f32, -0.5f32,  0.5f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32,

   -0.5f32,  0.5f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,
   -0.5f32,  0.5f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32,  1.0f32,
   -0.5f32, -0.5f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0f32,  1.0f32,
   -0.5f32, -0.5f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0f32,  1.0f32,
   -0.5f32, -0.5f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0f32,  0.0f32,
   -0.5f32,  0.5f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,

    0.5f32,  0.5f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,
    0.5f32,  0.5f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32,  1.0f32,
    0.5f32, -0.5f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0f32,  1.0f32,
    0.5f32, -0.5f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0f32,  1.0f32,
    0.5f32, -0.5f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0f32,  0.0f32,
    0.5f32,  0.5f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32,  0.0f32,

   -0.5f32, -0.5f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32,
    0.5f32, -0.5f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  1.0f32,  1.0f32,
    0.5f32, -0.5f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  1.0f32,  0.0f32,
    0.5f32, -0.5f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  1.0f32,  0.0f32,
   -0.5f32, -0.5f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32,  0.0f32,
   -0.5f32, -0.5f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32,

   -0.5f32,  0.5f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32,
    0.5f32,  0.5f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  1.0f32,  1.0f32,
    0.5f32,  0.5f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  1.0f32,  0.0f32,
    0.5f32,  0.5f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  1.0f32,  0.0f32,
   -0.5f32,  0.5f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32,  0.0f32,
   -0.5f32,  0.5f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32
];

static TEXTURE_CUBE_INDICES: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];

pub struct TexturedCube {
  vertex_array: Ref<VertexArray>,
  material: Ref<Material>,
  transform: Ref<Mat4F>
}

impl TexturedCube {
  pub fn new(position: Vec3F, texture: &str) -> TexturedCube {
    let transform = translate(position);
    let layout = BufferLayout::new(vec![AttributeType::Float3, AttributeType::Float3, AttributeType::Float2]);
    let vert_buff = VertexBuffer::create(TEXTURE_CUBE_VERTICES.to_vec(), layout);
    let ind_buff = IndexBuffer::create(TEXTURE_CUBE_INDICES.to_vec());
    let vertex_array = VertexArray::new(vec![vert_buff], ind_buff);
    let mut material = Material::new();
    material.diffuse_texture(Texture::from_file(texture));
    TexturedCube {
      vertex_array: Ref::from(vertex_array),
      material: Ref::from(material),
      transform: Ref::from(transform),
    }
  }
}

impl Drawable for TexturedCube {
  fn vertex_array(&self) -> &Ref<VertexArray> {
    &self.vertex_array
  }
  fn material(&self) -> &Ref<Material> {
    &self.material
  }

  fn transform(&self) -> &Ref<Mat4F> {
    &self.transform
  }

  fn shader_name(&self) -> String {
    "default_texture".to_string()
  }
}

// impl Renderable for TexturedCube {
//   fn draw(&self) -> RenderCommand {
//     RenderCommand::from(self.renderable())
//   }
// }

impl Entity for TexturedCube {
  fn register(self: Box<Self>, scene: &mut Scene) {
    let rend = self as Box<dyn Renderable>;
    let mr = GetMutRef(rend);
    scene.register_renderable(mr);
  }
}