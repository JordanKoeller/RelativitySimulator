
use renderer::*;
use ecs::{Material, DrawableId};


#[derive(Default)]
pub struct AssetLibrary {
  shader_lookup: std::collections::HashMap<String, usize>,
  shaders: Vec<Shader>,
  models: Vec<Mesh>,
}


impl AssetLibrary {
  pub fn register_shader(&mut self, shader: Shader) {
    let name = shader.name.clone();
    let ind = self.shaders.len();
    self.shader_lookup.insert(name, ind);
    self.shaders.push(shader);
  }

  pub fn register_asset(&mut self, mut asset: Mesh) -> DrawableId {
    let s_id = ShaderId(*self.shader_lookup.get(&asset.shader_name).expect(&format!("Shader {} not registered", asset.shader_name)));
    let ret = DrawableId(self.models.len());
    asset.registry = Some((ret.clone(), s_id));
    // let elem = DrawableMemo {
    //   vertex_array: asset.vertex_array,
    //   material: asset.material,
    //   shader_id: s_id
    // };
    self.models.push(asset);
    ret
  }

  pub fn get_shader(&self, id: &ShaderId) -> &Shader {
    &self.shaders[id.0]
  }

  pub fn get_asset(&self, id: &DrawableId) -> &Mesh {
    &self.models[id.0]
  }

  pub fn get_asset_mut(&mut self, id: &DrawableId) -> &mut Mesh {
    &mut self.models[id.0]
  }
}