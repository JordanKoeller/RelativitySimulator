use utils::*;
use cgmath::Matrix;
use std::ffi::c_void;

use renderer::{Texture, CubeMap};


#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Uniform {
    Int(i32),
    IntArray(Vec<i32>),
    Float(f32),
    Vec2(Vec2F),
    Vec3(Vec3F),
    Vec4(Vec4F),
    Mat3(Mat3F),
    Mat4(Mat4F),
    Bool(bool),
    Texture(Texture),
    CubeMap(CubeMap),
    UniformBuffer(UniformBuffer),
}

impl Uniform {
  pub unsafe fn serialize_into(&self, collector: &mut [f32]) {
    match self {
      Uniform::Int(elem) => collector[0] = *(elem as *const i32 as *const c_void as *const f32),
      Uniform::Float(elem) => collector[0] = *elem,
      Uniform::Vec2(v) => {
        collector[0] = v.x;
        collector[1] = v.y;
      },
      Uniform::Vec3(v) => {
        collector[0] = v.x;
        collector[1] = v.y;
        collector[2] = v.z;
      },
      Uniform::Vec4(v) => {
        collector[0] = v.x;
        collector[1] = v.y;
        collector[2] = v.z;
        collector[3] = v.w;
      },
      Uniform::Mat3(m) => {
        let m_sz = 9;
        let m_ptr = {
          let ptr = m.clone().as_ptr();
          std::slice::from_raw_parts(ptr, m_sz)
        };
        for i in 0..m_sz {
            collector[i] = m_ptr[i];
        }
      },
      Uniform::Mat4(m) => {
        let m_sz = 16;
        let m_ptr = {
          let ptr = m.clone().as_ptr();
          std::slice::from_raw_parts(ptr, m_sz)
        };
        for i in 0..m_sz {
            collector[i] = m_ptr[i];
        }
      },
      _ => println!("Uniform of type {:?} not supported", self)
    }
  }
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum UniformType {
    Int,
    IntArray,
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Mat4,
    Bool,
    Texture,
    UniformBuffer
}

#[allow(dead_code)]
pub enum UniformLifecycle {
  Frame,
  Runtime,

}

#[derive(Clone, Debug)]
pub struct UniformBuffer {

}

impl UniformBuffer {

}