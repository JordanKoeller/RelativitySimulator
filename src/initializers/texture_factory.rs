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
use renderer::{Texture, TextureType};

pub struct TextureFactory {}

impl Default for TextureFactory {
    fn default() -> TextureFactory {
        TextureFactory {}
    }
}

impl TextureFactory {
    fn new_2d_map(&self, spec: TextureSpec) -> Texture {
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

    fn new_1d_map(&self, spec: TextureSpec) -> Texture {
        self.new_2d_map(spec) //TODO: Make this an actual function
    }

    fn new_3d_map(&self, spec: TextureSpec) -> Texture {
        let mut tex_id = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, tex_id);
        }
        let suffixes = vec![
            "right".to_string(),
            "left".to_string(),
            "top".to_string(),
            "bottom".to_string(),
            "back".to_string(),
            "front".to_string(),
        ];
        let mut ind = 0;
        for suff in suffixes.iter() {
            let fname = format!("{}/{}.{}", spec.path, suff, "jpg");
            let img = image::open(&Path::new(&fname)).expect(&format!("Texture {} failed to load", fname));
            let data = img.raw_pixels();
            let format = match img {
                ImageLuma8(_) => gl::RED,
                ImageLumaA8(_) => gl::RG,
                ImageRgb8(_) => gl::RGB,
                ImageRgba8(_) => gl::RGBA,
            };
            unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + ind as u32,
                    0,
                    format as i32,
                    img.width() as i32,
                    img.height() as i32,
                    0,
                    format,
                    gl::UNSIGNED_BYTE,
                    &data[0] as *const u8 as *const c_void,
                );
            }
            ind += 1;
        }
        unsafe {
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        }
        spec.texture_type.to_value(tex_id)
    }
}

impl Factory for TextureFactory {
    type Resource = Texture;
    type Spec = TextureSpec;
    fn new_resource(&self, spec: Self::Spec) -> Self::Resource {
        match spec.texture_type {
            TextureType::CubeMap => self.new_3d_map(spec),
            TextureType::Texture1D => self.new_1d_map(spec),
            _ => self.new_2d_map(spec),
        }
    }
}
