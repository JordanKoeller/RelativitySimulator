use gl;
use std::collections::HashMap;
use std::ffi::{CStr, CString};

use crate::utils::RwAssetRef;

use super::ShaderId;

pub trait ShaderBinder {
  fn bind(&self, shader_id: u32);
  fn unbind(&self, shader_id: u32);
}

pub struct StdShaderBinder;

impl ShaderBinder for StdShaderBinder {
  fn bind(&self, shader_id: u32) {
    unsafe {
      gl::UseProgram(shader_id);
    }
  }
  fn unbind(&self, _shader_id: u32) {
    unsafe {
      gl::UseProgram(0u32);
    }
  }
}

pub struct DepthFuncShaderBinder {
  depth_func: gl::types::GLenum,
}

impl DepthFuncShaderBinder {
  pub fn new(depth_func: gl::types::GLenum) -> Self {
    Self { depth_func }
  }
}

impl ShaderBinder for DepthFuncShaderBinder {
  fn bind(&self, shader_id: u32) {
    unsafe {
      gl::DepthFunc(self.depth_func);
      gl::UseProgram(shader_id);
    }
  }
  fn unbind(&self, _shader_id: u32) {
    unsafe {
      gl::DepthFunc(gl::LESS);
      gl::UseProgram(0u32);
    }
  }
}

#[derive(Default)]
pub struct UniformSlots {
  slots: RwAssetRef<HashMap<String, i32>>,
}

impl UniformSlots {
  pub fn get_slot(&self, name: &str, shader_id: u32) -> i32 {
    let has_key = self.slots.get().contains_key(name);
    if has_key {
      *self.slots.get().get(name).unwrap()
    } else {
      let c_string = CString::new(name).unwrap();
      let slot = unsafe { gl::GetUniformLocation(shader_id, c_string.as_ptr()) };
      self.slots.get_mut().insert(name.to_owned(), slot.clone());
      slot
    }
  }
}

pub enum ShaderDepthFunction {
  LESS,
  LEQUAL,
}

impl ShaderDepthFunction {
  pub fn get_gl_enum(self) -> gl::types::GLenum {
    match self {
      ShaderDepthFunction::LESS => gl::LESS,
      ShaderDepthFunction::LEQUAL => gl::LEQUAL,
    }
  }
}
