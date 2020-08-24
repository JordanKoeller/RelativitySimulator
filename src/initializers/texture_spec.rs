// TODO: Fill in later

use renderer::TextureType;

#[derive(Clone)]
pub struct TextureSpec {
    pub path: String,
    pub texture_type: TextureType
}

impl TextureSpec {
    pub fn new(name: &str, texture_type: TextureType) -> TextureSpec {
        TextureSpec {
            path: name.to_string(),
            texture_type: texture_type
        }
    }
}