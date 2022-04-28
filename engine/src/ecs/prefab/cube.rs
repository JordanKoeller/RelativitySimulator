use std::ops::Deref;

use crate::ecs::{ComponentCache, PrefabBuilder, SystemUtilities};
use crate::graphics::{
    AttributeType, BufferConfig, BufferLayout, DataBufferBuilder, IndexBufferBuilder, MaterialComponent, MeshComponent,
    ShaderBuilder, TextureBuilder, VertexArrayBuilder,
};
use crate::physics::TransformComponent;
use crate::physics::{Collision, CollisionSummary};
use crate::utils::{swizzle_down, swizzle_up, Mat3F, Vec3F, Vec4F};
use specs::prelude::*;
use specs::{Component, VecStorage};

pub struct CubeState {
    texture_filename: String,
    position: Vec3F,
}

impl CubeState {
    pub fn new(filename: &str, position: Vec3F) -> Self {
        Self {
            texture_filename: filename.to_string(),
            position,
        }
    }
}

#[derive(Default)]
pub struct Cube {
    cache: ComponentCache,
}
impl PrefabBuilder for Cube {
    type PrefabState = CubeState;
    fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) {
        let mesh = self.cache.get_or(|| {
            let shader_id = api.assets().get_shader_id("default_texture").unwrap();
            let vai = VertexArrayBuilder::default()
                .set_index_buffer(IndexBufferBuilder::default().set_data(TEXTURE_CUBE_INDICES.to_vec()))
                .set_vertex_buffer(
                    DataBufferBuilder::default()
                        .set_data(TEXTURE_CUBE_VERTICES.to_vec())
                        .set_layout(BufferLayout::new(vec![
                            AttributeType::Float3,
                            AttributeType::Float3,
                            AttributeType::Float2,
                        ]))
                        .set_config(BufferConfig::static_vbo()),
                );
            let vai = api.assets().get_or_create_vertex_array("cube", vai);
            MeshComponent::new(vai, shader_id)
        });
        let texture_id = api.assets().get_or_create_texture(
            &state.texture_filename,
            TextureBuilder::default().set_file(&state.texture_filename),
        );
        let mut material = MaterialComponent::default();
        material.diffuse_texture(texture_id);
        material.ambient(Vec3F::new(1f32, 1f32, 1f32));
        let mut transform = TransformComponent::identity();
        transform.push_translation(state.position);
        api.entity_builder().with(material).with(transform).with(mesh).build();
    }
}

