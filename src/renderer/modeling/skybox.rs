use initializers::{AssetManager, AssetSpec, AttributeTypes, GLSpec, NormalShaderSpec, TextureSpec};
use renderer::Asset;
use renderer::{BaseRenderable, IRenderable, Shader, Texture, TextureType, UniformManager, UniformValue};
use utils::{translate, Color, Vec3F};

use gl;

const SKYBOX_VERTICES: [f32; 108] = [
    -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0,
    1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0,
    -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0,
    -1.0, 1.0, 1.0, -1.0, 1.0,
];

pub struct Skybox {
    pub asset: Asset,
    pub uniforms: UniformManager,
}

impl Skybox {
    pub fn new(dirpath: &str, library: &mut AssetManager) -> Skybox {
        let mgr = UniformManager::new();
        // mgr.set("color", UniformValue::Vec3(color));
        let skybox_inds = (0..36).collect();
        let asset_spec = AssetSpec::new_with_textures(
            (
                file!().to_string(),
                Box::new(NormalShaderSpec::new(
                    "shaders/skybox/skybox.vs",
                    "shaders/skybox/skybox.fs",
                )),
            ),
            (
                file!().to_string(),
                GLSpec::new(SKYBOX_VERTICES.to_vec(), skybox_inds, vec![AttributeTypes::Points]),
            ),
            vec![(file!().to_string(), TextureSpec::new(dirpath, TextureType::CubeMap))],
        );

        let asset_id = library.add_resource(file!(), asset_spec);

        Skybox {
            asset: library.get_resource(asset_id),
            uniforms: mgr,
        }
    }
}

// impl BaseRenderable for Skybox {
//     fn asset(&self) -> &Asset {
//         &self.asset
//     }
//     fn uniform_manager(&self) -> &UniformManager {
//         &self.uniforms
//     }
//     fn uniform_manager_mut(&mut self) -> &mut UniformManager {
//         &mut self.uniforms
//     }
// }

impl IRenderable for Skybox {
    fn uniform_manager(&self) -> &UniformManager {
        &self.uniforms
    }
    fn uniform_manager_mut(&mut self) -> &mut UniformManager {
        &mut self.uniforms
    }
    fn render(&self) {
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
        }
        self.asset.draw(self.uniform_manager());
        unsafe {
            gl::DepthFunc(gl::LESS);
        }
        // gl::BindVertexArray(self.vao);
        // gl::ActiveTexture(gl::TEXTURE0);
        // gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture_id);
        // gl::DrawArrays(gl::TRIANGLES, 0, 36);
        // gl::BindVertexArray(0);
        // gl::DepthFunc(gl::LESS);
    }

    fn shader(&self) -> &Shader {
        &self.asset.shader
    }
}
