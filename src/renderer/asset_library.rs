use std::collections::HashMap;

use std::ffi::CString;
use renderer::*;
use ecs::{Material, DrawableId};


#[derive(Default)]
pub struct AssetLibrary {
  shader_lookup: std::collections::HashMap<String, usize>,
  shaders: Vec<Shader>,
  models: Vec<Mesh>,
  active_shader_id: Option<usize>,
}


impl AssetLibrary {
  pub fn register_shader(&mut self, shader: Shader) {
    let name = shader.name.clone();
    let ind = self.shaders.len();
    self.shader_lookup.insert(name, ind);
    self.shaders.push(shader);
  }

  pub fn register_asset(&mut self, mut asset: Mesh) -> DrawableId {
    let s_id = *self.shader_lookup.get(&asset.shader_name).expect(&format!("Shader {} not registered", asset.shader_name));
    let ret = DrawableId(self.models.len(), s_id);
    asset.registry = Some(ret.clone());
    // let elem = DrawableMemo {
    //   vertex_array: asset.vertex_array,
    //   material: asset.material,
    //   shader_id: s_id
    // };
    self.models.push(asset);
    ret
  }

  pub fn get_shader(&self, id: &usize) -> &Shader {
    &self.shaders[*id]
  }

  pub fn get_active_shader(&self) -> &Shader {
    &self.shaders[self.active_shader_id.unwrap()]
  }

  pub fn get_asset(&self, id: &usize) -> &Mesh {
    &self.models[*id]
  }

  pub fn get_asset_mut(&mut self, id: &usize) -> &mut Mesh {
    &mut self.models[*id]
  }

  pub fn get_shader_and_mesh(&self, id: &DrawableId) -> (&Mesh, &Shader) {
    (self.get_asset(&id.0), self.get_shader(&id.1))
  }

  pub fn activate_get_mesh(&mut self, id: &DrawableId, uniforms: &[&HashMap<CString, Uniform>]) -> &mut Mesh {
    if let Some(s_id) = self.active_shader_id {
      if s_id != id.1 {
        let s_ref = self.get_shader(&id.1);
        self.activate_shader(s_ref, uniforms);
        self.active_shader_id = Some(id.1);
        self.get_asset_mut(&id.0)
      } else {
        self.get_asset_mut(&id.0)
      }
    } else {
      let s_ref = self.get_shader(&id.1);
      self.activate_shader(s_ref, uniforms);
      self.active_shader_id = Some(id.1);
      self.get_asset_mut(&id.0)
    }
  }

   fn activate_shader(&self, shader: &Shader, uniforms: &[&HashMap<CString, Uniform>]) {
     shader.bind();
     for mgr in uniforms.iter() {
      for (unif_name, unif_value) in mgr.iter() {
        shader.set_uniform(&unif_name, unif_value);
      }
     }
   }
}