use gl;
use std::path::Path;

use debug::*;
use lazy_static;

#[derive(Clone, Debug)]
enum TextureSource {
  File(String),
  Empty((u32, u32)),
  Null,
}

type TextureBuffer = (Vec<u8>, u32, u32, gl::types::GLenum);

pub trait TextureLike {
  fn bind(&self, slot: u32) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + slot);
      gl::BindTexture(self.texture_type(), self.id());
    }
  }

  fn id(&self) -> u32;
  fn texture_type(&self) -> gl::types::GLenum;
}

#[derive(Clone, Debug)]
pub struct Texture {
  source_data: TextureSource,
  width: u32,
  height: u32,
  id: u32,
}

impl Texture {
  pub fn from_file(path: &str) -> Texture {
    let (data, width, height, format) = texture_helpers::load_file(path, true);
    let src = TextureSource::File(path.to_string());
    let id = texture_helpers::create_2d_buffer(&data, &width, &height, &format);
    Texture {
      source_data: src,
      width: width,
      height: height,
      id: id,
    }
  }

  pub fn pre_made(id: u32, width: u32, height: u32) -> Texture {
    Texture {
      source_data: TextureSource::Null,
      width,
      height,
      id
    }
  }
}

impl TextureLike for Texture {
  fn id(&self) -> u32 {
    self.id
  }
  fn texture_type(&self) -> gl::types::GLenum {
    gl::TEXTURE_2D
  }
}

// impl Drop for Texture {
//   fn drop(&mut self) {
//     unsafe {
//       gl::DeleteTextures(1, &self.id);
//     }
//   }
// }

#[derive(Clone, Debug)]
pub struct CubeMap {
  source_data: TextureSource,
  width: u32,
  height: u32,
  id: u32,
}

impl TextureLike for CubeMap {
  fn id(&self) -> u32 {
    self.id
  }
  fn texture_type(&self) -> gl::types::GLenum {
    gl::TEXTURE_CUBE_MAP
  }
}

impl CubeMap {
  pub fn from_file(dirpath: &str) -> CubeMap {
    let faces = [
      "right.jpg",
      "left.jpg",
      "top.jpg",
      "bottom.jpg",
      "front.jpg",
      "back.jpg",
    ];
    let dir = Path::new(dirpath);
    let mut imgs = faces.iter().map(|file| {
      let full_path = dir.join(Path::new(file));
      texture_helpers::load_file(full_path.to_str().expect("Could not construct path for"), false)
    });
    let (id, width, height) = texture_helpers::create_cubemap_buffer(&mut imgs);
    CubeMap {
      source_data: TextureSource::File(dirpath.to_string()),
      width,
      height,
      id
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

  pub fn load_file(path: &str, flipv: bool) -> super::TextureBuffer {
    let img = image::open(&Path::new(path)).expect(&format!("Failed to load texture at {}", path));
    let img = if flipv {img.flipv()} else {img};
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

  pub fn create_cubemap_buffer<'a>(faces: &mut impl Iterator<Item = (Vec<u8>, u32, u32, gl::types::GLenum)>) -> (u32, u32, u32) {
    let mut texture = 0;
    let (mut ret_width, mut ret_height) = (0, 0);
    unsafe {
      gl::GenTextures(1, &mut texture);
      gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture);
    }
    faces.enumerate().for_each(|(i, (data, width, height, format))| unsafe {
      ret_width = width.clone();
      ret_height = height.clone();
      gl::TexImage2D(
        gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
        0,
        format as i32,
        width as i32,
        height as i32,
        0,
        format,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
      );
    });
    unsafe {
      gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
      gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
      gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
    }
    // unsafe {
    //   // load image, create texture and generate mipmaps
    //   gl::GenerateMipmap(gl::TEXTURE_2D);

    // }
    (texture, ret_width, ret_height)
  }
}

lazy_static! {
  pub static ref WHITE_TEXTURE: Texture = Texture::from_file("resources/textures/white.png");
}
