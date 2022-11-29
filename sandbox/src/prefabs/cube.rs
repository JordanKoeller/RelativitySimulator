use cgmath::prelude::*;
use specs::prelude::*;

use engine::ecs::{PrefabBuilder, SystemUtilities};
use engine::graphics::{
  Assets, ColorSpace, HydratedBuilderStep, MaterialComponent, MeshBufferBuilder, MeshBuilder, MeshComponent,
  ShadingStrategy, TextureBuilder, VertexArrayBuilder,
};
use engine::physics::{RigidBody, TransformComponent};
use engine::utils::{QuatF, Vec3F};

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

  fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) -> Entity {
    let mesh_builder = self.build_cube_mesh();
    let mesh_builder: VertexArrayBuilder = mesh_builder.into();
    let vai = api.get_else("cube", mesh_builder);
    let mesh = MeshComponent::new(vai, api.get_shader("default_texture").unwrap());
    let mut material = MaterialComponent::default();
    material.diffuse_texture(
      api.get_else(
        &state.texture_file,
        TextureBuilder::default()
          .with_color_space(ColorSpace::SRGB)
          .with_file(&state.texture_file),
      ),
    );
    material.specular_texture(api.get_else(
      &state.texture_file,
      TextureBuilder::default().with_file(&state.texture_file),
    ));
    material.normal_texture(api.get_else(
      &state.normal_file,
      TextureBuilder::default().with_file(&state.normal_file),
    ));
    let mut transform = TransformComponent::identity();
    transform.push_scale(Vec3F::new(state.scale, state.scale, state.scale));
    transform.push_translation(state.origin);
    let mut rigid_body = RigidBody::new_stationary();
    let rotation = QuatF::from_angle_y(cgmath::Deg(0.5f32)) * QuatF::from_angle_x(cgmath::Deg(0.5f32));
    rigid_body.angular_velocity = rotation;
    api
      .entity_builder()
      .and(|ett| ett.with(material).with(transform).with(mesh).with(rigid_body))
      .consume()
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

    builder.next()
  }
}

pub static CUBE_VERTICES: [f32; 180] = [
  // positions          // normals           // texture coords
  //FRONT
  -0.5, -0.5, -0.5, 0.0, 0.0, // Bottom-left
  0.5, 0.5, -0.5, 1.0, 1.0, // top-right
  0.5, -0.5, -0.5, 1.0, 0.0, // bottom-right
  0.5, 0.5, -0.5, 1.0, 1.0, // top-right
  -0.5, -0.5, -0.5, 0.0, 0.0, // bottom-left
  -0.5, 0.5, -0.5, 0.0, 1.0, // top-left
  // Front face
  -0.5, -0.5, 0.5, 0.0, 0.0, // bottom-left
  0.5, -0.5, 0.5, 1.0, 0.0, // bottom-right
  0.5, 0.5, 0.5, 1.0, 1.0, // top-right
  0.5, 0.5, 0.5, 1.0, 1.0, // top-right
  -0.5, 0.5, 0.5, 0.0, 1.0, // top-left
  -0.5, -0.5, 0.5, 0.0, 0.0, // bottom-left
  // Left face
  -0.5, 0.5, 0.5, 1.0, 0.0, // top-right
  -0.5, 0.5, -0.5, 1.0, 1.0, // top-left
  -0.5, -0.5, -0.5, 0.0, 1.0, // bottom-left
  -0.5, -0.5, -0.5, 0.0, 1.0, // bottom-left
  -0.5, -0.5, 0.5, 0.0, 0.0, // bottom-right
  -0.5, 0.5, 0.5, 1.0, 0.0, // top-right
  // Right face
  0.5, 0.5, 0.5, 1.0, 0.0, // top-left
  0.5, -0.5, -0.5, 0.0, 1.0, // bottom-right
  0.5, 0.5, -0.5, 1.0, 1.0, // top-right
  0.5, -0.5, -0.5, 0.0, 1.0, // bottom-right
  0.5, 0.5, 0.5, 1.0, 0.0, // top-left
  0.5, -0.5, 0.5, 0.0, 0.0, // bottom-left
  // Bottom face
  -0.5, -0.5, -0.5, 0.0, 1.0, // top-right
  0.5, -0.5, -0.5, 1.0, 1.0, // top-left
  0.5, -0.5, 0.5, 1.0, 0.0, // bottom-left
  0.5, -0.5, 0.5, 1.0, 0.0, // bottom-left
  -0.5, -0.5, 0.5, 0.0, 0.0, // bottom-right
  -0.5, -0.5, -0.5, 0.0, 1.0, // top-right
  // Top face
  -0.5, 0.5, -0.5, 0.0, 1.0, // top-left
  0.5, 0.5, 0.5, 1.0, 0.0, // bottom-right
  0.5, 0.5, -0.5, 1.0, 1.0, // top-right
  0.5, 0.5, 0.5, 1.0, 0.0, // bottom-right
  -0.5, 0.5, -0.5, 0.0, 1.0, // top-left
  -0.5, 0.5, 0.5, 0.0, 0.0, // bottom-left
];
