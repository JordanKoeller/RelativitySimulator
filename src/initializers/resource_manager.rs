use std::collections::HashMap;

use initializers::{AssetSpec, Factory, GLBufferFactory, ShaderFactory, TextureFactory};
use renderer::{Asset, GLBuffer, Shader, Texture};

pub struct ResourceManger<F: Factory + Default> {
    resource_map: Vec<F::Resource>,
    name_map: HashMap<String, usize>,
    factory: F,
}

impl<F: Factory + Default> Default for ResourceManger<F> {
    fn default() -> ResourceManger<F> {
        ResourceManger {
            factory: F::default(),
            resource_map: Vec::new(),
            name_map: HashMap::new(),
        }
    }
}

impl<F: Factory + Default> ResourceManger<F> {
    pub fn get_resource(&self, id: usize) -> &F::Resource {
        &self.resource_map[id]
    }

    pub fn add_resource(&mut self, name: &str, spec: F::Spec) -> usize {
        if self.name_map.contains_key(name) {
            self.name_map.get(name).expect("Name Not found").clone()
        } else {
            let rsrc = self.factory.new_resource(spec);
            self.resource_map.push(rsrc);
            let ind = self.resource_map.len() - 1;
            self.name_map.insert(name.to_string(), ind);
            ind
        }
    }
}

pub struct AssetManager {
    pub shader_mgr: ResourceManger<ShaderFactory>,
    pub gl_buff_mgr: ResourceManger<GLBufferFactory>,
    pub texture_mgr: ResourceManger<TextureFactory>,
    asset_name_map: HashMap<String, usize>,
    asset_library: Vec<Asset>,
}

impl Default for AssetManager {
    fn default() -> Self {
        AssetManager {
            shader_mgr: ResourceManger::default(),
            gl_buff_mgr: ResourceManger::default(),
            texture_mgr: ResourceManger::default(),
            asset_name_map: HashMap::new(),
            asset_library: Vec::new(),
        }
    }
}

struct AssetMemo {
    shader_id: usize,
    gl_buff_id: usize,
    texture_ids: Vec<usize>,
}

impl AssetManager {
    pub fn get_resource(&self, id: usize) -> Asset {
        self.asset_library[id].clone()
    }

    pub fn add_resource(&mut self, name: &str, spec: AssetSpec) -> usize {
        if self.asset_name_map.contains_key(name) {
            self.asset_name_map.get(name).expect("Name Not found").clone()
        } else {
            let shader_id = self.shader_mgr.add_resource(&spec.shader.0, spec.shader.1);
            let model_id = self.gl_buff_mgr.add_resource(&spec.mesh.0, spec.mesh.1);
            let shader = *self.shader_mgr.get_resource(shader_id);
            let model = *self.gl_buff_mgr.get_resource(model_id);
            let textures: Vec<Texture> = spec.textures.iter().map(|tex_spec| {
                let tex_id = self.texture_mgr.add_resource(&tex_spec.0, tex_spec.1.clone());
                *self.texture_mgr.get_resource(tex_id)
            }).collect();
            if textures.len() == 0 {
                self.asset_library.push(Asset::new_textureless(shader,model));
            } else {
                self.asset_library.push(Asset::new(shader, model, textures));
            }
            let ind = self.asset_library.len() - 1;
            self.asset_name_map.insert(name.to_string(), ind);
            ind
        }
    }
}

