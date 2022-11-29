use cgmath::prelude::*;
use specs::prelude::*;

use crate::ecs::{ComponentCache, PrefabBuilder, SystemUtilities};
use crate::graphics::{
  AssetLibrary, Assets, ColorSpace, HydratedBuilderStep, MaterialComponent, MeshBufferBuilder, MeshBuilder,
  MeshComponent, ShadingStrategy, TextureBuilder, VertexArrayBuilder,
};
use crate::physics::{RigidBody, TransformComponent};
use crate::utils::{lerp, Color, QuatF, Vec2F, Vec3F};

pub struct SphereState {
  radius: f32,
  origin: Vec3F,
  color: Color,
  texture_file: String,
  specular_file: String,
  normal_file: String,
  lod: u32,
}

impl SphereState {
  pub fn new(
    radius: f32,
    origin: Vec3F,
    color: Color,
    texture_file: &str,
    specular_file: &str,
    normal_file: &str,
    lod: u32,
  ) -> Self {
    Self {
      radius,
      origin,
      color,
      texture_file: texture_file.to_string(),
      specular_file: specular_file.to_string(),
      normal_file: normal_file.to_string(),
      lod,
    }
  }
}

#[derive(Default)]
pub struct Sphere {
  cache: ComponentCache,
}

impl PrefabBuilder for Sphere {
  type PrefabState = SphereState;

  fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) -> Entity {
    let mesh = self.cache.get_or(|| {
      let vai = api.get_or_create(&format!("sphere_{}", state.lod), || {
        let vao: VertexArrayBuilder = Self::build_from_polar(&state).into();
        vao
      });
      MeshComponent::new(vai, api.get_shader("default_texture").unwrap())
    });
    let mut material = MaterialComponent::default();
    material.diffuse_texture(api.assets().get_or_create("earth_texture", || {
      TextureBuilder::default()
        .with_color_space(ColorSpace::SRGB)
        .with_file(&state.texture_file)
    }));
    material.specular_texture(api.assets().get_or_create("earth_specular", || {
      TextureBuilder::default().with_file(&state.specular_file)
    }));
    material.normal_texture(api.assets().get_or_create("earth_normal", || {
      TextureBuilder::default().with_file(&state.normal_file)
    }));
    let mut transform = TransformComponent::identity();
    transform.push_scale(Vec3F::new(state.radius, state.radius, state.radius));
    transform.push_translation(state.origin);
    let axis_tilt = QuatF::from_angle_z(-cgmath::Deg(23.5f32));
    transform.push_rotation(&QuatF::from_angle_x(cgmath::Deg(90f32)));
    transform.push_rotation(&axis_tilt);
    let axis = axis_tilt.rotate_vector(Vec3F::unit_y());
    let mut rigid_body = RigidBody::new_stationary();
    let rotation = QuatF::from_axis_angle(axis, cgmath::Deg(0.25f32));
    rigid_body.angular_velocity = rotation;
    api
      .entity_builder()
      .and(|ett| ett.with(material).with(transform).with(mesh).with(rigid_body))
      .consume()
  }
}

impl Sphere {
  fn get_unit_sphere_coords(i: u32, j: u32, lod: u32) -> (Vec3F, Vec2F) {
    let theta = lerp(0f32, lod as f32, 0f32, std::f32::consts::PI * 2f32, i as f32);
    let psi = lerp(0f32, lod as f32, 0f32, std::f32::consts::PI, j as f32);
    let u = (j as f32) / (lod as f32);
    let v = (i as f32) / (lod as f32);
    (
      Vec3F::new(psi.sin() * theta.cos(), psi.sin() * theta.sin(), psi.cos()) / 2f32,
      Vec2F::new(1.0 - v, u),
    )
  }

  fn build_from_polar(state: &<Self as PrefabBuilder>::PrefabState) -> MeshBufferBuilder<HydratedBuilderStep> {
    let mut mesh_builder = MeshBuilder::default()
      .with_shading_strategy(ShadingStrategy::Preset)
      .next();
    for i in 0..state.lod {
      for j in 0..state.lod {
        let tl = Self::get_unit_sphere_coords(i, j, state.lod);
        let tr = Self::get_unit_sphere_coords(i + 1, j, state.lod);
        let bl = Self::get_unit_sphere_coords(i, j + 1, state.lod);
        let br = Self::get_unit_sphere_coords(i + 1, j + 1, state.lod);
        mesh_builder.push_vertex_flat(bl.0.x, bl.0.y, bl.0.z, bl.1.x, bl.1.y);
        mesh_builder.push_vertex_flat(tr.0.x, tr.0.y, tr.0.z, tr.1.x, tr.1.y);
        mesh_builder.push_vertex_flat(tl.0.x, tl.0.y, tl.0.z, tl.1.x, tl.1.y);
        mesh_builder.push_vertex_flat(bl.0.x, bl.0.y, bl.0.z, bl.1.x, bl.1.y);
        mesh_builder.push_vertex_flat(br.0.x, br.0.y, br.0.z, br.1.x, br.1.y);
        mesh_builder.push_vertex_flat(tr.0.x, tr.0.y, tr.0.z, tr.1.x, tr.1.y);
      }
    }
    for i in 0..mesh_builder.vertices.len() {
      mesh_builder.vertices[i].normal = mesh_builder.vertices[i].position.normalize();
    }
    mesh_builder.next()
  }

