use crate::datastructures::{GenericRegistry, Registry, RegistryItem};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::{
  Shader, ShaderBuilder, ShaderId, ShaderRegistry, TextureBuilder, TextureId, TextureRegistry, VertexArray,
  VertexArrayBuilder, VertexArrayId, VertexArrayRegistry,
};

pub trait Assets<T>
where
  T: RegistryItem + 'static,
{
  // Only need to connect to the registry for the particular asset type
  // to the trait and then the 4 interface functions can be defined in the trait

  fn registry(&self) -> &GenericRegistry<T>;
  fn registry_mut(&mut self) -> &mut GenericRegistry<T>;

  fn get_or_create<F: Fn() -> T>(&self, lookup_name: &str, func: F) -> T::K {
    if let Some(key) = self.registry().get_registry_id(lookup_name) {
      key
    } else {
      self.registry().enqueue_builder(lookup_name, func())
    }
  }

  fn get_else(&self, lookup_name: &str, builder: T) -> T::K {
    if let Some(key) = self.registry().get_registry_id(lookup_name) {
      key
    } else {
      self.registry().enqueue_builder(lookup_name, builder)
    }
  }

  fn get_asset_id(&self, lookup_name: &str) -> Option<T::K> {
    self.registry().get_registry_id(lookup_name)
  }

  fn get_asset(&self, key: &T::K) -> Option<RwLockReadGuard<'_, T::V>> {
    self.registry().fetch(key)
  }

  fn get_asset_mut(&self, key: &mut T::K) -> Option<RwLockWriteGuard<'_, T::V>> {
    self.registry().fetch_mut(key)
  }
}

pub struct AssetLibrary {
  shaders: ShaderRegistry,
  textures: TextureRegistry,
  buffers: VertexArrayRegistry,
}

impl Default for AssetLibrary {
  fn default() -> Self {
    Self {
      shaders: ShaderRegistry::create(),
      textures: TextureRegistry::create(),
      buffers: VertexArrayRegistry::create(),
    }
  }
}

impl AssetLibrary {
  
  pub fn flush_all(&mut self) {
    self.shaders.flush();
    self.textures.flush();
    self.buffers.flush();
  }

  pub fn get_shader(&self, name: &str) -> Option<ShaderId> {
    self.shaders.get_registry_id(name)
  }

  pub fn get_mesh_mut(&self, key: &mut VertexArrayId) -> Option<RwLockWriteGuard<'_, VertexArray>> {
    self.buffers.fetch_mut(key)
  }
}

// pub struct AssetLibrary {
//     shaders: ShaderRegistry,
//     textures: TextureRegistry,
//     buffers: VertexArrayRegistry,
// }

impl Assets<ShaderBuilder> for AssetLibrary {
  fn registry(&self) -> &GenericRegistry<ShaderBuilder> {
    &self.shaders
  }
  fn registry_mut(&mut self) -> &mut GenericRegistry<ShaderBuilder> {
    &mut self.shaders
  }
}

impl Assets<TextureBuilder> for AssetLibrary {
  fn registry(&self) -> &GenericRegistry<TextureBuilder> {
    &self.textures
  }
  fn registry_mut(&mut self) -> &mut GenericRegistry<TextureBuilder> {
    &mut self.textures
  }
}

impl Assets<VertexArrayBuilder> for AssetLibrary {
  fn registry(&self) -> &GenericRegistry<VertexArrayBuilder> {
    &self.buffers
  }
  fn registry_mut(&mut self) -> &mut GenericRegistry<VertexArrayBuilder> {
    &mut self.buffers
  }
}
