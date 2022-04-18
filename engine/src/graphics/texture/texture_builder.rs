use gl;
use gl::types::GLenum;

use crate::datastructures::KeyValueBuilder;
use crate::utils::{ReadAssetRef, RwAssetRef};

use super::texture_helpers;
use super::Texture;
use super::TextureBuffer;
use super::TextureId;

pub struct TextureBuilder {
    // File build first
    filename: Option<String>,

    //empty builder
    width: Option<u32>,
    height: Option<u32>,
    format: GLenum,
    texture_id: RwAssetRef<(u32, gl::types::GLenum)>,
}

impl Default for TextureBuilder {
    fn default() -> Self {
        TextureBuilder {
            filename: None,
            width: None,
            height: None,
            format: gl::RGB,
            texture_id: RwAssetRef::new((std::u32::MAX, gl::TEXTURE_2D)),
        }
    }
}

impl KeyValueBuilder for TextureBuilder {
    type K = TextureId;
    type V = Texture;

    fn build(mut self) -> Self::V {
        if !self.is_buildable() {
            panic!("Tried to build a texture from a incomplete builder!");
        }
        if let Some(filename) = self.filename {
            let file_buffer = texture_helpers::load_file(&filename, true);
            let texture_id = texture_helpers::create_2d_buffer(
                &file_buffer.data,
                &file_buffer.width,
                &file_buffer.height,
                &file_buffer.encoding,
            );
            self.texture_id.set((texture_id, gl::TEXTURE_2D));
            Texture::new(self.texture_id, file_buffer)
        } else {
            let pixel_size = match self.format {
                gl::RED => 1,
                gl::RG => 2,
                gl::RGB => 3,
                gl::RGBA => 4,
                _ => 3,
            };
            let data = Vec::with_capacity((self.width.unwrap() * self.height.unwrap() * pixel_size) as usize);
            let buffer = TextureBuffer {
                data,
                width: self.width.unwrap(),
                height: self.height.unwrap(),
                encoding: self.format,
            };
            let texture_id =
                texture_helpers::create_2d_buffer(&buffer.data, &buffer.width, &buffer.height, &buffer.encoding);
            self.texture_id.set((texture_id, gl::TEXTURE_2D));
            Texture::new(self.texture_id, buffer)
        }
    }

    fn key(&self) -> Self::K {
        TextureId::new(self.texture_id.ro_ref())
    }

    fn is_buildable(&self) -> bool {
        self.filename.is_some() || (self.width.is_some() && self.height.is_some())
    }
}

impl TextureBuilder {
    pub fn set_file(mut self, fname: &str) -> Self {
        self.filename = Some(fname.to_string());
        self
    }

    pub fn set_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn set_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn set_format(mut self, format: PixelSpec) -> Self {
        self.format = format.get_gl_enum();
        self
    }
}

pub enum PixelSpec {
    RED,
    RG,
    RGB,
    RGBA
}

impl PixelSpec {
    pub fn get_gl_enum(self) -> gl::types::GLenum {
        match self {
            PixelSpec::RED => gl::RED,
            PixelSpec::RG => gl::RG,
            PixelSpec::RGB => gl::RGB,
            PixelSpec::RGBA => gl::RGBA,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing::get_context;

    #[test]
    fn builder_knows_when_buildable() {
        let builder = TextureBuilder::default();
        assert_eq!(builder.is_buildable(), false);
        let builder = builder.set_file("test_resources/test_texture.png");
        assert_eq!(builder.is_buildable(), true);
        let builder = TextureBuilder::default();
        assert_eq!(builder.is_buildable(), false);
        let builder = builder.set_height(100);
        assert_eq!(builder.is_buildable(), false);
        let builder = builder.set_width(100);
        assert_eq!(builder.is_buildable(), true);
        let builder = builder.set_format(PixelSpec::RGB);
        assert_eq!(builder.is_buildable(), true);
    }

    #[test]
    fn builder_can_build_texture_from_file() {
        let _ctx = get_context();
        let builder = TextureBuilder::default();
        assert_eq!(builder.is_buildable(), false);
        let builder = builder.set_file("test_resources/test_texture.png");
        assert_eq!(builder.is_buildable(), true);
        let texture = builder.build();
        assert_ne!(texture.id(), std::u32::MAX);
    }
}