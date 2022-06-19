use specs::hibitset::BitSet;
use specs::prelude::*;

use crate::ecs::{PrefabBuilder, SystemUtilities};
use crate::graphics::{
    HydratedBuilderStep, MaterialComponent, MeshBufferBuilder, MeshBuilder, MeshComponent, ShadingStrategy,
    TextureBuilder, TextureId, VertexArrayBuilder,
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
            shading_strategy: ShadingStrategy::PerFace,
        }
    }
}

#[derive(Default)]
pub struct ModelBuilder;

impl PrefabBuilder for ModelBuilder {
    type PrefabState = ModelLoader;

    fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) {
        let (meshes, mtl_results) = tobj::load_obj(&state.path, &tobj::GPU_LOAD_OPTIONS)
            .expect(&format!("Could not load model {}", &state.path));
        let materials = mtl_results.expect(&format!("Could not load material  on model {}", state.path));
        let mut entity_set = BitSet::new();
        let mut mesh_index = 0usize;
        for t_mesh in meshes.iter() {
            mesh_index += 1;
            let (mesh_builder, mtl_id_opt) = self.build_mesh_component(&t_mesh.mesh, state.shading_strategy);
            let mut ett = api.entity_builder();
            let vai = api
                .assets()
                .get_or_create_vertex_array(&format!("{}_{}", state.path, mesh_index), mesh_builder);
            let mesh_component = MeshComponent::new(vai, api.assets().get_shader_id("default_texture").unwrap());
            ett = ett.with(mesh_component);
            if let Some(mtl_id) = mtl_id_opt {
                let material = self.build_material(&materials[mtl_id], &api, &state);
                ett = ett.with(material)
            }
            let mut transform = TransformComponent::identity();
            transform.push_translation(Vec3F::new(4f64, 4f64, 4f64));
            ett = ett.with(transform);
            let entity = ett.build();
            entity_set.add(entity.id());
        }
    }
}

impl ModelBuilder {
    fn build_mesh_component(
        &self,
        mesh: &tobj::Mesh,
        shading_strategy: ShadingStrategy,
    ) -> (VertexArrayBuilder, Option<usize>) {
        let mut mesh_builder = MeshBuilder::default().with_shading_strategy(shading_strategy).next();
        for t_index in mesh.indices.iter() {
            let i = (t_index * 3) as usize;
            let uv = (t_index * 2) as usize;
            let v_i = if !mesh.texcoords.is_empty() {
                mesh_builder.push_vertex_flat(
                    mesh.positions[i] as f64,
                    mesh.positions[i + 1] as f64,
                    mesh.positions[i + 2] as f64,
                    mesh.texcoords[uv] as f64,
                    1.0f64 - mesh.texcoords[uv + 1] as f64,
                )
            } else {
                mesh_builder.push_vertex(
                    mesh.positions[i] as f64,
                    mesh.positions[i + 1] as f64,
                    mesh.positions[i + 2] as f64,
                )
            };
            if !mesh.normals.is_empty() {
                mesh_builder.vertices[v_i].normal = Vec3F::new(
                    mesh.normals[i] as f64,
                    mesh.normals[i + 1] as f64,
                    mesh.normals[i + 2] as f64,
                );
            }
        }
        let hydrated_builder = mesh_builder.hydrate();
        (hydrated_builder.into(), mesh.material_id.clone())
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
        material.shininess(mtl.shininess as f64);
        material.dissolve(mtl.dissolve as f64);
        if mtl.normal_texture.len() > 0 {
            material.normal_texture(self.to_tex(&state.path, &mtl.normal_texture, &api));
        }
        if mtl.diffuse_texture.len() > 0 {
            material.diffuse_texture(self.to_tex(&state.path, &mtl.diffuse_texture, &api));
        }
        if mtl.specular_texture.len() > 0 {
            material.specular_texture(self.to_tex(&state.path, &mtl.specular_texture, &api));
        }
        material
    }

    fn to_vec(&self, v: &[f32; 3]) -> Vec3F {
        Vec3F::new(v[0] as f64, v[1] as f64, v[2] as f64)
    }

    fn to_tex<'a>(&self, resource_path: &str, subpath: &str, api: &SystemUtilities<'a>) -> TextureId {
        let path = std::path::Path::new(resource_path)
            .parent()
            .unwrap()
            .join(subpath)
            .to_str()
            .unwrap()
            .to_string();
        api.assets()
            .get_or_create_texture(&path, TextureBuilder::default().with_file(&path))
    }
}
