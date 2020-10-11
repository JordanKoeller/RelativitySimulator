use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::ptr;
use std::str;

use cgmath::prelude::*;
use gl;

use renderer::platform::{TextureLike, Uniform};

#[derive(Clone, Eq, PartialEq)]
pub enum ShaderStep {
  VertexShader(String),
  FragmentShader(String),
  // ComputeShader(String),
  // GeometryShader(String),
  TessControlShader(String),
  TessEvalShader(String),
}

impl ShaderStep {
  pub fn typestring(&self) -> String {
    match self {
      ShaderStep::VertexShader(_) => "VERTEX_SHADER".to_string(),
      ShaderStep::FragmentShader(_) => "FRAGMENT_SHADER".to_string(),
      ShaderStep::TessControlShader(_) => "TESS_CONTROL_SHADER".to_string(),
      ShaderStep::TessEvalShader(_) => "TESS_EVALUATION_SHADER".to_string(),
    }
  }

  pub fn text(&self) -> &str {
    match self {
      ShaderStep::VertexShader(s) => s,
      ShaderStep::FragmentShader(s) => s,
      ShaderStep::TessControlShader(s) => s,
      ShaderStep::TessEvalShader(s) => s,
    }
  }

  pub fn glEnum(&self) -> gl::types::GLenum {
    match self {
      ShaderStep::FragmentShader(_) => gl::FRAGMENT_SHADER,
      ShaderStep::VertexShader(_) => gl::VERTEX_SHADER,
      ShaderStep::TessControlShader(_) => gl::TESS_CONTROL_SHADER,
      ShaderStep::TessEvalShader(_) => gl::TESS_EVALUATION_SHADER,
    }
  }
}

pub struct ShaderState {
  pub id: u32,
  pub name: String,
  pub program_source: String,
  pub uniforms: RefCell<std::collections::HashMap<CString, i32>>,
}

pub struct SimpleShader {
  state: ShaderState
}

impl Shader for SimpleShader {
  fn shader_state(&self) -> &ShaderState {
    &self.state
  }
  fn shader_state_mut(&mut self) -> &mut ShaderState {
    &mut self.state
  }
}

impl SimpleShader {
  pub fn new(name: &str, shader_body: &str) -> Self {
    let shader_steps = decompress(shader_body.to_string());
    let program_id = compile_program(shader_steps);
    let state = ShaderState {
      id: program_id,
      name: name.to_string(),
      program_source: shader_body.to_string(),
      uniforms: RefCell::new(std::collections::HashMap::new()),
    };
    SimpleShader {
      state,
    }
  }
  pub fn from_file(name: &str, shader_path: &str) -> Self {
    let file_body = super::shader_preprocessor::file_includer(shader_path);
    Self::new(name, &file_body)
  }
}

pub struct SkyboxShader {
  state: ShaderState
}

impl Shader for SkyboxShader {
  fn shader_state(&self) -> &ShaderState {
    &self.state
  }
  fn shader_state_mut(&mut self) -> &mut ShaderState {
    &mut self.state
  }

  fn bind(&self) {
    unsafe {
      gl::DepthFunc(gl::LEQUAL);
      gl::UseProgram(self.shader_state().id);
    }
  }
  fn unbind(&self) {
    unsafe {
      gl::DepthFunc(gl::LESS);
      gl::UseProgram(0);
    }
  }
}

impl SkyboxShader {
  pub fn new(name: &str, shader_body: &str) -> Self {
    let shader_steps = decompress(shader_body.to_string());
    let program_id = compile_program(shader_steps);
    let state = ShaderState {
      id: program_id,
      name: name.to_string(),
      program_source: shader_body.to_string(),
      uniforms: RefCell::new(std::collections::HashMap::new()),
    };
    SkyboxShader {
      state,
    }
  }
  pub fn from_file(name: &str, shader_path: &str) -> Self {
    let file_body = super::shader_preprocessor::file_includer(shader_path);
    Self::new(name, &file_body)
  }
}

pub trait Shader {
  fn shader_state(&self) -> &ShaderState;
  fn shader_state_mut(&mut self) -> &mut ShaderState;


  fn bind(&self) {
    unsafe {
      gl::UseProgram(self.shader_state().id);
    }
  }
  fn unbind(&self) {
    unsafe {
      gl::UseProgram(0);
    }
  }

  fn name(&self) -> &str {
    &self.shader_state().name
  }

