use initializers::{AssetManager, AssetSpec, GLSpec, NormalShaderSpec, AttributeTypes};
use renderer::Asset;
use renderer::{UniformManager, UniformValue, Shader, IRenderable, BaseRenderable};
use utils::{translate, Color, Vec3F};

const CUBE_VERTICES: [f32; 24] = [
    -0.5, -0.5, -0.5, //ftl 0
     0.5, -0.5, -0.5, //ftr 1
    -0.5,  0.5, -0.5, //fbl 2
     0.5,  0.5, -0.5, //fbr 3
    -0.5, -0.5,  0.5, //btl 4
     0.5, -0.5,  0.5, //btr 5
    -0.5,  0.5,  0.5, //bbl 6
     0.5,  0.5,  0.5, //bbr 7
];

const CUBE_INDICES: [u32; 36] = [
    // front face
    0, 1, 2,
    1, 3, 2,
    // left face
    0, 4, 6,
    6, 2, 0,
    // right face
    1, 5, 7,
    1, 7, 3,
    // top face
    0, 1, 5,
    0, 5, 4,
    // bottom face
    2, 3, 7,
    2, 7, 6,
    // back face
    4, 5, 6,
    6, 7, 6
];

pub struct ColoredCube {
    pub asset: Asset,
    pub uniforms: UniformManager
}

impl ColoredCube {
    pub fn new(position: Vec3F, color: Color, library: &mut AssetManager) -> ColoredCube {
        let mut mgr = UniformManager::new();
        mgr.set("color", UniformValue::Vec3(color));
        mgr.set("model", UniformValue::Mat4(translate(position)));
        let asset_spec = AssetSpec::new(
            ("world".to_string(),Box::new(NormalShaderSpec::new("shaders/1.model_loading.vs", "shaders/1.model_loading.fs"))),
            ("cube".to_string(), GLSpec::new(CUBE_VERTICES.to_vec(), CUBE_INDICES.to_vec(), vec![AttributeTypes::Points]))); 
        let asset_id = library.add_resource("cube", asset_spec);

        ColoredCube {
            asset: library.get_resource(asset_id),
            uniforms: mgr
        }
    }

    pub fn draw(&mut self) {
        self.asset.draw(&mut self.uniforms);
    }

    pub fn shader(&mut self) -> &mut Shader {
        &mut self.asset.shader
    }
}

impl BaseRenderable for ColoredCube {
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
