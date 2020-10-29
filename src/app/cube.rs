
use utils::*;

use renderer::{Drawable, Material, Texture};
use renderer::{AttributeType, BufferLayout, IndexBuffer, VertexArray, VertexBuffer};


pub static TEXTURE_CUBE_VERTICES: [f32; 288] = [
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

pub static TEXTURE_CUBE_INDICES: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];

pub struct TexturedCube {
  vertex_array: Ref<VertexArray>,
  material: Ref<Material>,
}

impl TexturedCube {
  pub fn new(texture: &str) -> TexturedCube {
    let layout = BufferLayout::new(vec![AttributeType::Float3, AttributeType::Float3, AttributeType::Float2]);
    let vert_buff = VertexBuffer::create(TEXTURE_CUBE_VERTICES.to_vec(), layout);
    let ind_buff = IndexBuffer::create(TEXTURE_CUBE_INDICES.to_vec());
    let vertex_array = VertexArray::new(vec![vert_buff], ind_buff);
    let mut material = Material::new();
    material.diffuse_texture(Texture::from_file(texture));
    TexturedCube {
      vertex_array: Ref::from(vertex_array),
      material: Ref::from(material),
    }
  }
}

impl Drawable for TexturedCube {
  fn vertex_array(&self) -> &VertexArray {
    &self.vertex_array
  }
  fn material(&self) -> &Material {
    &self.material
  }

  fn shader_name(&self) -> String {
    "lorentz".to_string()
  }
}


pub struct Cube {
  vertex_array: VertexArray,
  material: Material,
}

impl Cube {
  pub fn new(material: Material) -> Cube {
    let layout = BufferLayout::new(vec![AttributeType::Float3, AttributeType::Float3, AttributeType::Float2]);
    let vert_buff = VertexBuffer::create(TEXTURE_CUBE_VERTICES.to_vec(), layout);
    let ind_buff = IndexBuffer::create(TEXTURE_CUBE_INDICES.to_vec());
    let vertex_array = VertexArray::new(vec![vert_buff], ind_buff);
    Cube {
      vertex_array,
      material,
    }
  }
}

impl Drawable for Cube {
  fn vertex_array(&self) -> &VertexArray {
    &self.vertex_array
  }
  fn material(&self) -> &Material {
    &self.material
  }

  fn shader_name(&self) -> String {
    "lorentz".to_string()
  }
}