pub static TEXTURE_CUBE_VERTICES: [f32; 288] = [
    // positions                 // normals                // texture coords
    -0.5f32,
    -0.5f32,
    -0.5f32,
    0.0f32,
    0.0f32,
    -1.0f32,
    0.3333333f32,
    0.25f32,
    0.5f32,
    -0.5f32,
    -0.5f32,
    0.0f32,
    0.0f32,
    -1.0f32,
    0.6666666f32,
    0.25f32,
    0.5f32,
    0.5f32,
    -0.5f32,
    0.0f32,
    0.0f32,
    -1.0f32,
    0.6666666f32,
    0.50f32, // FRONT
    0.5f32,
    0.5f32,
    -0.5f32,
    0.0f32,
    0.0f32,
    -1.0f32,
    0.6666666f32,
    0.50f32,
    -0.5f32,
    0.5f32,
    -0.5f32,
    0.0f32,
    0.0f32,
    -1.0f32,
    0.3333333f32,
    0.50f32,
    -0.5f32,
    -0.5f32,
    -0.5f32,
    0.0f32,
    0.0f32,
    -1.0f32,
    0.3333333f32,
    0.25f32,
    -0.5f32,
    -0.5f32,
    0.5f32,
    0.0f32,
    0.0f32,
    1.0f32,
    0.3333333f32,
    1.0f32,
    0.5f32,
    -0.5f32,
    0.5f32,
    0.0f32,
    0.0f32,
    1.0f32,
    0.6666666f32,
    1.0f32,
    0.5f32,
    0.5f32,
    0.5f32,
    0.0f32,
    0.0f32,
    1.0f32,
    0.6666666f32,
    0.75f32, // BACK
    0.5f32,
    0.5f32,
    0.5f32,
    0.0f32,
    0.0f32,
    1.0f32,
    0.6666666f32,
    0.75f32,
    -0.5f32,
    0.5f32,
    0.5f32,
    0.0f32,
    0.0f32,
    1.0f32,
    0.3333333f32,
    0.75f32,
    -0.5f32,
    -0.5f32,
    0.5f32,
    0.0f32,
    0.0f32,
    1.0f32,
    0.3333333f32,
    1.0f32,
    -0.5f32,
    0.5f32,
    0.5f32,
    -1.0f32,
    0.0f32,
    0.0f32,
    0.33333333f32,
    0.5f32, // B
    -0.5f32,
    0.5f32,
    -0.5f32,
    -1.0f32,
    0.0f32,
    0.0f32,
    0.33333333f32,
    0.75f32, // A
    -0.5f32,
    -0.5f32,
    -0.5f32,
    -1.0f32,
    0.0f32,
    0.0f32,
    0.0000000f32,
    0.75f32, // C RIGHT
    -0.5f32,
    -0.5f32,
    -0.5f32,
    -1.0f32,
    0.0f32,
    0.0f32,
    0.0000000f32,
    0.75f32, // C
    -0.5f32,
    -0.5f32,
    0.5f32,
    -1.0f32,
    0.0f32,
    0.0f32,
    0.0000000f32,
    0.5f32, // D
    -0.5f32,
    0.5f32,
    0.5f32,
    -1.0f32,
    0.0f32,
    0.0f32,
    0.33333333f32,
    0.5f32, // B
    0.5f32,
    0.5f32,
    0.5f32,
    1.0f32,
    0.0f32,
    0.0f32,
    0.33333333f32,
    0.5f32, // B
    0.5f32,
    0.5f32,
    -0.5f32,
    1.0f32,
    0.0f32,
    0.0f32,
    0.33333333f32,
    0.75f32, // A
    0.5f32,
    -0.5f32,
    -0.5f32,
    1.0f32,
    0.0f32,
    0.0f32,
    0.0000000f32,
    0.75f32, // C LEFT
    0.5f32,
    -0.5f32,
    -0.5f32,
    1.0f32,
    0.0f32,
    0.0f32,
    0.0000000f32,
    0.75f32, // C
    0.5f32,
    -0.5f32,
    0.5f32,
    1.0f32,
    0.0f32,
    0.0f32,
    0.0000000f32,
    0.5f32, // D
    0.5f32,
    0.5f32,
    0.5f32,
    1.0f32,
    0.0f32,
    0.0f32,
    0.33333333f32,
    0.5f32, // B
    -0.5f32,
    -0.5f32,
    -0.5f32,
    0.0f32,
    -1.0f32,
    0.0f32,
    0.3333333f32,
    0.25f32,
    0.5f32,
    -0.5f32,
    -0.5f32,
    0.0f32,
    -1.0f32,
    0.0f32,
    0.6666666f32,
    0.25f32,
    0.5f32,
    -0.5f32,
    0.5f32,
    0.0f32,
    -1.0f32,
    0.0f32,
    0.6666666f32,
    0.0f32, // BOTTOM
    0.5f32,
    -0.5f32,
    0.5f32,
    0.0f32,
    -1.0f32,
    0.0f32,
    0.6666666f32,
    0.0f32,
    -0.5f32,
    -0.5f32,
    0.5f32,
    0.0f32,
    -1.0f32,
    0.0f32,
    0.3333333f32,
    0.0f32,
    -0.5f32,
    -0.5f32,
    -0.5f32,
    0.0f32,
    -1.0f32,
    0.0f32,
    0.3333333f32,
    0.25f32,
    -0.5f32,
    0.5f32,
    -0.5f32,
    0.0f32,
    1.0f32,
    0.0f32,
    0.3333333f32,
    0.75f32,
    0.5f32,
    0.5f32,
    -0.5f32,
    0.0f32,
    1.0f32,
    0.0f32,
    0.6666666f32,
    0.75f32,
    0.5f32,
    0.5f32,
    0.5f32,
    0.0f32,
    1.0f32,
    0.0f32,
    0.6666666f32,
    0.5f32,
    0.5f32,
    0.5f32,
    0.5f32,
    0.0f32,
    1.0f32,
    0.0f32,
    0.6666666f32,
    0.5f32, // TOP
    -0.5f32,
    0.5f32,
    0.5f32,
    0.0f32,
    1.0f32,
    0.0f32,
    0.3333333f32,
    0.5f32,
    -0.5f32,
    0.5f32,
    -0.5f32,
    0.0f32,
    1.0f32,
    0.0f32,
    0.3333333f32,
    0.75f32,
];

pub static TEXTURE_CUBE_INDICES: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];