  fn build_from_cube(state: &<Self as PrefabBuilder>::PrefabState) -> MeshBufferBuilder<HydratedBuilderStep> {
    let mut mesh_builder = MeshBuilder::default()
      .with_shading_strategy(ShadingStrategy::PerVertex)
      .next();
    for i in 0..state.lod {
      for j in 0..state.lod {
        let ii = lerp(0 as f32, state.lod as f32, -0.5f32, 0.5f32, i as f32);
        let i1 = lerp(0 as f32, state.lod as f32, -0.5f32, 0.5f32, (i + 1) as f32);
        let jj = lerp(0 as f32, state.lod as f32, -0.5f32, 0.5f32, j as f32);
        let j1 = lerp(0 as f32, state.lod as f32, -0.5f32, 0.5f32, (j + 1) as f32);
        // front
        mesh_builder.push_vertex(ii, jj, -0.5f32);
        mesh_builder.push_vertex(ii, j1, -0.5f32);
        mesh_builder.push_vertex(i1, j1, -0.5f32);

        mesh_builder.push_vertex(ii, jj, -0.5f32);
        mesh_builder.push_vertex(i1, j1, -0.5f32);
        mesh_builder.push_vertex(i1, jj, -0.5f32);
        // // back
        mesh_builder.push_vertex(ii, jj, 0.5f32);
        mesh_builder.push_vertex(i1, j1, 0.5f32);
        mesh_builder.push_vertex(ii, j1, 0.5f32);

        mesh_builder.push_vertex(ii, jj, 0.5f32);
        mesh_builder.push_vertex(i1, jj, 0.5f32);
        mesh_builder.push_vertex(i1, j1, 0.5f32);
        // //top
        mesh_builder.push_vertex(ii, 0.5f32, jj);
        mesh_builder.push_vertex(i1, 0.5f32, j1);
        mesh_builder.push_vertex(i1, 0.5f32, jj);

        mesh_builder.push_vertex(ii, 0.5f32, jj);
        mesh_builder.push_vertex(ii, 0.5f32, j1);
        mesh_builder.push_vertex(i1, 0.5f32, j1);
        // //bottom
        mesh_builder.push_vertex(ii, -0.5f32, jj);
        mesh_builder.push_vertex(i1, -0.5f32, jj);
        mesh_builder.push_vertex(i1, -0.5f32, j1);

        mesh_builder.push_vertex(ii, -0.5f32, jj);
        mesh_builder.push_vertex(ii, -0.5f32, j1);
        mesh_builder.push_vertex(i1, -0.5f32, j1);
        // //left
        mesh_builder.push_vertex(-0.5f32, ii, jj);
        mesh_builder.push_vertex(-0.5f32, i1, j1);
        mesh_builder.push_vertex(-0.5f32, i1, jj);

        mesh_builder.push_vertex(-0.5f32, ii, jj);
        mesh_builder.push_vertex(-0.5f32, ii, j1);
        mesh_builder.push_vertex(-0.5f32, i1, j1);
        // //right
        mesh_builder.push_vertex(0.5f32, ii, jj);
        mesh_builder.push_vertex(0.5f32, i1, jj);
        mesh_builder.push_vertex(0.5f32, i1, j1);

        mesh_builder.push_vertex(0.5f32, ii, jj);
        mesh_builder.push_vertex(0.5f32, i1, j1);
        mesh_builder.push_vertex(0.5f32, ii, j1);
      }
    }
    for i in 0..mesh_builder.vertices.len() {
      let mut vert = mesh_builder.vertices[i].clone();
      let direction_vector = vert.position.normalize();
      let theta = (-direction_vector.y).acos() / std::f32::consts::PI;
      let phi = direction_vector.z.atan2(direction_vector.x) / std::f32::consts::PI / 2f32;
      vert.uv = Vec2F::new(theta, phi);
      vert.position = direction_vector;
      mesh_builder.vertices[i] = vert;
    }
    mesh_builder.hydrate()
    // mesh_builder.hydrate_mock()
  }
}
