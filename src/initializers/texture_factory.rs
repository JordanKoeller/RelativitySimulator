// TODO: Fill in later
use std::os::raw::c_void;
use std::path::Path;

use gl;

extern crate image;
// use image;
use image::DynamicImage::*;
use image::GenericImage;

use initializers::Factory;
use initializers::TextureSpec;
use renderer::Texture;

pub struct TextureFactory {}

impl Default for TextureFactory {
    fn default() -> TextureFactory {
        TextureFactory {}
    }
}

impl Factory for TextureFactory {
    type Resource = Texture;
    type Spec = TextureSpec;
    fn new_resource(&self, spec: Self::Spec) -> Self::Resource {
        let mut tex_id = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
        }
        let img = image::open(&Path::new(&spec.path)).expect("Failed to load texture");
        let format = match img {
            ImageLuma8(_) => gl::RED,
            ImageLumaA8(_) => gl::RG,
            ImageRgb8(_) => gl::RGB,
            ImageRgba8(_) => gl::RGBA,
        };
        let data = img.raw_pixels();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        spec.texture_type.to_value(tex_id)
    }
}
