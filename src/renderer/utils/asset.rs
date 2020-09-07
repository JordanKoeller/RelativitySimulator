use renderer::utils::{Shader, IShader, GLBuffer, Texture, UniformManager};


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
        for t in self.textures.iter() {
            self.shader.set_texture(t);
        }
        self.buffer.draw();
    }
}