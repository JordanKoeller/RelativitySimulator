use initializers::{ShaderSpec, GLSpec};

pub struct AssetSpec {
    pub shader: (String, Box<dyn ShaderSpec>),
    pub mesh: (String, GLSpec),
    //pub textures: Vec<(String, TextureSpec)>
}

impl AssetSpec {
    pub fn new(shader_spec: (String, Box<dyn ShaderSpec>), mesh_spec: (String, GLSpec)) -> AssetSpec {
        AssetSpec {
            shader: shader_spec,
            mesh: mesh_spec
        }
    }
}