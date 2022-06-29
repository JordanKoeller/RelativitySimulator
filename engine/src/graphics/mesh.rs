use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

use super::{ShaderId, VertexArrayId};
use crate::utils::Vec3F;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MeshComponent {
    pub vertex_array_id: VertexArrayId,
    pub shader_id: ShaderId,
}

impl MeshComponent {
    pub fn new(vertex_array_id: VertexArrayId, shader_id: ShaderId) -> Self {
        Self {
            vertex_array_id,
            shader_id,
        }
    }
}

impl Component for MeshComponent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
