use std::hash::Hash;

use crate::utils::ReadAssetRef;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct VertexArrayId(ReadAssetRef<u32>);

impl VertexArrayId {
    pub fn new(v: ReadAssetRef<u32>) -> Self {
        Self(v)
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.get());
        }
    }

    pub fn get(&self) -> u32 {
        *self.0.get()
    }
}
