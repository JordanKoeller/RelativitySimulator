use crate::graphics::{PixelSpec, TextureBuilder, TextureId};
use crate::utils::*;

use std::ptr;

use gl;

static MAX_FRAMEBUFFER_SIZE: i32 = 8192;

pub struct FramebufferSpec {
  pub dims: Vec2I,
  pub samples: u32,
  pub swapchain_target: bool,
}

pub struct Framebuffer {
  pub spec: FramebufferSpec,
  color_attachment: u32,
  depth_attachment: u32,
  id: RwAssetRef<u32>,
}

impl Framebuffer {
  pub fn new(spec: FramebufferSpec) -> Framebuffer {
    let mut ret = Framebuffer {
      spec,
      color_attachment: 0,
      depth_attachment: 0,
      id: RwAssetRef::new(0),
    };
    ret.initialize();
    ret
  }

  pub fn from_dims(w: i32, h: i32) -> Framebuffer {
    let spec = FramebufferSpec {
      dims: Vec2I::new(w, h),
      samples: 1,
      swapchain_target: false,
    };
    Framebuffer::new(spec)
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, self.id());
      gl::Viewport(0, 0, self.spec.dims.x, self.spec.dims.y);
    }
  }

  pub fn bind_texture_slot(&self, slot: u32) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + slot);
      gl::BindTexture(gl::TEXTURE_2D, self.color_attachment);
    }
  }

  pub fn unbind_texture_slot(&self, slot: u32) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + slot);
      gl::BindTexture(gl::TEXTURE_2D, 0);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
  }

  pub fn id(&self) -> u32 {
    *self.id.get()
  }

  fn initialize(&mut self) {
    let mut id = self.id();
    unsafe {
      // Create Framebuffer
      gl::CreateFramebuffers(1, &mut id);
      gl::BindFramebuffer(gl::FRAMEBUFFER, id);

      // Color texture buffer
      gl::CreateTextures(gl::TEXTURE_2D, 1, &mut self.color_attachment);
      gl::BindTexture(gl::TEXTURE_2D, self.color_attachment);
      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA8 as i32,
        self.spec.dims.x,
        self.spec.dims.y,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        ptr::null(),
      );
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
      gl::BindTexture(gl::TEXTURE_2D, 0);
      gl::FramebufferTexture2D(
        gl::FRAMEBUFFER,
        gl::COLOR_ATTACHMENT0,
        gl::TEXTURE_2D,
        self.color_attachment,
        0,
      );

      // Time to do a depth buffer
      gl::CreateTextures(gl::TEXTURE_2D, 1, &mut self.depth_attachment);
      gl::BindTexture(gl::TEXTURE_2D, self.depth_attachment);
      gl::TexStorage2D(
        gl::TEXTURE_2D,
        1,
        gl::DEPTH24_STENCIL8,
        self.spec.dims.x,
        self.spec.dims.y,
      );
      gl::FramebufferTexture2D(
        gl::FRAMEBUFFER,
        gl::DEPTH_STENCIL_ATTACHMENT,
        gl::TEXTURE_2D,
        self.depth_attachment,
        0,
      );

      // TOOD: Assert complete
      gl::BindFramebuffer(gl::FRAMEBUFFER, id);
      let framebuffer_status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
      if framebuffer_status != gl::FRAMEBUFFER_COMPLETE {
        println!("Frame buffer is not ready! {}", framebuffer_status);
      }
    }

    self.id.set(id);
  }
}

impl Drop for Framebuffer {
  fn drop(&mut self) {
    let mut id = self.id();
    if self.id() != 0 {
      unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, id);
        gl::DeleteFramebuffers(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, self.color_attachment);
        gl::DeleteTextures(1, &mut self.color_attachment);
        gl::BindTexture(gl::TEXTURE_2D, self.depth_attachment);
        gl::DeleteTextures(1, &mut self.depth_attachment);
      }
      self.color_attachment = 0;
      self.depth_attachment = 0;
    }
  }
}
