use std::ffi::{CStr, CString};

use utils::*;

use renderer::{Texture, Uniform, WHITE_TEXTURE, TextureLike, DEBUG_TEXTURE};

#[derive(Clone, Debug)]
pub struct Material {
  pub uniforms: Vec<(CString, Uniform)>,
}

impl Material {
  pub fn ambient(&mut self, v: Vec3F) {
    self.upsert_uniform(c_str!("ambient"), Uniform::Vec3(v));
  }

  pub fn diffuse(&mut self, v: Vec3F) {
    self.upsert_uniform(c_str!("diffuse"), Uniform::Vec3(v));
  }

  pub fn specular(&mut self, v: Vec3F) {
    self.upsert_uniform(c_str!("specular"), Uniform::Vec3(v));
  }
  #[allow(dead_code)]

  pub fn shininess(&mut self, v: f32) {
    self.upsert_uniform(c_str!("shininess"), Uniform::Float(v));
  }
  #[allow(dead_code)]
  pub fn dissolve(&mut self, v: f32) {
    self.upsert_uniform(c_str!("dissolve"), Uniform::Float(v));
  }
  #[allow(dead_code)]
  pub fn optical_density(&mut self, v: f32) {
    self.upsert_uniform(c_str!("optical_density"), Uniform::Float(v));
  }
  #[allow(dead_code)]
  pub fn diffuse_texture(&mut self, v: Texture) {
    self.upsert_uniform(c_str!("diffuse_texture"), Uniform::Texture(v));
  }

  pub fn ambient_texture(&mut self, v: Texture) {
    self.upsert_uniform(c_str!("ambient_texture"), Uniform::Texture(v));
  }

  pub fn specular_texture(&mut self, v: Texture) {
    self.upsert_uniform(c_str!("specular_texture"), Uniform::Texture(v));
  }

  pub fn normal_texture(&mut self, v: Texture) {
    self.upsert_uniform(c_str!("normal_texture"), Uniform::Texture(v));
  }
  #[allow(dead_code)]
  pub fn shininess_texture(&mut self, v: Texture) {
    self.upsert_uniform(c_str!("shininess_texture"), Uniform::Texture(v));
  }

  #[allow(dead_code)]
  pub fn dissolve_texture(&mut self, v: Texture) {
    self.upsert_uniform(c_str!("dissolve_texture"), Uniform::Texture(v));
  }

  pub fn unknown_uniform(&mut self, name: &str, uniform: Uniform) {
    let c_str = CString::new(name).expect("Could not convert string to cstring");
    self.upsert_uniform(&c_str, uniform);
  }
  pub fn uniforms(&self) -> &Vec<(CString, Uniform)> {
    &self.uniforms
  }
  pub fn new() -> Material {
    let mut ret = Material { uniforms: Vec::new() };
    ret.diffuse_texture(WHITE_TEXTURE.clone());
    ret.ambient_texture(WHITE_TEXTURE.clone());
    ret.specular_texture(WHITE_TEXTURE.clone());

    ret.diffuse(Vec3F::new(1f32, 1f32, 1f32));
    ret.ambient(Vec3F::new(1f32, 1f32, 1f32));
    ret.specular(Vec3F::new(1f32, 1f32, 1f32));
    ret.normal_texture(WHITE_TEXTURE.clone());
    ret.unknown_uniform("debug_texture", Uniform::Texture(DEBUG_TEXTURE.clone()));
    ret
  }

  fn upsert_uniform(&mut self, c_str: &CStr, value: Uniform) {
    let c_string = CString::from(c_str);
    let mut flag = true;
    for i in 0..self.uniforms.len() {
      if flag && self.uniforms[i].0 == c_string {
        self.uniforms[i].1 = value.clone();
        flag = false;
        break;
      }
    }
    if flag {
      self.uniforms.push((c_string, value));
    }
  }

  pub fn refresh(&mut self) {
    self.uniforms.iter_mut().for_each(|(_, uniform)| {
      match uniform {
        Uniform::CubeMap(c) => c.refresh(),
        Uniform::Texture(t) => t.refresh(),
        _ => {}
      }
    });
  }
}
