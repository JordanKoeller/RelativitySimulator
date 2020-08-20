use std::ffi::{CStr, CString};
use std::ptr;
use std::str;

use gl;
use gl::types::*;

use cgmath::prelude::*;
use cgmath::{Matrix, Matrix3, Matrix4, Vector3};

use renderer::uniform::{UniformType, UniformValue};
use renderer::shader_preprocessor;

/// NOTE: mixture of `shader_s.h` and `shader_m.h` (the latter just contains
/// a few more setters for uniforms)
#[allow(dead_code)]
trait IShader {
  fn id(&self) -> u32;

  /// activate the shader
  /// ------------------------------------------------------------------------
  fn use_program(&mut self) {
    unsafe { gl::UseProgram(self.id()) }
  }

  fn close_program(&mut self) {}

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
      UniformValue::UInt(f) => self.set_int(&cstring, f.clone() as i32),
      UniformValue::Vec3(f) => self.set_vector3(&cstring, f),
      UniformValue::Mat3(f) => self.set_mat3(&cstring, f),
      UniformValue::Mat4(f) => self.set_mat4(&cstring, f),
      _ => println!("Shader does not support uniform of type {:?}", uniform),
    }
  }

  /// utility function for checking shader compilation/linking errors.
  /// ------------------------------------------------------------------------
  fn check_compile_errors(&self, shader: u32, type_: &str) {
    unsafe {
      let mut success = gl::FALSE as GLint;
      let mut info_log: [u8; 2048] = [0; 2048];
      if type_ != "PROGRAM" {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
          gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
          println!(
            "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
            type_,
            str::from_utf8(&info_log).expect("Encountered error on utf8 cast in SHADER check")
          );
        }
      } else {
        gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
          gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
          println!(
            "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
            type_,
            str::from_utf8(&info_log).expect("Encountered error on utf8 cast in PROGRAM check")
          );
        }
      }
    }
  }
}

pub struct Shader {
  pub id: u32,
}

impl IShader for Shader {
  fn id(&self) -> u32 {
    self.id
  }
}

