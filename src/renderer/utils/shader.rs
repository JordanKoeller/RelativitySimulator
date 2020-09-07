use std::ffi::{CStr, CString};

use gl;

use cgmath::prelude::*;
use cgmath::{Matrix, Matrix3, Matrix4, Vector3};

use renderer::utils::{UniformType, UniformValue};
use renderer::utils::Texture;

/// NOTE: mixture of `shader_s.h` and `shader_m.h` (the latter just contains
/// a few more setters for uniforms)
#[allow(dead_code)]
pub trait IShader {
  fn id(&self) -> u32;

  /// activate the shader
  /// ------------------------------------------------------------------------
  fn use_program(&self) {
    // println!("Setting program {}", self.id());
    unsafe { gl::UseProgram(self.id()) }
  }

  fn close_program(&mut self) {
    unsafe { gl::UseProgram(0) }
  }

  /// utility uniform functions
  /// ------------------------------------------------------------------------
  fn set_bool(&self, name: &CStr, value: bool) {
    unsafe {
      gl::Uniform1i(gl::GetUniformLocation(self.id(), name.as_ptr()), value as i32);
    }
  }
  /// ------------------------------------------------------------------------
  fn set_int(&self, name: &CStr, value: i32) {
    unsafe {
      gl::Uniform1i(gl::GetUniformLocation(self.id(), name.as_ptr()), value);
    }
  }
  /// ------------------------------------------------------------------------
  fn set_float(&self, name: &CStr, value: f32) {
    unsafe {
      gl::Uniform1f(gl::GetUniformLocation(self.id(), name.as_ptr()), value);
    }
  }
  /// ------------------------------------------------------------------------
  fn set_vector3(&self, name: &CStr, value: &Vector3<f32>) {
    unsafe {
      gl::Uniform3fv(gl::GetUniformLocation(self.id(), name.as_ptr()), 1, value.as_ptr());
    }
  }
  /// ------------------------------------------------------------------------
  fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
    unsafe {
      gl::Uniform3f(gl::GetUniformLocation(self.id(), name.as_ptr()), x, y, z);
    }
  }
  /// ------------------------------------------------------------------------
  fn set_mat4(&self, name: &CStr, mat: &Matrix4<f32>) {
    unsafe {
      gl::UniformMatrix4fv(
        gl::GetUniformLocation(self.id(), name.as_ptr()),
        1,
        gl::FALSE,
        mat.as_ptr(),
      );
    }
  }

  /// ------------------------------------------------------------------------
  fn set_mat3(&self, name: &CStr, mat: &Matrix3<f32>) {
    unsafe {
      gl::UniformMatrix3fv(
        gl::GetUniformLocation(self.id(), name.as_ptr()),
        1,
        gl::FALSE,
        mat.as_ptr(),
      );
    }
  }

  fn set_uniform(&self, name: &String, uniform: &UniformValue) {
    let cstring = CString::new(name.as_bytes()).expect("Invalid Shader Name");
    match uniform {
      UniformValue::Bool(f) => self.set_bool(&cstring, f.clone()),
      UniformValue::Float(f) => self.set_float(&cstring, f.clone()),
      UniformValue::Int(f) => self.set_int(&cstring, f.clone()),
      UniformValue::Vec3(f) => self.set_vector3(&cstring, f),
      UniformValue::Mat3(f) => self.set_mat3(&cstring, f),
      UniformValue::Mat4(f) => self.set_mat4(&cstring, f),
      _ => println!("Shader does not support uniform of type {:?}", uniform),
    }
  }

  fn set_texture(&self, texture: &Texture) {
    let unif_value = UniformValue::Int(0);
    self.set_uniform(&texture.to_type().get_name(), &unif_value);
  }
}

#[derive(Copy, Clone)]
pub struct Shader {
  pub id: u32,
}

impl IShader for Shader {
  fn id(&self) -> u32 {
    self.id
  }
}
impl Shader {
  pub fn new(id: u32) -> Shader {
    Shader { id: id}
  }
}