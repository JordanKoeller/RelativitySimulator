use specs::Entity;

use std::collections::HashMap;

use ecs::{DrawableId, Material};
use renderer::*;
use std::ffi::{CStr, CString};
use utils::Mat4F;

#[derive(Default)]
pub struct AssetLibrary {
  shader_lookup: std::collections::HashMap<String, usize>,
  shaders: Vec<Shader>,
  models: Vec<Mesh>,
  active_shader_id: Option<usize>,
  active_asset_id: Option<usize>,
}

impl AssetLibrary {
  pub fn register_shader(&mut self, shader: Shader) {
    let name = shader.name.clone();
    let ind = self.shaders.len();
    println!("Registered shader {} as ID {}", name, ind);
    self.shader_lookup.insert(name, ind);
    self.shaders.push(shader);
  }

  pub fn register_asset(&mut self, mut asset: Mesh) -> DrawableId {
    let s_id = *self
      .shader_lookup
      .get(&asset.shader_name)
      .expect(&format!("Shader {} not registered", asset.shader_name));
    let ret = DrawableId(self.models.len(), s_id);
    asset.registry = Some(ret.clone());
    self.models.push(asset);
    ret
  }

  #[inline]
  pub fn get_shader(&self, id: &usize) -> &Shader {
    &self.shaders[*id]
  }

  #[inline]
  fn get_active_shader(&self) -> &Shader {
    &self.shaders[self.active_shader_id.unwrap()]
  }

  #[inline]
  fn get_active_asset(&self) -> &Mesh {
    &self.models[self.active_asset_id.unwrap()]
  }
  #[inline]
  fn get_active_asset_mut(&mut self) -> &mut Mesh {
    &mut self.models[self.active_asset_id.unwrap()]
  }

  #[inline]
  pub fn get_asset(&self, id: &usize) -> &Mesh {
    &self.models[*id]
  }

  #[inline]
  pub fn get_asset_mut(&mut self, id: &usize) -> &mut Mesh {
    &mut self.models[*id]
  }

  #[inline]
  pub fn get_shader_and_mesh(&self, id: &DrawableId) -> (&Mesh, &Shader) {
    (self.get_asset(&id.0), self.get_shader(&id.1))
  }

  #[inline]
  pub fn active_is_instanced(&self) -> bool {
    if let Some(id) = self.active_asset_id {
      self.get_active_asset().instanced()
    } else {
      false
    }
  }

  #[inline]
  pub fn upsert_instance_data(
    &mut self,
    entity: &Entity,
    model: &Mat4F,
    material: &Material,
    textures: &mut TextureBinder,
  ) {
    self
      .get_active_asset_mut()
      .upsert_instance(entity, model, material, textures);
    // println!("Instance table contains {} instances", self.get_active_asset().instance_table.as_ref().unwrap().num_instances());
  }

  #[inline]
  pub fn free_instance(&mut self, entity: &Entity) {
    self.get_active_asset_mut().clear_instance(entity);
  }

  pub fn draw_active_mesh(&mut self, model: Mat4F, material: &Material, textures: &mut TextureBinder) {
    if !self.active_is_instanced() {
      let shader = self.get_active_shader();
      for (unif_name, unif) in material.uniforms() {
        match unif {
          Uniform::Texture(tex) => {
            shader.set_texture(textures.get_slot(tex.id()), unif_name, tex);
          }
          Uniform::CubeMap(tex) => {
            shader.set_texture(textures.get_slot(tex.id()), unif_name, tex);
          }
          _ => shader.set_uniform(&unif_name, &unif),
        }
      }
      shader.set_uniform(c_str!("model"), &Uniform::Mat4(model));
      let et = self.get_active_shader().element_type;
      self.get_active_asset().draw(&et);
    } else {
      panic!("Tried to draw an instanced mesh as non-instanced")
    }
  }

  pub fn flush_and_activate_drawable(&mut self, id: &DrawableId, uniforms: &[&HashMap<CString, Uniform>]) {
    if let Some(curr_active) = self.active_asset_id {
      if self.active_is_instanced() && curr_active != id.0 {
        self.flush_instances();
      }
    }
    self.activate_shader(id.1, uniforms);
    self.activate_mesh(id.0);
  }

  #[inline]
  pub fn flush_instances(&self) {
    if self.active_is_instanced() {
      let et = self.get_active_shader().element_type;
      self.get_active_asset().draw(&et);
      // println!("Drawing {} instances", self.get_active_asset().instance_table.as_ref().unwrap().len());
    }
  }

  #[inline]
  pub fn activate_drawable(&mut self, id: &DrawableId, uniforms: &[&HashMap<CString, Uniform>]) {
    self.activate_shader(id.1, uniforms);
    self.activate_mesh(id.0);
  }

  #[inline]
  fn activate_mesh(&mut self, id: usize) {
    self.active_asset_id = Some(id);
    self.get_active_asset().vao.bind();
  }

  fn activate_shader(&mut self, s_id: usize, uniforms: &[&HashMap<CString, Uniform>]) {
    if let Some(curr_active) = self.active_shader_id {
      if curr_active == s_id {
        // Shader is already active. There is nothing to do.
      } else {
        self.active_shader_id = Some(s_id);
        self.enable_shader(self.get_active_shader(), uniforms);
      }
    } else {
      self.active_shader_id = Some(s_id);
      self.enable_shader(self.get_active_shader(), uniforms);
    }
  }

  #[inline]
  fn enable_shader(&self, shader: &Shader, uniforms: &[&HashMap<CString, Uniform>]) {
    // println!("Activating shader {}, {}", shader.name, self.active_shader_id.unwrap());
    shader.bind();
    for mgr in uniforms.iter() {
      for (unif_name, unif_value) in mgr.iter() {
        shader.set_uniform(&unif_name, unif_value);
      }
    }
  }

  pub fn deactivate_all(&mut self) {
    self.active_shader_id = None;
    self.active_asset_id = None;
  }
}