impl Shader {
  fn new(vertex_path: &str, fragment_path: &str) -> Self {
    let mut shader = Shader { id: 0 };
    let (vertex_code, set1) = shader_preprocessor(vertex_path);
    let (fragment_code, set2) = shader_preprocessor(fragment_path);

    let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
    let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();

    // 2. compile shaders
    unsafe {
      // vertex shader
      let vertex = gl::CreateShader(gl::VERTEX_SHADER);
      gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(vertex);
      shader.check_compile_errors(vertex, "VERTEX");
      // fragment Shader
      let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
      gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(fragment);
      shader.check_compile_errors(fragment, "FRAGMENT");
      // shader Program
      let id = gl::CreateProgram();
      gl::AttachShader(id, vertex);
      gl::AttachShader(id, fragment);
      gl::LinkProgram(id);
      shader.check_compile_errors(id, "PROGRAM");
      // delete the shaders as they're linked into our program now and no longer necessary
      gl::DeleteShader(vertex);
      gl::DeleteShader(fragment);
      shader.id = id;
    }

    shader
  }
  pub fn tesselation_pipeline(vertex_path: &str, fragment_path: &str, cs_path: &str, es_path: &str) -> Self {
    let mut shader = Shader { id: 0 };
    let (vertex_code, set1) = shader_preprocessor(vertex_path);
    let (fragment_code, set2) = shader_preprocessor(fragment_path);
    let (cs_code, set3) = shader_preprocessor(cs_path);
    let (es_code, set4) = shader_preprocessor(es_path);
    let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
    let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();
    let es_shader_code = CString::new(es_code.as_bytes()).unwrap();
    let cs_shader_code = CString::new(cs_code.as_bytes()).unwrap();
    // 2. compile shaders
    unsafe {
      // vertex shader
      let vertex = gl::CreateShader(gl::VERTEX_SHADER);
      gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(vertex);
      shader.check_compile_errors(vertex, "VERTEX");
      println!("Validated vertex shader");
      // fragment Shader
      let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
      gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(fragment);
      shader.check_compile_errors(fragment, "FRAGMENT");
      println!("Validated fragment shader");
      // evaluation shader
      let es_shader = gl::CreateShader(gl::TESS_EVALUATION_SHADER);
      gl::ShaderSource(es_shader, 1, &es_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(es_shader);
      shader.check_compile_errors(es_shader, "EVALUATION");
      println!("Validated evaluation shader");
      // control shader
      let cs_shader = gl::CreateShader(gl::TESS_CONTROL_SHADER);
      gl::ShaderSource(cs_shader, 1, &cs_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(cs_shader);
      shader.check_compile_errors(cs_shader, "CONTROL");
      println!("Validated control shader");
      // shader Program
      let id = gl::CreateProgram();
      gl::AttachShader(id, vertex);
      gl::AttachShader(id, fragment);
      gl::AttachShader(id, cs_shader);
      gl::AttachShader(id, es_shader);
      gl::LinkProgram(id);
      shader.check_compile_errors(id, "PROGRAM");
      // delete the shaders as they're linked into our program now and no longer necessary
      gl::DeleteShader(vertex);
      gl::DeleteShader(fragment);
      gl::DeleteShader(es_shader);
      gl::DeleteShader(cs_shader);
      shader.id = id;
    }
    shader
  }
}

pub struct DebugShader {
  shader: Shader,
  expected_uniforms: std::collections::HashSet<(String, UniformType)>,
  set_uniforms: std::collections::HashSet<(String, UniformType)>,
}

impl IShader for DebugShader {
  fn id(&self) -> u32 {
    self.shader.id()
  }

  fn use_program(&mut self) {
    self.set_uniforms = std::collections::HashSet::new();
  }

  fn close_program(&mut self) {
    if self.set_uniforms != self.expected_uniforms {
      let delta: Vec<(String, UniformType)> = self.expected_uniforms.difference(&self.set_uniforms).cloned().collect();
      panic!("Tried to run shader missing {} uniforms {:?}", delta.len(), delta);
    }
  }

  fn set_uniform(&mut self, name: &String, uniform: &UniformValue) {
    // let expects_uniform = self.expected_uniforms.contains(name);
    let uniform_pair = match uniform {
      UniformValue::Bool(f) => (name.clone(), UniformType::Bool),
      UniformValue::Float(f) => (name.clone(), UniformType::Float),
      UniformValue::Int(f) => (name.clone(), UniformType::Int),
      UniformValue::UInt(f) => (name.clone(), UniformType::UInt),
      UniformValue::Vec3(f) => (name.clone(), UniformType::Vec3),
      UniformValue::Mat3(f) => (name.clone(), UniformType::Mat3),
      UniformValue::Mat4(f) => (name.clone(), UniformType::Mat4),
      _ => panic!("Shader does not support uniform of type {:?}", uniform),
    };
    if self.expected_uniforms.contains(&uniform_pair) {
      self.set_uniforms.insert(uniform_pair);
      self.set_uniform(name, uniform);
    } else {
      panic!("Tried to set uniform {} on shader not expecting that uniform", name);
    }
  }
}

impl DebugShader {
  fn new(vertex_path: &str, fragment_path: &str) -> Self {
    let mut shader = Shader { id: 0 };
    let (vertex_code, set1) = shader_preprocessor(vertex_path);
    let (fragment_code, set2) = shader_preprocessor(fragment_path);

    let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
    let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();

    let all_uniforms: std::collections::HashSet<(String, UniformType)> = set1.union(&set2).cloned().collect();

    // 2. compile shaders
    unsafe {
      // vertex shader
      let vertex = gl::CreateShader(gl::VERTEX_SHADER);
      gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(vertex);
      shader.check_compile_errors(vertex, "VERTEX");
      // fragment Shader
      let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
      gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(fragment);
      shader.check_compile_errors(fragment, "FRAGMENT");
      // shader Program
      let id = gl::CreateProgram();
      gl::AttachShader(id, vertex);
      gl::AttachShader(id, fragment);
      gl::LinkProgram(id);
      shader.check_compile_errors(id, "PROGRAM");
      // delete the shaders as they're linked into our program now and no longer necessary
      gl::DeleteShader(vertex);
      gl::DeleteShader(fragment);
      shader.id = id;
    }

    DebugShader {
      shader: shader,
      expected_uniforms: all_uniforms,
      set_uniforms: std::collections::HashSet::new(),
    }
  }
  pub fn tesselation_pipeline(vertex_path: &str, fragment_path: &str, cs_path: &str, es_path: &str) -> Self {
    let mut shader = Shader { id: 0 };
    let (vertex_code, set1) = shader_preprocessor(vertex_path);
    let (fragment_code, set2) = shader_preprocessor(fragment_path);
    let (cs_code, set3) = shader_preprocessor(cs_path);
    let (es_code, set4) = shader_preprocessor(es_path);
    let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
    let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();
    let es_shader_code = CString::new(es_code.as_bytes()).unwrap();
    let cs_shader_code = CString::new(cs_code.as_bytes()).unwrap();
    // 2. compile shaders
    let all_uniforms: std::collections::HashSet<(String, UniformType)> =
      vec![set1, set2, set3, set4].iter().flatten().cloned().collect();

    unsafe {
      // vertex shader
      let vertex = gl::CreateShader(gl::VERTEX_SHADER);
      gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(vertex);
      shader.check_compile_errors(vertex, "VERTEX");
      println!("Validated vertex shader");
      // fragment Shader
      let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
      gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(fragment);
      shader.check_compile_errors(fragment, "FRAGMENT");
      println!("Validated fragment shader");
      // evaluation shader
      let es_shader = gl::CreateShader(gl::TESS_EVALUATION_SHADER);
      gl::ShaderSource(es_shader, 1, &es_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(es_shader);
      shader.check_compile_errors(es_shader, "EVALUATION");
      println!("Validated evaluation shader");
      // control shader
      let cs_shader = gl::CreateShader(gl::TESS_CONTROL_SHADER);
      gl::ShaderSource(cs_shader, 1, &cs_shader_code.as_ptr(), ptr::null());
      gl::CompileShader(cs_shader);
      shader.check_compile_errors(cs_shader, "CONTROL");
      println!("Validated control shader");
      // shader Program
      let id = gl::CreateProgram();
      gl::AttachShader(id, vertex);
      gl::AttachShader(id, fragment);
      gl::AttachShader(id, cs_shader);
      gl::AttachShader(id, es_shader);
      gl::LinkProgram(id);
      shader.check_compile_errors(id, "PROGRAM");
      // delete the shaders as they're linked into our program now and no longer necessary
      gl::DeleteShader(vertex);
      gl::DeleteShader(fragment);
      gl::DeleteShader(es_shader);
      gl::DeleteShader(cs_shader);
      shader.id = id;
    }
    DebugShader {
      shader: shader,
      expected_uniforms: all_uniforms,
      set_uniforms: std::collections::HashSet::new(),
    }
  }
}
