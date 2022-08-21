use std::hash::Hash;

use crate::utils::ReadAssetRef;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextureId(ReadAssetRef<(u32, gl::types::GLenum, String)>);

impl TextureId {
  pub fn new(v: ReadAssetRef<(u32, gl::types::GLenum, String)>) -> Self {
    Self(v)
  }

  pub fn bind(&self, slot: u32) {
    let (id, texture_type, _) = *self.0.get();
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + slot);
      gl::BindTexture(texture_type, id);
    }
  }

  pub fn unbind(&self, slot: u32) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + slot);
      gl::BindTexture(gl::TEXTURE_2D, 0u32);
    }
  }

  pub fn id(&self) -> u32 {
    self.0.get().0
  }

  pub fn name(&self) -> String {
    self.0.get().2.clone()
  }
}
