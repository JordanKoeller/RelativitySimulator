use std::hash::Hash;

use crate::utils::ReadAssetRef;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ShaderId(ReadAssetRef<u32>);

impl ShaderId {
  pub fn new(v: ReadAssetRef<u32>) -> Self {
    Self(v)
  }

  pub fn get(&self) -> u32 {
    *self.0.get()
  }
}
