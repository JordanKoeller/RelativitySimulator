use specs::hibitset::BitSet;
use specs::prelude::*;

use crate::ecs::{EntityManager, PrefabBuilder, SystemUtilities};
use crate::graphics::{
  Assets, ColorSpace, HydratedBuilderStep, MaterialComponent, MeshBufferBuilder, MeshBuilder, MeshComponent,
  ShadingStrategy, TextureBuilder, TextureId, VertexArrayBuilder,
};
use crate::physics::TransformComponent;
use crate::utils::{Vec2F, Vec3F};

pub struct ModelLoader {
  path: String,
  shading_strategy: ShadingStrategy,
}

impl ModelLoader {
  pub fn new(path: &str) -> Self {
    Self {
      path: path.to_string(),
      shading_strategy: ShadingStrategy::Preset,
    }
  }
}

#[derive(Default)]
pub struct ModelBuilder;

impl PrefabBuilder for ModelBuilder {
  type PrefabState = ModelLoader;

  fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) -> Entity {
    let (meshes, mtl_results) =
      tobj::load_obj(&state.path, &tobj::GPU_LOAD_OPTIONS).expect(&format!("Could not load model {}", &state.path));
    let materials = mtl_results.expect(&format!("Could not load material  on model {}", state.path));
    let mut mesh_index = 0usize;
    let mut builder = api.entity_builder();
    for t_mesh in meshes.iter() {
      mesh_index += 1;
      let mtl_id_opt = t_mesh.mesh.material_id;
      let ett = builder.spawn_child();
      let vai = api.get_or_create(&format!("{}_{}", state.path, mesh_index), || {
        self.build_mesh_component(&t_mesh.mesh, state.shading_strategy)
      });
      let mesh_component = MeshComponent::new(vai, api.get_shader("default_texture").unwrap());
      ett.with(mesh_component);
      if let Some(mtl_id) = mtl_id_opt {
        let material = self.build_material(&materials[mtl_id], &api, &state);
        ett.with(material);
      };
      let mut transform = TransformComponent::identity();
      transform.push_translation(Vec3F::new(4f32, 4f32, 4f32));
      ett.with(transform);
    }
    builder.consume()
  }
}

impl ModelBuilder {
  fn build_mesh_component(&self, mesh: &tobj::Mesh, shading_strategy: ShadingStrategy) -> VertexArrayBuilder {
    let mut mesh_builder = MeshBuilder::default()
      .with_shading_strategy(shading_strategy)
      .with_index_buffer(mesh.indices.clone());
    for &i in mesh.indices.iter() {
      let p_i = (i * 3) as usize;
      let uv_i = (i * 2) as usize;
      mesh_builder.set(i as usize).position = Vec3F::new(
        mesh.positions[p_i] as f32,
        mesh.positions[p_i + 1] as f32,
        mesh.positions[p_i + 2] as f32,
      );
      if !mesh.normals.is_empty() {
        mesh_builder.set(i as usize).normal = Vec3F::new(
          mesh.normals[p_i] as f32,
          mesh.normals[p_i + 1] as f32,
          mesh.normals[p_i + 2] as f32,
        );
      }
      if !mesh.texcoords.is_empty() {
        mesh_builder.set(i as usize).uv =
          Vec2F::new(mesh.texcoords[uv_i] as f32, 1.0f32 - mesh.texcoords[uv_i + 1] as f32);
      }
    }
    let hydrated_builder = mesh_builder.hydrate();
    hydrated_builder.into()
  }

  fn build_material<'a>(
    &self,
    mtl: &tobj::Material,
    api: &SystemUtilities<'a>,
    state: &ModelLoader,
  ) -> MaterialComponent {
    let mut material = MaterialComponent::default();
    // material.ambient(self.to_vec(&mtl.diffuse));
    // material.diffuse(self.to_vec(&mtl.diffuse));
    // material.specular(self.to_vec(&mtl.specular));
    material.shininess(mtl.shininess as f32);
    material.dissolve(mtl.dissolve as f32);
    if mtl.normal_texture.len() > 0 {
      material.normal_texture(self.to_tex(&state.path, &mtl.normal_texture, &api, ColorSpace::RGB));
    }
    if mtl.diffuse_texture.len() > 0 {
      material.diffuse_texture(self.to_tex(&state.path, &mtl.diffuse_texture, &api, ColorSpace::SRGB));
    }
    if mtl.specular_texture.len() > 0 {
      material.specular_texture(self.to_tex(&state.path, &mtl.specular_texture, &api, ColorSpace::RGB));
    }
    material
  }

  fn to_vec(&self, v: &[f32; 3]) -> Vec3F {
    Vec3F::new(v[0] as f32, v[1] as f32, v[2] as f32)
  }

  fn to_tex<'a>(
    &self,
    resource_path: &str,
    subpath: &str,
    api: &SystemUtilities<'a>,
    color_space: ColorSpace,
  ) -> TextureId {
    let path = std::path::Path::new(resource_path)
      .parent()
      .unwrap()
      .join(subpath)
      .to_str()
      .unwrap()
      .to_string();
    api.get_or_create(&path, || {
      TextureBuilder::default().with_color_space(color_space).with_file(&path)
    })
  }
}
