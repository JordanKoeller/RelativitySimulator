use crate::datastructures::Registry;

use super::{ShaderBuilder, ShaderId, ShaderRegistry, TextureBuilder, TextureId, TextureRegistry, VertexArrayRegistry, VertexArrayBuilder, VertexArrayId};

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
    fn get_or_create_texture(&self, lookup_name: &str, builder: TextureBuilder) -> TextureId {
        self.textures.enqueue_builder(lookup_name, builder)
    }

    fn get_or_create_shader(&self, lookup_name: &str, builder: ShaderBuilder) -> ShaderId {
        self.shaders.enqueue_builder(lookup_name, builder)
    }

    fn get_or_create_vertex_array(&self, lookup_name: &str, builder: VertexArrayBuilder) -> VertexArrayId {
        self.buffers.enqueue_builder(lookup_name, builder)
    }

    fn get_texture(&self, lookup_name: &str) -> Option<TextureId> {
        self.textures.get_registry_id(lookup_name)
    }

    fn get_shader(&self, lookup_name: &str) -> Option<ShaderId> {
        self.shaders.get_registry_id(lookup_name)
    }

    fn get_vertex_array(&self, lookup_name: &str) -> Option<VertexArrayId> {
        self.buffers.get_registry_id(lookup_name)
    }

    fn flush_all(&mut self) {
        self.shaders.flush();
        self.textures.flush();
        self.buffers.flush();
    }
}
