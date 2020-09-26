use cgmath::prelude::*;
use std::ffi::{CStr, CString};

use utils::*;

use renderer::{Drawable, Material};
use renderer::{AttributeType, BufferLayout, IndexBuffer, Shader, Uniform, VertexArray, VertexBuffer};

const CUBE_VERTICES: [f32; 24] = [
  -0.5, -0.5, -0.5, //ftl 0
  0.5, -0.5, -0.5, //ftr 1
  -0.5, 0.5, -0.5, //fbl 2
  0.5, 0.5, -0.5, //fbr 3
  -0.5, -0.5, 0.5, //btl 4
  0.5, -0.5, 0.5, //btr 5
  -0.5, 0.5, 0.5, //bbl 6
  0.5, 0.5,
  0.5, //bbr 7

       //   0.5,  0.5, 0.0,  // top right
       //   0.5, -0.5, 0.0,  // bottom right
       //  -0.5, -0.5, 0.0,  // bottom left
       //  -0.5,  0.5, 0.0   // top left
];

const CUBE_INDICES: [u32; 36] = [
  // front face
  0, 1, 2, 1, 3, 2, // left face
  0, 4, 6, 6, 2, 0, // right face
  1, 5, 7, 1, 7, 3, // top face
  0, 1, 5, 0, 5, 4, // bottom face
  2, 3, 7, 2, 7, 6, // back face
  4, 5, 6, 6, 7, 6,
  // 0, 1, 3,  // first Triangle
  // 1, 2, 3   // second Triangle
];

pub struct ColoredCube {
  vertex_array: VertexArray,
  uniforms: Vec<(CString, Uniform)>,
}

impl ColoredCube {
  pub fn new(position: Vec3F, color: Color) -> ColoredCube {
    let transform = translate(position);
    let layout = BufferLayout::new(vec![AttributeType::Float3]);
    let vert_buff = VertexBuffer::create(CUBE_VERTICES.to_vec(), layout);
    let ind_buff = IndexBuffer::create(CUBE_INDICES.to_vec());
    let vertex_array = VertexArray::new(vec![vert_buff], ind_buff);
    let uniforms = vec![
      (CString::new("model").unwrap(), Uniform::Mat4(transform)),
      (CString::new("color").unwrap(), Uniform::Vec3(color)),
    ];
    ColoredCube { vertex_array, uniforms }
  }
}

impl Drawable for ColoredCube {
  fn vertex_array(&self) -> &VertexArray {
    &self.vertex_array
  }
  fn material(&self) -> &Material {
    &self.uniforms
  }
}
