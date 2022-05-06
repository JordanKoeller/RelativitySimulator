use specs::prelude::*;
use cgmath::prelude::*;

use crate::graphics::{MeshBuilder, ShadingStrategy, MaterialComponent, MeshComponent, MeshBufferBuilder, HydratedBuilderStep};
use crate::ecs::{PrefabBuilder, SystemUtilities};
use crate::physics::TransformComponent;
use crate::utils::{Vec3F, Vec2F, Color, lerp};

pub struct SphereState {
    radius: f32,
    origin: Vec3F,
    color: Color,
    lod: u32,
}

impl SphereState {
    pub fn new(radius: f32, origin: Vec3F, color: Color, lod: u32) -> Self {
        Self {
            radius,
            origin,
            color,
            lod
        }
    }
}


#[derive(Default)]
pub struct Sphere;

impl PrefabBuilder for Sphere {
    type PrefabState = SphereState;

    fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) {
        let mesh_builder = self.build_from_cube(&state);
        let vai = api.assets().get_or_create_vertex_array(&format!("sphere_{}", state.lod), mesh_builder.into());
        let mesh = MeshComponent::new(vai, api.assets().get_shader_id("default_texture").unwrap());
        let mut material = MaterialComponent::default();
        material.ambient(state.color.clone());
        material.specular(state.color.clone());
        material.diffuse(state.color.clone());
        let mut transform = TransformComponent::identity();
        transform.push_scale(Vec3F::new(state.radius, state.radius, state.radius));
        transform.push_translation(state.origin);
        api.entity_builder().with(material).with(transform).with(mesh).build();
    }


}

impl Sphere {
    fn get_unit_sphere_coords(&self, i: u32, j: u32, lod: u32) -> Vec3F {
        let theta = lerp(0f32, lod as f32, 0f32, std::f32::consts::PI * 2f32, i as f32);
        let psi = lerp(0f32, lod as f32, 0f32, std::f32::consts::PI, j as f32);
        Vec3F::new(
            psi.sin()*theta.cos(),
            psi.sin()*theta.sin(),
            psi.cos()
        )
    }

    fn build_from_polar(&self, state: &<Self as PrefabBuilder>::PrefabState) -> MeshBufferBuilder<HydratedBuilderStep> {
        let mut mesh_builder = MeshBuilder::default().with_shading_strategy(ShadingStrategy::PerVertex).next();
        for i in 0..state.lod {
            for j in 0..state.lod {
                let tl_coord = self.get_unit_sphere_coords(i, j, state.lod);
                let tr_coord = self.get_unit_sphere_coords(i + 1, j, state.lod);
                let bl_coord = self.get_unit_sphere_coords(i, j + 1, state.lod);
                let br_coord = self.get_unit_sphere_coords(i + 1, j + 1, state.lod);
                mesh_builder.push_vertex(bl_coord, Vec2F::zero());
                mesh_builder.push_vertex(tr_coord, Vec2F::zero());
                mesh_builder.push_vertex(tl_coord, Vec2F::zero());
                mesh_builder.push_vertex(bl_coord, Vec2F::zero());
                mesh_builder.push_vertex(br_coord, Vec2F::zero());
                mesh_builder.push_vertex(tr_coord, Vec2F::zero());
            }
        }
        mesh_builder.hydrate()
    }

    fn build_from_cube(&self, state: &<Self as PrefabBuilder>::PrefabState) -> MeshBufferBuilder<HydratedBuilderStep> {
        let mut mesh_builder = MeshBuilder::default().with_shading_strategy(ShadingStrategy::PerFace).next();
        for i in 0..state.lod {
            for j in 0..state.lod {
                let ii = lerp(0 as f32, state.lod as f32, -0.5f32, 0.5f32, i as f32);
                let i1 = lerp(0 as f32, state.lod as f32, -0.5f32, 0.5f32, (i + 1) as f32);
                let jj = lerp(0 as f32, state.lod as f32, -0.5f32, 0.5f32, j as f32);
                let j1 = lerp(0 as f32, state.lod as f32, -0.5f32, 0.5f32, (j + 1) as f32);
                // front
                mesh_builder.push_vertex(Vec3F::new(ii, jj, -0.5f32), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, j1, -0.5f32), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(ii, j1, -0.5f32), Vec2F::zero());

                mesh_builder.push_vertex(Vec3F::new(ii, jj, -0.5f32), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, jj, -0.5f32), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, j1, -0.5f32), Vec2F::zero());
                // // back
                mesh_builder.push_vertex(Vec3F::new(ii, jj, 0.5f32), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(ii, j1, 0.5f32), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, j1, 0.5f32), Vec2F::zero());

                mesh_builder.push_vertex(Vec3F::new(ii, jj, 0.5f32), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, j1, 0.5f32), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, jj, 0.5f32), Vec2F::zero());
                // //top
                mesh_builder.push_vertex(Vec3F::new(ii, 0.5f32, jj), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, 0.5f32, jj), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, 0.5f32, j1), Vec2F::zero());

                mesh_builder.push_vertex(Vec3F::new(ii, 0.5f32, jj), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, 0.5f32, j1), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(ii, 0.5f32, j1), Vec2F::zero());
                // //bottom
                mesh_builder.push_vertex(Vec3F::new(ii, -0.5f32, jj), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, -0.5f32, j1), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, -0.5f32, jj), Vec2F::zero());

                mesh_builder.push_vertex(Vec3F::new(ii, -0.5f32, jj), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(i1, -0.5f32, j1), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(ii, -0.5f32, j1), Vec2F::zero());
                // //left
                mesh_builder.push_vertex(Vec3F::new(-0.5f32, ii, jj), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(-0.5f32, i1, jj), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(-0.5f32, i1, j1), Vec2F::zero());

                mesh_builder.push_vertex(Vec3F::new(-0.5f32, ii, jj), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(-0.5f32, i1, j1), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(-0.5f32, ii, j1), Vec2F::zero());
                // //right
                mesh_builder.push_vertex(Vec3F::new(0.5f32, ii, jj), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(0.5f32, i1, j1), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(0.5f32, i1, jj), Vec2F::zero());

                mesh_builder.push_vertex(Vec3F::new(0.5f32, ii, jj), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(0.5f32, ii, j1), Vec2F::zero());
                mesh_builder.push_vertex(Vec3F::new(0.5f32, i1, j1), Vec2F::zero());
            }
        }

        let mut mesh_builder = mesh_builder.hydrate();
        for i in 0..mesh_builder.num_vertices() {
            let mut vert = mesh_builder.vertices()[i].clone();
            let direction_vector = vert.position.normalize();
            vert.position = direction_vector;
            vert.normal = direction_vector;
            mesh_builder.vertices()[i] = vert;
        }
        mesh_builder
    }
}