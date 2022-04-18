use crate::datastructures::KeyValueBuilder;
use crate::utils::RwAssetRef;

use super::{VertexArray, VertexArrayId, DataBuffer, IndexBuffer, IndexBufferBuilder, DataBufferBuilder};

pub struct VertexArrayBuilder {
    id: RwAssetRef<u32>,
    index_buffer_builder: Option<IndexBufferBuilder>,
    vertex_buffer_builder: Option<DataBufferBuilder>,
    instancing_buffer_builder: Option<DataBufferBuilder>,

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