use gl;

use super::TextureId;
use crate::utils::{ReadAssetRef, RwAssetRef, RGB};

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

  pub fn width(&self) -> usize {
    self.buffer.width as usize
  }

  pub fn height(&self) -> usize {
    self.buffer.height as usize
  }

  pub fn pixels(&self) -> TextureView<'_> {
    TextureView(&self.buffer)
  }
}

pub struct TextureView<'a>(&'a TextureBuffer);

impl<'a> std::ops::Index<&[usize; 2]> for TextureView<'a> {
  type Output = [u8];

  fn index(&self, index: &[usize; 2]) -> &Self::Output {
    if index[0] < self.width() && index[1] < self.height() {
      let x_i = 3 * (index[0] * (self.0.width as usize) + index[1]);
      return &self.0.data[x_i..x_i + 3];
    }
    panic!(
      "IndexOutOfBounds: coordinate ({}, {}) on TextureView with size ({}, {})",
      index[0],
      index[1],
      self.width(),
      self.height()
    )
  }
}

impl<'a> std::ops::Index<usize> for TextureView<'a> {
  type Output = [u8];

  fn index(&self, index: usize) -> &Self::Output {
    if index < self.len() {
      let x_i = 3 * index;
      return &self.0.data[x_i..x_i + 3];
    }
    panic!(
      "IndexOutOfBounds: coordinate ({}) on TextureView with size ({})",
      index,
      self.len()
    )
  }
}

impl<'a> TextureView<'a> {
  pub fn len(&self) -> usize {
    self.0.data.len() / 3
  }

  pub fn width(&self) -> usize {
    self.0.width as usize
  }

  pub fn height(&self) -> usize {
    self.0.height as usize
  }
}