  fn set_uniform(&self, name: &CStr, unif: &Uniform) {
    // println!("Setting shader {:?}", name);
    let mut unif_mgr = self.shader_state().uniforms.borrow_mut();
    if let Some(&loc) = unif_mgr.get(name.clone()) {
      set_unif_helper(unif, loc);
    } else {
      let loc = unsafe { gl::GetUniformLocation(self.shader_state().id, name.as_ptr()) };
      unif_mgr.insert(name.to_owned(), loc);
      set_unif_helper(unif, loc);
    }
  }

  fn set_texture(&self, slot: u32, name: &CStr, texture: &dyn TextureLike) {
    texture.bind(slot);
    let unif = Uniform::Int(slot as i32);
    self.set_uniform(name, &unif);
  }
}

fn decompress(body: String) -> Vec<ShaderStep> {
  body
    .split("#shader ")
    .filter(|s| {
      if let Some(first_line) = s.lines().next() {
        if SHADER_OPTIONS.iter().any(|&e| e == first_line) {
          true
        } else {
          false
        }
      } else {
        false
      }
    })
    .map(|s| {
      let mut lines: Vec<&str> = s.lines().collect();
      let first_line = lines.remove(0);
      match first_line {
        "vertex" => ShaderStep::VertexShader(lines.join("\n")),
        "fragment" => ShaderStep::FragmentShader(lines.join("\n")),
        _ => panic!("Could not determine shader type from label"),
      }
    })
    .collect()
}
fn compile_program(steps: Vec<ShaderStep>) -> u32 {
  unsafe {
    let mut program = gl::CreateProgram();
    for step in steps.into_iter() {
      compile_shader(&mut program, step);
    }
    gl::LinkProgram(program);
    // Error checking
    let mut err_log = Vec::with_capacity(512);
    let mut err_code = 0;
    err_log.set_len(512 - 1);
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut err_code);
    if err_code != gl::TRUE as gl::types::GLint {
      gl::GetProgramInfoLog(
        program,
        512,
        ptr::null_mut(),
        err_log.as_mut_ptr() as *mut gl::types::GLchar,
      );
      println!(
        "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
        str::from_utf8(&err_log).unwrap()
      );
    }
    program
  }
}
unsafe fn compile_shader(program: &mut u32, shader: ShaderStep) {
  let shader_c_code = CString::new(shader.text().as_bytes()).unwrap();
  let shader_id = gl::CreateShader(shader.glEnum());
  gl::ShaderSource(shader_id, 1, &shader_c_code.as_ptr(), ptr::null());
  gl::CompileShader(shader_id);
  let mut err_log = Vec::with_capacity(512);
  let mut err_code = 0;
  err_log.set_len(512 - 1);
  gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut err_code);
  if err_code != gl::TRUE as gl::types::GLint {
    gl::GetShaderInfoLog(
      shader_id,
      512,
      ptr::null_mut(),
      err_log.as_mut_ptr() as *mut gl::types::GLchar,
    );
    println!(
      "ERROR::SHADER::{}::COMPILATION_FAILED\n{}",
      shader.typestring(),
      str::from_utf8(&err_log).unwrap()
    )
  }
  gl::AttachShader(*program, shader_id);
  gl::DeleteShader(shader_id);
  // let shader_code = CString::new(shader.0.as_bytes()).unwrap();
}

fn set_unif_helper(unif: &Uniform, loc: i32) {
  unsafe {
    match unif {
      Uniform::Int(v) => gl::Uniform1i(loc, v.clone()),
      Uniform::Float(v) => gl::Uniform1f(loc, v.clone()),
      Uniform::Vec2(v) => gl::Uniform2f(loc, v.x, v.y),
      Uniform::Vec3(v) => gl::Uniform3f(loc, v.x, v.y, v.z),
      Uniform::Vec4(v) => gl::Uniform4f(loc, v.x, v.y, v.z, v.w),
      Uniform::Mat3(v) => gl::UniformMatrix3fv(loc, 1, gl::FALSE, v.as_ptr()),
      Uniform::Mat4(v) => gl::UniformMatrix4fv(loc, 1, gl::FALSE, v.as_ptr()),
      Uniform::Bool(v) => gl::Uniform1i(loc, v.clone() as i32),
      Uniform::IntArray(arr) => gl::Uniform1iv(loc, arr.len() as i32, &arr[0] as *const i32),
      _ => panic!("Please set texture uniforms through the set_texture method"),
    }
  }
  // gl::Uniform1i(lo, )
}

const SHADER_OPTIONS: [&str; 2] = ["vertex", "fragment"];

// pub struct ShaderLibrary {

// }
