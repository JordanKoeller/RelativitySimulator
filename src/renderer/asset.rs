// use renderer::{GLBuffer, ResourceManger, Shader, Texture, UniformManager, IShader};

// pub trait Asset {
//     fn buffer_id(&self) -> &str;
//     fn shader_id(&self) -> &str;
//     fn texture_ids(&self) -> Vec<&str>;
//     fn uniform_manager(&self) -> &UniformManager;
//     fn self_draw(
//         &self,
//         shader_mgr: &ResourceManger<Shader>,
//         txtr_mgr: &ResourceManger<Texture>,
//         mesh_mgr: &ResourceManger<GLBuffer>,
//     ) -> bool {
//         let shader = shader_mgr.get_resource(self.shader_id());
//         let _textures = self.texture_ids().iter().map(|e| txtr_mgr.get_resource(e));
//         let buffer = mesh_mgr.get_resource(self.buffer_id());
//         for (unif_name, unif_value) in self.uniform_manager().get_all() {
//             shader.set_uniform(unif_name, unif_value);
//         }
//         // Need to figure out what to do with textures.
//         buffer.draw();
//         true
//     }
// }

// pub struct ColoredAsset {
//     buffer_id: String,
//     shader_id: String,
//     uniform_manager: UniformManager,
// }

// impl ColoredAsset {
//     pub fn new(buffer_id: String, shader_id: String) -> ColoredAsset {
//         ColoredAsset {
//             buffer_id,
//             shader_id,
//             uniform_manager: UniformManager::new(),
//         }
//     }
// }

// impl Asset for ColoredAsset {
//     fn buffer_id(&self) -> &str {
//         &self.buffer_id
//     }
//     fn shader_id(&self) -> &str {
//         &self.shader_id
//     }
//     fn texture_ids(&self) -> Vec<&str> {
//         Vec::new()
//     }
//     fn uniform_manager(&self) -> &UniformManager {
//         &self.uniform_manager
//     }
// }
use renderer::{Shader, IShader, GLBuffer, Texture, UniformManager};


#[derive(Clone)]
pub struct Asset {
    pub shader: Shader,
    pub buffer: GLBuffer,
    pub textures: Vec<Texture>,
}

impl Asset {
    pub fn new(shader: Shader, buffer: GLBuffer, textures: Vec<Texture>) -> Asset {
        Asset {
            shader,
            buffer,
            textures,
        }
    }

    pub fn new_textureless(shader: Shader, buffer: GLBuffer) -> Asset {
        Asset {
            shader,
            buffer,
            textures: Vec::default(),
        }
    }

    pub fn draw(&self, uniforms: &UniformManager) {
        // self.shader.use_program();
        for u in uniforms.get_all() {
            self.shader.set_uniform(u.0, u.1);
        }
        // for t in self.textures.iter() {
            
        // }
        self.buffer.draw();
    }
}