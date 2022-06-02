use cgmath::prelude::*;
use specs::prelude::*;

use engine::ecs::{PrefabBuilder, SystemUtilities};
use engine::graphics::{
    HydratedBuilderStep, MaterialComponent, MeshBufferBuilder, MeshBuilder, MeshComponent, ShadingStrategy,
    TextureBuilder,
};
use engine::physics::{RigidBody, TransformComponent};
use engine::utils::{lerp, Color, QuatF, Vec2F, Vec3F};

pub struct CubeState {
    scale: f32,
    origin: Vec3F,
    texture_file: String,
    normal_file: String,
}

impl CubeState {
    pub fn new(scale: f32, origin: Vec3F, texture_file: &str, normal_file: &str) -> Self {
        Self {
            scale,
            origin,
            texture_file: texture_file.to_string(),
            normal_file: normal_file.to_string(),
        }
    }
}

#[derive(Default)]
pub struct Cube;

impl PrefabBuilder for Cube {
    type PrefabState = CubeState;

    fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) {
        let mesh_builder = self.build_cube_mesh();
        let vai = api.assets().get_or_create_vertex_array("cube", mesh_builder.into());
        let mesh = MeshComponent::new(vai, api.assets().get_shader_id("default_texture").unwrap());
        let mut material = MaterialComponent::default();
        material.diffuse_texture(api.assets().get_or_create_texture(
            &state.texture_file,
            TextureBuilder::default().with_file(&state.texture_file),
        ));
        material.specular_texture(api.assets().get_or_create_texture(
            &state.texture_file,
            TextureBuilder::default().with_file(&state.texture_file),
        ));
        material.normal_texture(api.assets().get_or_create_texture(
            &state.normal_file,
            TextureBuilder::default().with_file(&state.normal_file),
        ));
        let mut transform = TransformComponent::identity();
        transform.push_scale(Vec3F::new(state.scale, state.scale, state.scale));
        transform.push_translation(state.origin);
        let mut rigid_body = RigidBody::new_stationary();
        let rotation = QuatF::from_angle_y(cgmath::Deg(0.5f32)) * QuatF::from_angle_x(cgmath::Deg(0.5f32));
        rigid_body.angular_velocity = rotation;
        api.entity_builder()
            .with(material)
            .with(transform)
            .with(mesh)
            .with(rigid_body)
            .build();
    }
}

impl Cube {
    fn build_cube_mesh(&self) -> MeshBufferBuilder<HydratedBuilderStep> {
        let mut builder = MeshBuilder::default()
            .with_shading_strategy(ShadingStrategy::PerFace)
            .next();

        for i in 0..36 {
            let ii = i * 5;
            builder.push_vertex_flat(
                CUBE_VERTICES[ii],
                CUBE_VERTICES[ii + 1],
                CUBE_VERTICES[ii + 2],
                CUBE_VERTICES[ii + 3],
                CUBE_VERTICES[ii + 4],
            );
        }

        builder.hydrate()
    }
}

pub static CUBE_VERTICES: [f32; 180] = [
    // positions          // normals           // texture coords
    //FRONT
    -0.5f32, -0.5f32, -0.5f32, 0.0f32, 0.0f32, 0.5f32, -0.5f32, -0.5f32, 1.0f32, 0.0f32, 0.5f32, 0.5f32, -0.5f32,
    1.0f32, 1.0f32, 0.5f32, 0.5f32, -0.5f32, 1.0f32, 1.0f32, -0.5f32, 0.5f32, -0.5f32, 0.0f32, 1.0f32, -0.5f32,
    -0.5f32, -0.5f32, 0.0f32, 0.0f32, //BACK
    -0.5f32, -0.5f32, 0.5f32, 0.0f32, 0.0f32, 0.5f32, 0.5f32, 0.5f32, 1.0f32, 1.0f32, 0.5f32, -0.5f32, 0.5f32, 1.0f32,
    0.0f32, 0.5f32, 0.5f32, 0.5f32, 1.0f32, 1.0f32, -0.5f32, -0.5f32, 0.5f32, 0.0f32, 0.0f32, -0.5f32, 0.5f32, 0.5f32,
    0.0f32, 1.0f32, //FRONT
    -0.5f32, 0.5f32, 0.5f32, 1.0f32, 0.0f32, -0.5f32, 0.5f32, -0.5f32, 1.0f32, 1.0f32, -0.5f32, -0.5f32, -0.5f32,
    0.0f32, 1.0f32, -0.5f32, -0.5f32, -0.5f32, 0.0f32, 1.0f32, -0.5f32, -0.5f32, 0.5f32, 0.0f32, 0.0f32, -0.5f32,
    0.5f32, 0.5f32, 1.0f32, 0.0f32, //BACK
    0.5f32, 0.5f32, 0.5f32, 1.0f32, 0.0f32, 0.5f32, -0.5f32, -0.5f32, 0.0f32, 1.0f32, 0.5f32, 0.5f32, -0.5f32, 1.0f32,
    1.0f32, 0.5f32, -0.5f32, -0.5f32, 0.0f32, 1.0f32, 0.5f32, 0.5f32, 0.5f32, 1.0f32, 0.0f32, 0.5f32, -0.5f32, 0.5f32,
    0.0f32, 0.0f32, //FRONT
    -0.5f32, -0.5f32, -0.5f32, 0.0f32, 1.0f32, 0.5f32, -0.5f32, -0.5f32, 1.0f32, 1.0f32, 0.5f32, -0.5f32, 0.5f32,
    1.0f32, 0.0f32, 0.5f32, -0.5f32, 0.5f32, 1.0f32, 0.0f32, -0.5f32, -0.5f32, 0.5f32, 0.0f32, 0.0f32, -0.5f32,
    -0.5f32, -0.5f32, 0.0f32, 1.0f32, //BACK
    -0.5f32, 0.5f32, -0.5f32, 0.0f32, 1.0f32, 0.5f32, 0.5f32, 0.5f32, 1.0f32, 0.0f32, 0.5f32, 0.5f32, -0.5f32, 1.0f32,
    1.0f32, 0.5f32, 0.5f32, 0.5f32, 1.0f32, 0.0f32, -0.5f32, 0.5f32, -0.5f32, 0.0f32, 1.0f32, -0.5f32, 0.5f32, 0.5f32,
    0.0f32, 0.0f32,
];
