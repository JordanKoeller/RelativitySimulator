#[derive(Clone, Eq, PartialEq)]
pub enum Texture {
    DiffuseMap(u32),
    NormalMap(u32),
    SpecularMap(u32),
    ParallaxMap(u32),
    HeightMap(u32),
    Texture1D(u32),
    Texture2D(u32),
    Texture3D(u32),
}

#[derive(Clone, Eq, PartialEq)]
pub enum TextureType {
    DiffuseMap,
    NormalMap,
    SpecularMap,
    ParallaxMap,
    HeightMap,
    Texture1D,
    Texture2D,
    Texture3D
}

impl TextureType {
    pub fn to_value(&self, value: u32) -> Texture {
        match self {
            TextureType::DiffuseMap => Texture::DiffuseMap(value),
            TextureType::NormalMap => Texture::NormalMap(value),
            TextureType::SpecularMap => Texture::SpecularMap(value),
            TextureType::ParallaxMap => Texture::ParallaxMap(value),
            TextureType::HeightMap => Texture::HeightMap(value),
            TextureType::Texture1D => Texture::Texture1D(value),
            TextureType::Texture2D => Texture::Texture2D(value),
            TextureType::Texture3D => Texture::Texture3D(value),
        }
    }
}

impl Texture {
    pub fn to_type(&self) -> TextureType {
        match self {
            Texture::DiffuseMap(_) => TextureType::DiffuseMap,
            Texture::NormalMap(_) => TextureType::NormalMap,
            Texture::SpecularMap(_) => TextureType::SpecularMap,
            Texture::ParallaxMap(_) => TextureType::ParallaxMap,
            Texture::HeightMap(_) => TextureType::HeightMap,
            Texture::Texture1D(_) => TextureType::Texture1D,
            Texture::Texture2D(_) => TextureType::Texture2D,
            Texture::Texture3D(_) => TextureType::Texture3D,
        }
    }
}
