use specs::Builder;
use crate::renderer::{
    AttributeType, BufferLayout, CubeMap, DataBuffer, Drawable, IndexBuffer, TextureLike, Uniform, VertexArray,
};

use crate::ecs::{Material, PrefabBuilder, MeshComponent};
use crate::physics::TransformComponent;

pub struct SkyboxPrefab {
    texture_name: String,
}

impl SkyboxPrefab {
    pub fn new(name: &str) -> Self {
        Self {
            texture_name: name.to_string()
        }
    }
}

pub struct SkyboxBuilder;

impl PrefabBuilder for SkyboxBuilder {
    type PrefabState = SkyboxPrefab;

    fn build<B: Builder>(&self, entity_builder: B, state: Self::PrefabState) -> B {
        entity_builder
            .with(MeshComponent::new(self.vertex_array(), self.shader_name()))
            .with(self.material(&state))
            .with(TransformComponent::identity())
    }

}

impl SkyboxBuilder {
    fn vertex_array(&self) -> VertexArray {
        let layout = BufferLayout::new(vec![AttributeType::Float3]);
        let vert_buff = DataBuffer::static_buffer(&SKYBOX_VERTICES, layout);
        let ind_buff = IndexBuffer::create(SKYBOX_INDICES.to_vec());
        VertexArray::new(vert_buff, ind_buff)
    }

    fn material(&self, state: &<Self as PrefabBuilder>::PrefabState) -> Material {
        let mut material = Material::new();
        let cube_map = CubeMap::from_file(&state.texture_name);
        // cube_map.refresh();
        material.unknown_uniform("skybox", Uniform::CubeMap(cube_map));
        material
    }

    fn shader_name(&self) -> String {
        "skybox".to_string()
    }
}

static SKYBOX_VERTICES: [f32; 108] = [
    // positions       // normals        // texture coords
    -1.0f32, 1.0f32, -1.0f32, -1.0f32, -1.0f32, -1.0f32, 1.0f32, -1.0f32, -1.0f32, 1.0f32, -1.0f32, -1.0f32, 1.0f32,
    1.0f32, -1.0f32, -1.0f32, 1.0f32, -1.0f32, -1.0f32, -1.0f32, 1.0f32, -1.0f32, -1.0f32, -1.0f32, -1.0f32, 1.0f32,
    -1.0f32, -1.0f32, 1.0f32, -1.0f32, -1.0f32, 1.0f32, 1.0f32, -1.0f32, -1.0f32, 1.0f32, 1.0f32, -1.0f32, -1.0f32,
    1.0f32, -1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, -1.0f32, 1.0f32, -1.0f32,
    -1.0f32, -1.0f32, -1.0f32, 1.0f32, -1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32,
    -1.0f32, 1.0f32, -1.0f32, -1.0f32, 1.0f32, -1.0f32, 1.0f32, -1.0f32, 1.0f32, 1.0f32, -1.0f32, 1.0f32, 1.0f32,
    1.0f32, 1.0f32, 1.0f32, 1.0f32, -1.0f32, 1.0f32, 1.0f32, -1.0f32, 1.0f32, -1.0f32, -1.0f32, -1.0f32, -1.0f32,
    -1.0f32, -1.0f32, 1.0f32, 1.0f32, -1.0f32, -1.0f32, 1.0f32, -1.0f32, -1.0f32, -1.0f32, -1.0f32, 1.0f32, 1.0f32,
    -1.0f32, 1.0f32,
];

static SKYBOX_INDICES: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];
