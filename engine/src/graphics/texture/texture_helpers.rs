extern crate image;
use gl;
use image::DynamicImage::*;
use image::GenericImage;
use std::os::raw::c_void;
use std::path::Path;

use super::TextureBuffer;

pub fn load_file(path: &str, flipv: bool) -> super::TextureBuffer {
    let img = image::open(&Path::new(path)).expect(&format!("Failed to load texture at {}", path));
    let img = if flipv { img.flipv() } else { img };
    let data = img.raw_pixels();
    let fmt = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
    };

    super::TextureBuffer {
        data,
        width: img.width(),
        height: img.height(),
        encoding: fmt,
    }
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

pub fn create_cubemap_buffer<'a>(faces: &mut impl Iterator<Item = TextureBuffer>, format: u32) -> (u32, TextureBuffer) {
    let mut texture_id = 0;
    let (mut w, mut h) = (0, 0);
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);
    }
    faces.enumerate().for_each(|(i, buf)| unsafe {
        w = buf.width.clone();
        h = buf.height.clone();
        gl::TexImage2D(
            gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
            0,
            format as i32,
            buf.width as i32,
            buf.height as i32,
            0,
            format,
            gl::UNSIGNED_BYTE,
            &buf.data[0] as *const u8 as *const c_void,
        );
    });
    unsafe {
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
    }
    (
        texture_id,
        TextureBuffer {
            data: vec![],
            width: w,
            height: h,
            encoding: format,
        },
    )
}
