use gl;

#[derive(Clone)]
enum TextureSource {
  File(String),
  Empty((u32, u32)),
}

type TextureBuffer = (Vec<u8>, u32, u32, gl::types::GLenum);

#[derive(Clone)]
pub struct Texture {
  source_data: TextureSource,
  width: u32,
  height: u32,
  id: u32,
}

impl Texture {
  pub fn from_file(path: &str) -> Texture {
    let (data, width, height, format) = texture_helpers::load_file(path);
    let src = TextureSource::File(path.to_string());
    let id = texture_helpers::create_2d_buffer(&data, &width, &height, &format);
    Texture {
      source_data: src,
      width: width,
      height: height,
      id: id,
    }
  }

  pub fn bind(&self, slot: u32) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + slot);
      gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteTextures(1, &self.id);
    }
  }
}

mod texture_helpers {
  extern crate image;
  use gl;
  use image::DynamicImage::*;
  use image::GenericImage;
  use std::os::raw::c_void;
  use std::path::Path;

  pub fn load_file(path: &str) -> super::TextureBuffer {
    let img = image::open(&Path::new(path)).expect(&format!("Failed to load texture at {}", path));
    let data = img.raw_pixels();
    let fmt = match img {
      ImageLuma8(_) => gl::RED,
      ImageLumaA8(_) => gl::RG,
      ImageRgb8(_) => gl::RGB,
      ImageRgba8(_) => gl::RGBA,
    };

    (data, img.width(), img.height(), fmt)
  }

  pub fn create_2d_buffer(data: &Vec<u8>, width: &u32, height: &u32, format: &gl::types::GLenum) -> u32 {
    let mut texture = 0;
    unsafe {
      gl::GenTextures(1, &mut texture);
      gl::BindTexture(gl::TEXTURE_2D, texture);
      // load image, create texture and generate mipmaps
      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        *format as i32,
        *width as i32,
        *height as i32,
        0,
        *format,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
      );
      gl::GenerateMipmap(gl::TEXTURE_2D);

      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }
    texture
  }
}
