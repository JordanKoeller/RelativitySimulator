use crate::ecs::{ComponentCache, PrefabBuilder, SystemUtilities};
use crate::graphics::{
    Assets, AttributeType, BufferConfig, BufferLayout, ColorSpace, DataBufferBuilder, IndexBufferBuilder,
    MaterialComponent, MeshComponent, ShaderBuilder, TextureBuilder, Uniform, VertexArrayBuilder,
};
use crate::physics::TransformComponent;
use specs::{Builder, Entity};

pub struct SkyboxPrefab {
    texture_filename: String,
}

impl SkyboxPrefab {
    pub fn new(filename: &str) -> Self {
        Self {
            texture_filename: filename.to_string(),
        }
    }
}

#[derive(Default)]
pub struct SkyboxBuilder {
    cache: ComponentCache,
}

impl PrefabBuilder for SkyboxBuilder {
    type PrefabState = SkyboxPrefab;

    fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) -> Entity {
        let mesh = self.cache.get_or(|| {
            let shader_id = api.assets().get_shader("skybox").unwrap();
            let vai = api.get_or_create("cubemap_mesh", || {
                VertexArrayBuilder::default()
                    .with_index_buffer(IndexBufferBuilder::default().with_data(SKYBOX_INDICES.to_vec()))
                    .with_vertex_buffer(
                        DataBufferBuilder::default()
                            .with_data(SKYBOX_VERTICES.to_vec())
                            .with_layout(BufferLayout::new(vec![AttributeType::Float3, AttributeType::Float2]))
                            .with_config(BufferConfig::static_vbo()),
                    )
            });
            MeshComponent::new(vai, shader_id)
        });
        let texture_id = api.assets().get_or_create(&state.texture_filename, || {
            TextureBuilder::default()
                .with_color_space(ColorSpace::SRGB)
                .with_file(&state.texture_filename)
                .set_is_cubemap(true)
        });
        let mut material = MaterialComponent::default();
        material.unknown_uniform("skybox", Uniform::CubeMap(texture_id));
        let transform = TransformComponent::identity();
        api.entity_builder().with(material).with(transform).with(mesh).build()
    }
}

pub static SKYBOX_VERTICES: [f32; 180] = [
    // positions                 // normals                // texture coords
    -0.5, -0.5, -0.5, 0.0, 0.0, // Bottom-left
    0.5, -0.5, -0.5, 1.0, 0.0, // bottom-right
    0.5, 0.5, -0.5, 1.0, 1.0, // top-right
    0.5, 0.5, -0.5, 1.0, 1.0, // top-right
    -0.5, 0.5, -0.5, 0.0, 1.0, // top-left
    -0.5, -0.5, -0.5, 0.0, 0.0, // bottom-left
    // Front face
    -0.5, -0.5, 0.5, 0.0, 0.0, // bottom-left
    0.5, 0.5, 0.5, 1.0, 1.0, // top-right
    0.5, -0.5, 0.5, 1.0, 0.0, // bottom-right
    0.5, 0.5, 0.5, 1.0, 1.0, // top-right
    -0.5, -0.5, 0.5, 0.0, 0.0, // bottom-left
    -0.5, 0.5, 0.5, 0.0, 1.0, // top-left
    // Left face
    -0.5, 0.5, 0.5, 1.0, 0.0, // top-right
    -0.5, -0.5, -0.5, 0.0, 1.0, // bottom-left
    -0.5, 0.5, -0.5, 1.0, 1.0, // top-left
    -0.5, -0.5, -0.5, 0.0, 1.0, // bottom-left
    -0.5, 0.5, 0.5, 1.0, 0.0, // top-right
    -0.5, -0.5, 0.5, 0.0, 0.0, // bottom-right
    // Right face
    0.5, 0.5, 0.5, 1.0, 0.0, // top-left
    0.5, 0.5, -0.5, 1.0, 1.0, // top-right
    0.5, -0.5, -0.5, 0.0, 1.0, // bottom-right
    0.5, -0.5, -0.5, 0.0, 1.0, // bottom-right
    0.5, -0.5, 0.5, 0.0, 0.0, // bottom-left
    0.5, 0.5, 0.5, 1.0, 0.0, // top-left
    // Bottom face
    -0.5, -0.5, -0.5, 0.0, 1.0, // top-right
    0.5, -0.5, 0.5, 1.0, 0.0, // bottom-left
    0.5, -0.5, -0.5, 1.0, 1.0, // top-left
    0.5, -0.5, 0.5, 1.0, 0.0, // bottom-left
    -0.5, -0.5, -0.5, 0.0, 1.0, // top-right
    -0.5, -0.5, 0.5, 0.0, 0.0, // bottom-right
    // Top face
    -0.5, 0.5, -0.5, 0.0, 1.0, // top-left
    0.5, 0.5, -0.5, 1.0, 1.0, // top-right
    0.5, 0.5, 0.5, 1.0, 0.0, // bottom-right
    0.5, 0.5, 0.5, 1.0, 0.0, // bottom-right
    -0.5, 0.5, 0.5, 0.0, 0.0, // bottom-left
    -0.5, 0.5, -0.5, 0.0, 1.0, // top-left
];

pub static SKYBOX_INDICES: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];
