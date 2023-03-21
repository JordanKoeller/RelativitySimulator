use cgmath::prelude::*;
use specs::prelude::*;
use std::ops::Deref;

use crate::ecs::{ComponentCache, PrefabBuilder, SystemUtilities};
use crate::graphics::{
  Assets, AttributeType, BufferConfig, BufferLayout, ColorSpace, DataBufferBuilder, HydratedBuilderStep,
  IndexBufferBuilder, MaterialComponent, MeshBufferBuilder, MeshBuilder, MeshComponent, ShaderBuilder, ShadingStrategy,
  TextureBuilder, VertexArrayBuilder,
};
use crate::physics::TransformComponent;
use crate::physics::{Collision, CollisionSummary};
use crate::utils::{swizzle_down, swizzle_up, DegF, Mat3F, QuatF, Vec3F, Vec4F};
use specs::prelude::*;
use specs::{Component, VecStorage};

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CubeState {
  texture_filename: String,
  position: Vec3F,
  rotation: Vec3F,
  scale: Vec3F,
  faces: Vec<CubeFace>,
}

impl CubeState {
  pub fn new(filename: &str, position: Vec3F) -> Self {
    Self {
      texture_filename: filename.to_string(),
      position,
      rotation: Vec3F::zero(),
      scale: Vec3F::new(1f32, 1f32, 1f32),
      faces: CubeFace::all().into(),
    }
  }
}

#[derive(Default)]
pub struct Cube;

impl PrefabBuilder for Cube {
  type PrefabState = CubeState;

  fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) -> Entity {
    let mesh_builder = self.build_cube_mesh(&state);
    let mesh_builder: VertexArrayBuilder = mesh_builder.into();
    let vai = api.get_else("cube", mesh_builder);
    let mesh = MeshComponent::new(vai, api.get_shader("default_texture").unwrap());
    let mut material = MaterialComponent::default();
    material.diffuse_texture(
      api.get_else(
        &state.texture_filename,
        TextureBuilder::default()
          .with_color_space(ColorSpace::SRGB)
          .with_file(&state.texture_filename),
      ),
    );
    material.specular_texture(api.get_else(
      &state.texture_filename,
      TextureBuilder::default().with_file(&state.texture_filename),
    ));
    // material.normal_texture(api.get_else(
    //   &state.normal_file,
    //   TextureBuilder::default().with_file(&state.normal_file),
    // ));
    let mut transform = TransformComponent::identity();
    let rotation = QuatF::from_angle_x(cgmath::Deg(state.rotation.x))
      * QuatF::from_angle_y(cgmath::Deg(state.rotation.y))
      * QuatF::from_angle_z(cgmath::Deg(state.rotation.z));
    transform.push_scale(state.scale);
    transform.push_rotation(&rotation);
    transform.push_translation(state.position);

    api
      .entity_builder()
      .and(|ett| ett.with(material).with(transform).with(mesh))
      .consume()
  }
}

impl Cube {
  fn build_cube_mesh(&self, state: &CubeState) -> MeshBufferBuilder<HydratedBuilderStep> {
    let mut builder = MeshBuilder::default()
      .with_shading_strategy(ShadingStrategy::PerFace)
      .next();

    for face in state.faces.iter() {
      let coords = face.coords();
      for i in 0..6 {
        let ii = i * 5;
        builder.push_vertex_flat(
          coords[ii],
          coords[ii + 1],
          coords[ii + 2],
          coords[ii + 3],
          coords[ii + 4],
        );
      }
    }

    builder.next()
  }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum CubeFace {
  North,
  East,
  South,
  West,
  Top,
  Bottom,
}

impl CubeFace {
  pub fn all() -> [CubeFace; 6] {
    [
      Self::North,
      Self::East,
      Self::South,
      Self::West,
      Self::Top,
      Self::Bottom,
    ]
  }

  pub fn cardinal_directions() -> [CubeFace; 4] {
    [Self::North, Self::East, Self::South, Self::West]
  }

  pub fn coords(&self) -> &[f32] {
    match self {
      Self::North => &CUBE_VERTICES[0..30],
      Self::East => &CUBE_VERTICES[90..120],
      Self::South => &CUBE_VERTICES[30..60],
      Self::West => &CUBE_VERTICES[60..90],
      Self::Top => &CUBE_VERTICES[120..150],
      Self::Bottom => &CUBE_VERTICES[150..180],
    }
  }
}

pub static CUBE_VERTICES: [f32; 180] = [
  // positions          // normals           // texture coords
  //Back face
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
  // Top face
  -0.5, 0.5, -0.5, 0.0, 1.0, // top-left
  0.5, 0.5, 0.5, 1.0, 0.0, // bottom-right
  0.5, 0.5, -0.5, 1.0, 1.0, // top-right
  0.5, 0.5, 0.5, 1.0, 0.0, // bottom-right
  -0.5, 0.5, -0.5, 0.0, 1.0, // top-left
  -0.5, 0.5, 0.5, 0.0, 0.0, // bottom-left
  // Bottom face
  -0.5, -0.5, -0.5, 0.0, 1.0, // top-right
  0.5, -0.5, -0.5, 1.0, 1.0, // top-left
  0.5, -0.5, 0.5, 1.0, 0.0, // bottom-left
  0.5, -0.5, 0.5, 1.0, 0.0, // bottom-left
  -0.5, -0.5, 0.5, 0.0, 0.0, // bottom-right
  -0.5, -0.5, -0.5, 0.0, 1.0, // top-right
];
