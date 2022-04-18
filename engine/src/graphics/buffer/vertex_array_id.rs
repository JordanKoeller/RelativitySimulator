use std::hash::Hash;

use crate::utils::ReadAssetRef;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct VertexArrayId(ReadAssetRef<u32>);

impl VertexArrayId {
    pub fn new(v: ReadAssetRef<u32>) -> Self {
        Self(v)
    }

    pub fn bind(&self) {

    }

    pub fn unbind(&self) {

    }

    pub fn id(&self) -> u32 {
        *self.0.get()
    }
}