use crate::datastructures::KeyValueBuilder;
use crate::utils::RwAssetRef;

use super::{DataBuffer, DataBufferBuilder, IndexBuffer, IndexBufferBuilder, VertexArray, VertexArrayId};

pub struct VertexArrayBuilder {
    id: RwAssetRef<u32>,
    index_buffer_builder: Option<IndexBufferBuilder>,
    vertex_buffer_builder: Option<DataBufferBuilder>,
    instancing_buffer_builder: Option<DataBufferBuilder>,
}

impl Default for VertexArrayBuilder {
    fn default() -> Self {
        Self {
            id: RwAssetRef::new(std::u32::MAX),
            index_buffer_builder: None,
            vertex_buffer_builder: None,
            instancing_buffer_builder: None,
        }
    }
}

impl VertexArrayBuilder {
    pub fn with_vertex_buffer(mut self, builder: DataBufferBuilder) -> Self {
        self.vertex_buffer_builder = Some(builder);
        self
    }

    pub fn with_index_buffer(mut self, builder: IndexBufferBuilder) -> Self {
        self.index_buffer_builder = Some(builder);
        self
    }
}

impl KeyValueBuilder for VertexArrayBuilder {
    type K = VertexArrayId;
    type V = VertexArray;

    fn key(&self) -> Self::K {
        VertexArrayId::new(self.id.ro_ref())
    }

    fn build(self) -> Self::V {
        if self.is_buildable() {
            let mut vao = VertexArray::new(
                self.vertex_buffer_builder.unwrap().build(),
                self.index_buffer_builder.unwrap().build(),
                self.id,
            );
            if let Some(instancing_buffer) = self.instancing_buffer_builder {
                vao.add_instancing_buffer(instancing_buffer.build(), false);
            }
            vao.refresh();
            vao
        } else {
            panic!("Tried to build an unbuildable VertexArray!");
        }
    }

    fn is_buildable(&self) -> bool {
        self.index_buffer_builder.is_some() && self.vertex_buffer_builder.is_some()
    }
}
