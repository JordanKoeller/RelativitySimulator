use gl;

use super::TextureId;
use crate::utils::{ReadAssetRef, RwAssetRef};

#[derive(Clone, Debug)]
pub struct TextureBuffer {
  pub data: Vec<u8>,
  pub width: u32,
  pub height: u32,
  pub encoding: gl::types::GLenum,
}

#[derive(Clone, Debug)]
pub struct Texture {
  id: RwAssetRef<(u32, gl::types::GLenum, String)>,
  buffer: TextureBuffer,
}

impl Texture {
  pub fn new(id: RwAssetRef<(u32, gl::types::GLenum, String)>, buffer: TextureBuffer) -> Self {
    Self { id, buffer }
  }

  pub fn bind(&self, slot: u32) {
    let (id, texture_type, _) = *self.id.get();
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + slot);
      gl::BindTexture(texture_type, id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, 0u32);
    }
  }

  pub fn id(&self) -> u32 {
    self.id.get().0
  }
}
