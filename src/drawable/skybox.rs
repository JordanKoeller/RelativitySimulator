use std::mem::size_of;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::CStr;

use image;
use image::DynamicImage::*;
use image::GenericImage;

use cgmath::Zero;

use gl;

use drawable::Drawable;
use shader_manager::ShaderManager;

const SKYBOX_VERTICES: [f32; 108] = [
  -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0,
  1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0,
  1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
  1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0,
  1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0,
  -1.0, 1.0,
];

pub struct Skybox {
  texture_id: u32,
  vao: u32,
  vbo: u32,
  view_matrix: cgmath::Matrix4<f32>,
  proj_matrix: cgmath::Matrix4<f32>
}

fn init_skybox() -> (u32, u32) {
  let mut vao = 0;
  let mut vbo = 0;
  unsafe {
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::BindVertexArray(vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
      gl::ARRAY_BUFFER,
      (SKYBOX_VERTICES.len() * size_of::<f32>()) as isize,
      &SKYBOX_VERTICES[0] as *const f32 as *const c_void,
      gl::STATIC_DRAW,
    );
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
      0,
      3,
      gl::FLOAT,
      gl::FALSE,
      3 * size_of::<f32>() as i32,
      0 as *const c_void,
    );
  }
  (vao, vbo)
}

fn init_texture(faces: [String; 6]) -> u32 {
  let mut texture_id = 0;
  unsafe {
    gl::GenTextures(1, &mut texture_id);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);
    for i in 0..6 {
      let filename = &faces[i];
      let img = image::open(&Path::new(filename)).expect("Texture failed to load");
      // let img = img.flipv(); // May be unecessary
      let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
      };
      let data = img.raw_pixels();
      gl::TexImage2D(
        gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
        0,
        format as i32,
        img.width() as i32,
        img.height() as i32,
        0,
        format,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
      );
    }
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
  }
  texture_id
}

impl Skybox {

  pub fn new(faces: [String; 6]) -> Skybox {
    let (vao, vbo) = init_skybox();
    let tex = init_texture(faces);
    Skybox {
      vao: vao,
      vbo: vbo,
      texture_id: tex,
      view_matrix: cgmath::Matrix4::<f32>::zero(),
      proj_matrix: cgmath::Matrix4::<f32>::zero()
    }
  }

  pub fn set_matrices(&mut self, view_matrix: &cgmath::Matrix4<f32>, projec_matrix: &cgmath::Matrix4<f32>) {
    self.view_matrix = view_matrix.clone();
    self.proj_matrix = projec_matrix.clone();
    self.view_matrix.w[0] = 0.0;
    self.view_matrix.w[1] = 0.0;
    self.view_matrix.w[2] = 0.0;
  }
}

impl Drawable for Skybox {
  fn shader_name(&self) -> String {
    "skybox".to_string()
  }

  fn draw(&self, shader: &ShaderManager) {
    let s = shader.get_shader(self.shader_name());
    s.use_program();
    unsafe {
      gl::DepthFunc(gl::LEQUAL);
    }
    s.set_int(c_str!("skybox"), 0);
    s.set_mat4(c_str!("viewMatrix"), &self.view_matrix);
    s.set_mat4(c_str!("projectionMatrix"), &self.proj_matrix);
    unsafe {
      gl::BindVertexArray(self.vao);
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture_id);
      gl::DrawArrays(gl::TRIANGLES, 0, 36);
      gl::BindVertexArray(0);
      gl::DepthFunc(gl::LESS);
    }


  }
}