#[derive(Clone, Eq, PartialEq, Copy)]
pub enum Texture {
    DiffuseMap(u32),
    NormalMap(u32),
    SpecularMap(u32),
    ParallaxMap(u32),
    HeightMap(u32),
    Texture1D(u32),
    Texture2D(u32),
    CubeMap(u32),
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
    CubeMap,
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
            TextureType::CubeMap => Texture::CubeMap(value),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            TextureType::DiffuseMap => "diffuseMap".to_string(),
            TextureType::NormalMap => "normalMap".to_string(),
            TextureType::SpecularMap => "specularMap".to_string(),
            TextureType::ParallaxMap => "parallaxMap".to_string(),
            TextureType::HeightMap => "heightMap".to_string(),
            TextureType::Texture1D => "texture1D".to_string(),
            TextureType::Texture2D => "texture2D".to_string(),
            TextureType::CubeMap => "texture3D".to_string(),
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
            Texture::CubeMap(_) => TextureType::CubeMap,
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Texture::DiffuseMap(v) => v.clone(),
            Texture::NormalMap(v) => v.clone(),
            Texture::SpecularMap(v) => v.clone(),
            Texture::ParallaxMap(v) => v.clone(),
            Texture::HeightMap(v) => v.clone(),
            Texture::Texture1D(v) => v.clone(),
            Texture::Texture2D(v) => v.clone(),
            Texture::CubeMap(v) => v.clone(),
        }
    }
}
