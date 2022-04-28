use crate::datastructures::Registry;

use super::{
    Shader, ShaderBuilder, ShaderId, ShaderRegistry, TextureBuilder, TextureId, TextureRegistry, VertexArray,
    VertexArrayBuilder, VertexArrayId, VertexArrayRegistry,
};

pub struct AssetLibrary {
    shaders: ShaderRegistry,
    textures: TextureRegistry,
    buffers: VertexArrayRegistry,
}

impl Default for AssetLibrary {
    fn default() -> Self {
        Self {
            shaders: ShaderRegistry::create(),
            textures: TextureRegistry::create(),
            buffers: VertexArrayRegistry::create(),
        }
    }
}

impl AssetLibrary {
    pub fn get_or_create_texture(&self, lookup_name: &str, builder: TextureBuilder) -> TextureId {
        self.textures.enqueue_builder(lookup_name, builder)
    }

    pub fn get_or_create_shader(&self, lookup_name: &str, builder: ShaderBuilder) -> ShaderId {
        self.shaders.enqueue_builder(lookup_name, builder)
    }

    pub fn get_or_create_vertex_array(&self, lookup_name: &str, builder: VertexArrayBuilder) -> VertexArrayId {
        self.buffers.enqueue_builder(lookup_name, builder)
    }

    pub fn get_texture_id(&self, lookup_name: &str) -> Option<TextureId> {
        self.textures.get_registry_id(lookup_name)
    }

    pub fn get_shader_id(&self, lookup_name: &str) -> Option<ShaderId> {
        self.shaders.get_registry_id(lookup_name)
    }

    pub fn get_vertex_array_id(&self, lookup_name: &str) -> Option<VertexArrayId> {
        self.buffers.get_registry_id(lookup_name)
    }

    pub fn get_shader_mut(&mut self, shader_id: &ShaderId) -> Option<&mut Shader> {
        self.shaders.fetch_mut(shader_id)
    }

    pub fn get_shader(&self, shader_id: &ShaderId) -> Option<&Shader> {
        self.shaders.fetch(shader_id)
    }

    pub fn get_vertex_array(&self, vertex_array_id: &VertexArrayId) -> Option<&VertexArray> {
        self.buffers.fetch(vertex_array_id)
    }

    pub fn flush_all(&mut self) {
        self.shaders.flush();
        self.textures.flush();
        self.buffers.flush();
    }
}
