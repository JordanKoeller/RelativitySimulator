use std::hash::Hash;

use crate::utils::ReadAssetRef;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextureId(ReadAssetRef<(u32, gl::types::GLenum)>);

impl TextureId {
    pub fn new(v: ReadAssetRef<(u32, gl::types::GLenum)>) -> Self {
        Self(v)
    }

    pub fn bind(&self, slot: u32) {
        let (id, texture_type) = *self.0.get();
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
        self.0.get().0
    }
}