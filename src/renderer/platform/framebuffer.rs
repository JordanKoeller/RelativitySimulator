use utils::*;
use renderer::Texture;

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
  id: u32,
}

impl Framebuffer {
  pub fn new(spec: FramebufferSpec) -> Framebuffer {
    let mut ret = Framebuffer {
      spec,
      color_attachment: 0,
      depth_attachment: 0,
      id: 0,
    };
    ret.invalidate();
    ret
  }

  pub fn dims(w: i32, h: i32) -> Framebuffer {
    let spec = FramebufferSpec {
      dims: Vec2I::new(w, h),
      samples: 1,
      swapchain_target: false
    };
    Framebuffer::new(spec)
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
      gl::Viewport(0, 0, self.spec.dims.x, self.spec.dims.y);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
  }

  pub fn resize(&mut self, dim: Vec2I) {
    if dim.x == 0 || dim.y == 0 || dim.x > MAX_FRAMEBUFFER_SIZE || dim.y > MAX_FRAMEBUFFER_SIZE {
      println!("Invalid Framebuffer size. Silently ignoring resize call.");
      return;
    }
    self.spec.dims = dim;
    self.invalidate();
  }

  pub fn texture(&self) -> Texture {
    Texture::pre_made(self.color_attachment, self.spec.dims.x as u32, self.spec.dims.y as u32)
  }

  fn invalidate(&mut self) {
    unsafe {
      if self.id != 0 {
        gl::DeleteFramebuffers(1, &mut self.id);
        gl::DeleteTextures(1, &mut self.color_attachment);
        gl::DeleteTextures(1, &mut self.depth_attachment);
      }
      gl::CreateFramebuffers(1, &mut self.id);
      gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);

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

      gl::FramebufferTexture2D(
        gl::FRAMEBUFFER,
        gl::COLOR_ATTACHMENT0,
        gl::TEXTURE_2D,
        self.color_attachment,
        0,
      );
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
      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
  }
}
