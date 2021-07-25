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
  pub fn get_active_shader_mut(&mut self) -> &mut Shader {
    &mut self.shaders[self.active_shader_id.unwrap()]
  }

  #[inline]
  pub fn get_active_shader(&self) -> &Shader {
    &self.shaders[self.active_shader_id.unwrap()]
  }

  #[inline]
  pub fn get_active_asset(&self) -> &Mesh {
    &self.models[self.active_asset_id.unwrap()]
  }
  #[inline]
  pub fn get_active_asset_mut(&mut self) -> &mut Mesh {
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

  // #[inline]
  // pub fn get_shader_and_mesh(&self, id: &DrawableId) -> (&Mesh, &Shader) {
  //   (self.get_asset(&id.0), self.get_shader(&id.1))
  // }

  // #[inline]
  // pub fn active_is_instanced(&self) -> bool {
  //   if let Some(_) = self.active_asset_id {
  //     self.get_active_asset().instanced()
  //   } else {
  //     false
  //   }
  // }

  #[inline]
  pub fn upsert_instance_data(
    &mut self,
    entity: &Entity,
    model: &Mat4F,
    material: &Material,
    textures: &mut TextureBinder,
  ) {
    self.models[self.active_asset_id.unwrap()].upsert_instance(
      entity,
      model,
      material,
      textures,
      &self.shaders[self.active_shader_id.unwrap()]
    );
    // self
    //   .get_active_asset_mut()
    //   .upsert_instance(entity, model, material, textures, shader);
    // println!("Instance table contains {} instances", self.get_active_asset().instance_table.as_ref().unwrap().num_instances());
  }

  pub fn select(&mut self, id: &DrawableId) -> GPUState<'_> {
    self.active_asset_id = Some(id.0);
    self.active_shader_id = Some(id.1);
    GPUState::new(self, id.clone())
  }


  // #[inline]
  // fn enable_shader(&self, shader: &Shader, uniforms: &[&HashMap<CString, Uniform>]) {
  //   // println!("Activating shader {}, {}", shader.name, self.active_shader_id.unwrap());
  //   shader.bind();
  //   for mgr in uniforms.iter() {
  //     for (unif_name, unif_value) in mgr.iter() {
  //       shader.set_uniform(&unif_name, unif_value);
  //     }
  //   }
  // }


  pub fn deactivate_all(&mut self) {
    self.active_shader_id = None;
    self.active_asset_id = None;
  }
}
