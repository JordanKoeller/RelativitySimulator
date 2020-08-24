use initializers::{AssetManager, AssetSpec, AttributeTypes, GLSpec, NormalShaderSpec, TextureSpec};
use renderer::Asset;
use renderer::{BaseRenderable, TextureType, UniformManager, UniformValue};
use utils::{translate, Vec3F};

const TEXTURE_CUBE_VERTICES: [f32; 180] = [
    // positions       // normals        // texture coords
    -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, -0.5,
    0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5,
    0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5, 0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0,
    0.0, -0.5, 0.5, -0.5, 1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0,
    -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5,
    -0.5, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5,
    1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0,
    1.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5,
    0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0,
];

const TEXTURE_CUBE_INDICES: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];

pub struct TexturedBlock {
    pub asset: Asset,
    pub uniforms: UniformManager,
}

impl TexturedBlock {
    pub fn new(position: Vec3F, texture_path: &str, library: &mut AssetManager) -> TexturedBlock {
        let mut mgr = UniformManager::new();
        mgr.set("model", UniformValue::Mat4(translate(position)));
        let asset_spec = AssetSpec::new_with_textures(
            (
                file!().to_string(),
                Box::new(NormalShaderSpec::new(
                    "shaders/textured_block/vs.glsl",
                    "shaders/textured_block/fs.glsl",
                )),
            ),
            (
                file!().to_string(),
                GLSpec::new(
                    TEXTURE_CUBE_VERTICES.to_vec(),
                    TEXTURE_CUBE_INDICES.to_vec(),
                    vec![AttributeTypes::Points, AttributeTypes::UVCoords],
                ),
            ),
            vec![(
                file!().to_string(),
                TextureSpec::new(texture_path, TextureType::DiffuseMap),
            )],
        );
        let asset_id = library.add_resource(file!(), asset_spec);

        TexturedBlock {
            asset: library.get_resource(asset_id),
            uniforms: mgr,
        }
    }
}

impl BaseRenderable for TexturedBlock {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn uniform_manager(&self) -> &UniformManager {
        &self.uniforms
    }
    fn uniform_manager_mut(&mut self) -> &mut UniformManager {
        &mut self.uniforms
    }
}
