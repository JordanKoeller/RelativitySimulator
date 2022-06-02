use cgmath::prelude::*;
use specs::prelude::*;

use crate::ecs::{PrefabBuilder, SystemUtilities};
use crate::graphics::{
    HydratedBuilderStep, MaterialComponent, MeshBufferBuilder, MeshBuilder, MeshComponent, ShadingStrategy,
    TextureBuilder,
};
use crate::physics::TransformComponent;
use crate::utils::{lerp, Color, Vec2F, Vec3F};

pub struct SphereState {
    radius: f64,
    origin: Vec3F,
    color: Color,
    texture_file: String,
    specular_file: String,
    normal_file: String,
    lod: u32,
}

impl SphereState {
    pub fn new(
        radius: f64,
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
pub struct Sphere;

impl PrefabBuilder for Sphere {
    type PrefabState = SphereState;

    fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) {
        let mesh_builder = self.build_from_polar(&state);
        let vai = api
            .assets()
            .get_or_create_vertex_array(&format!("sphere_{}", state.lod), mesh_builder.into());
        let mesh = MeshComponent::new(vai, api.assets().get_shader_id("default_texture").unwrap());
        let mut material = MaterialComponent::default();
        material.ambient_texture(api.assets().get_or_create_texture(
            "earth_texture",
            TextureBuilder::default().with_file(&state.texture_file),
        ));
        material.diffuse_texture(api.assets().get_or_create_texture(
            "earth_texture",
            TextureBuilder::default().with_file(&state.texture_file),
        ));
        material.specular_texture(api.assets().get_or_create_texture(
            "earth_specular",
            TextureBuilder::default().with_file(&state.specular_file),
        ));
        material.normal_texture(
            api.assets()
                .get_or_create_texture("earth_texture", TextureBuilder::default().with_file(&state.normal_file)),
        );
        let mut transform = TransformComponent::identity();
        transform.push_scale(Vec3F::new(state.radius, state.radius, state.radius));
        transform.push_translation(state.origin);
        api.entity_builder().with(material).with(transform).with(mesh).build();
    }
}

impl Sphere {
    fn get_unit_sphere_coords(&self, i: u32, j: u32, lod: u32) -> (Vec3F, Vec2F) {
        let theta = lerp(0f64, lod as f64, 0f64, std::f64::consts::PI * 2f64, i as f64);
        let psi = lerp(0f64, lod as f64, 0f64, std::f64::consts::PI, j as f64);
        let u = (j as f64) / (lod as f64);
        let v = (i as f64) / (lod as f64);
        (
            Vec3F::new(psi.sin() * theta.cos(), psi.sin() * theta.sin(), psi.cos()),
            Vec2F::new(v, u),
        )
    }

    fn build_from_polar(&self, state: &<Self as PrefabBuilder>::PrefabState) -> MeshBufferBuilder<HydratedBuilderStep> {
        let mut mesh_builder = MeshBuilder::default()
            .with_shading_strategy(ShadingStrategy::PerVertex)
            .next();
        for i in 0..state.lod {
            for j in 0..state.lod {
                let tl = self.get_unit_sphere_coords(i, j, state.lod);
                let tr = self.get_unit_sphere_coords(i + 1, j, state.lod);
                let bl = self.get_unit_sphere_coords(i, j + 1, state.lod);
                let br = self.get_unit_sphere_coords(i + 1, j + 1, state.lod);
                mesh_builder.push_vertex_flat(bl.0.x, bl.0.y, bl.0.z, bl.1.x, bl.1.y);
                mesh_builder.push_vertex_flat(tr.0.x, tr.0.y, tr.0.z, tr.1.x, tr.1.y);
                mesh_builder.push_vertex_flat(tl.0.x, tl.0.y, tl.0.z, tl.1.x, tl.1.y);
                mesh_builder.push_vertex_flat(bl.0.x, bl.0.y, bl.0.z, bl.1.x, bl.1.y);
                mesh_builder.push_vertex_flat(br.0.x, br.0.y, br.0.z, br.1.x, br.1.y);
                mesh_builder.push_vertex_flat(tr.0.x, tr.0.y, tr.0.z, tr.1.x, tr.1.y);
            }
        }
        mesh_builder.hydrate()
    }

    fn build_from_cube(&self, state: &<Self as PrefabBuilder>::PrefabState) -> MeshBufferBuilder<HydratedBuilderStep> {
        let mut mesh_builder = MeshBuilder::default()
            .with_shading_strategy(ShadingStrategy::PerVertex)
            .next();
        for i in 0..state.lod {
            for j in 0..state.lod {
                let ii = lerp(0 as f64, state.lod as f64, -0.5f64, 0.5f64, i as f64);
                let i1 = lerp(0 as f64, state.lod as f64, -0.5f64, 0.5f64, (i + 1) as f64);
                let jj = lerp(0 as f64, state.lod as f64, -0.5f64, 0.5f64, j as f64);
                let j1 = lerp(0 as f64, state.lod as f64, -0.5f64, 0.5f64, (j + 1) as f64);
                // front
                mesh_builder.push_vertex(ii, jj, -0.5f64);
                mesh_builder.push_vertex(i1, j1, -0.5f64);
                mesh_builder.push_vertex(ii, j1, -0.5f64);

                mesh_builder.push_vertex(ii, jj, -0.5f64);
                mesh_builder.push_vertex(i1, jj, -0.5f64);
                mesh_builder.push_vertex(i1, j1, -0.5f64);
                // // back
                mesh_builder.push_vertex(ii, jj, 0.5f64);
                mesh_builder.push_vertex(ii, j1, 0.5f64);
                mesh_builder.push_vertex(i1, j1, 0.5f64);

                mesh_builder.push_vertex(ii, jj, 0.5f64);
                mesh_builder.push_vertex(i1, j1, 0.5f64);
                mesh_builder.push_vertex(i1, jj, 0.5f64);
                // //top
                mesh_builder.push_vertex(ii, 0.5f64, jj);
                mesh_builder.push_vertex(i1, 0.5f64, jj);
                mesh_builder.push_vertex(i1, 0.5f64, j1);

                mesh_builder.push_vertex(ii, 0.5f64, jj);
                mesh_builder.push_vertex(i1, 0.5f64, j1);
                mesh_builder.push_vertex(ii, 0.5f64, j1);
                // //bottom
                mesh_builder.push_vertex(ii, -0.5f64, jj);
                mesh_builder.push_vertex(i1, -0.5f64, j1);
                mesh_builder.push_vertex(i1, -0.5f64, jj);

                mesh_builder.push_vertex(ii, -0.5f64, jj);
                mesh_builder.push_vertex(i1, -0.5f64, j1);
                mesh_builder.push_vertex(ii, -0.5f64, j1);
                // //left
                mesh_builder.push_vertex(-0.5f64, ii, jj);
                mesh_builder.push_vertex(-0.5f64, i1, jj);
                mesh_builder.push_vertex(-0.5f64, i1, j1);

                mesh_builder.push_vertex(-0.5f64, ii, jj);
                mesh_builder.push_vertex(-0.5f64, i1, j1);
                mesh_builder.push_vertex(-0.5f64, ii, j1);
                // //right
                mesh_builder.push_vertex(0.5f64, ii, jj);
                mesh_builder.push_vertex(0.5f64, i1, j1);
                mesh_builder.push_vertex(0.5f64, i1, jj);

                mesh_builder.push_vertex(0.5f64, ii, jj);
                mesh_builder.push_vertex(0.5f64, ii, j1);
                mesh_builder.push_vertex(0.5f64, i1, j1);
            }
        }

        let mut maxes = Vec2F::new(0.5f64, 0.5f64);
        let mut mins = Vec2F::new(0.5f64, 0.5f64);
        for i in 0..mesh_builder.num_vertices() {
            let mut vert = mesh_builder.vertices()[i].clone();
            let direction_vector = vert.position.normalize();
            let theta = (-direction_vector.y).acos() / std::f64::consts::PI;
            let phi = direction_vector.z.atan2(direction_vector.x) / std::f64::consts::PI / 2f64;
            vert.uv = Vec2F::new(1f64 - phi, theta);
            maxes.x = vert.uv.x.max(maxes.x);
            mins.x = vert.uv.x.min(mins.y);
            maxes.y = vert.uv.y.max(maxes.y);
            mins.y = vert.uv.y.min(mins.y);
            vert.position = direction_vector;
            mesh_builder.vertices()[i] = vert;
        }
        let mesh_builder = mesh_builder.hydrate();
        println!("Min = {:?} Max = {:?}", mins, maxes);
        mesh_builder
        // mesh_builder.hydrate_mock()
    }
}
