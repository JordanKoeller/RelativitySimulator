use cgmath::prelude::*;

use super::super::{
    AttributeType, BufferConfig, BufferLayout, BufferStorageLevel, BufferType, DataBufferBuilder, IndexBufferBuilder,
    VertexArrayBuilder,
};
use crate::datastructures::{HasPosition, KdTree, SpatialIndex};
use crate::utils::{Vec2F, Vec3F};

pub enum MeshPrimative {
    POINT,
    LINE,
    TRIANGLE,
    QUAD,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ShadingStrategy {
    PerVertex,
    PerFace,
    Preset, // Indicates that the user set the normals, so they don't need to be computed.
}

pub trait MeshBuildStep {}

pub struct NewBuilderStep;
impl MeshBuildStep for NewBuilderStep {}

pub struct AddingVerticesStep;
impl MeshBuildStep for AddingVerticesStep {}

pub struct SettingVerticesStep;
impl MeshBuildStep for SettingVerticesStep {}

pub struct HydratedBuilderStep;
impl MeshBuildStep for HydratedBuilderStep {}

#[derive(Clone)]
pub struct Vertex {
    pub position: Vec3F,
    pub uv: Vec2F,
    // The rest will be filled programatically
    pub normal: Vec3F,
    pub tangent: Vec3F,
    pub bitangent: Vec3F,
    base_index: Option<usize>,
}

impl HasPosition for Vertex {
    fn position(&self) -> &Vec3F {
        &self.position
    }
}

impl Vertex {
    fn with_base_index(&self, index: usize) -> Self {
        Self {
            base_index: Some(index),
            ..self.clone()
        }
    }
}

pub struct MeshBufferBuilder<T: MeshBuildStep> {
    pub vertices: Vec<Vertex>,
    primative_type: MeshPrimative,
    storage_type: BufferStorageLevel,
    shading_strategy: ShadingStrategy,
    // The rest is filled programatically
    index_buffer: Vec<u32>,
    layout: BufferLayout,
    attribute_divisor: u32,
    _marker: std::marker::PhantomData<T>,
}

impl Default for MeshBufferBuilder<NewBuilderStep> {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            primative_type: MeshPrimative::TRIANGLE,
            storage_type: BufferStorageLevel::STATIC,
            shading_strategy: ShadingStrategy::PerFace,
            // The rest are filled programatically
            index_buffer: Vec::new(),
            layout: BufferLayout::new(vec![
                AttributeType::Float3, // Position
                AttributeType::Float3, // Normal
                AttributeType::Float3, // Tangent
                AttributeType::Float3, // Bitangent
                AttributeType::Float2, // Texture Coordinates
            ]),
            attribute_divisor: 0,
            _marker: std::marker::PhantomData::<NewBuilderStep>::default(),
        }
    }
}



impl MeshBufferBuilder<NewBuilderStep> {
    pub fn with_storage_type(mut self, storage_type: BufferStorageLevel) -> Self {
        self.storage_type = storage_type;
        self
    }

    pub fn with_primative(mut self, primative_type: MeshPrimative) -> Self {
        self.primative_type = primative_type;
        self
    }

    pub fn with_shading_strategy(mut self, strategy: ShadingStrategy) -> Self {
        self.shading_strategy = strategy;
        self
    }

    pub fn with_index_buffer(mut self, indices: Vec<u32>) -> MeshBufferBuilder<SettingVerticesStep> {
        self.index_buffer = indices;
        self.consume::<SettingVerticesStep>()
    }

    pub fn next(self) -> MeshBufferBuilder<AddingVerticesStep> {
        self.consume::<AddingVerticesStep>()
    }
}

pub type MeshBuilder = MeshBufferBuilder<NewBuilderStep>;

impl MeshBufferBuilder<AddingVerticesStep> {
    pub fn push_vertex(&mut self, x: f64, y: f64, z: f64) -> usize {
        let vertex = Vertex {
            position: Vec3F::new(x, y, z),
            uv: Vec2F::zero(),
            // The rest will be filled programatically
            normal: Vec3F::zero(),
            tangent: Vec3F::zero(),
            bitangent: Vec3F::zero(),
            base_index: None,
        };
        let i = self.vertices.len();
        self.vertices.push(vertex);
        i
    }

    pub fn push_vertex_flat(&mut self, x: f64, y: f64, z: f64, u: f64, v: f64) -> usize {
        let vertex = Vertex {
            position: Vec3F::new(x, y, z),
            uv: Vec2F::new(u, v),
            // The rest will be filled programatically
            normal: Vec3F::zero(),
            tangent: Vec3F::zero(),
            bitangent: Vec3F::zero(),
            base_index: None,
        };
        let i = self.vertices.len();
        self.vertices.push(vertex);
        i
    }

    pub fn hydrate_mock(self) -> MeshBufferBuilder<HydratedBuilderStep> {
        let mut builder = self.consume::<HydratedBuilderStep>();
        for i in 0..builder.vertices.len() {
            builder.index_buffer.push(i as u32);
        }
        builder
    }

    pub fn next(self) -> MeshBufferBuilder<HydratedBuilderStep> {
        self.hydrate()
    }
}

impl MeshBufferBuilder<SettingVerticesStep> {

    pub fn set(&mut self, index: usize) -> &mut Vertex {
        &mut self.vertices[index]
    }

    pub fn get(&self, index: usize) -> &Vertex {
        &self.vertices[index]
    }
}

impl Into<VertexArrayBuilder> for MeshBufferBuilder<HydratedBuilderStep> {
    fn into(self) -> VertexArrayBuilder {
        let vertex_data: Vec<f64> = self
            .vertices
            .iter()
            .flat_map(|vertex| {
                vec![
                    vertex.position.x,
                    vertex.position.y,
                    vertex.position.z,
                    vertex.normal.x,
                    vertex.normal.y,
                    vertex.normal.z,
                    vertex.tangent.x,
                    vertex.tangent.y,
                    vertex.tangent.z,
                    vertex.bitangent.x,
                    vertex.bitangent.y,
                    vertex.bitangent.z,
                    vertex.uv.x,
                    vertex.uv.y,
                ]
            })
            .collect();
        let config = BufferConfig {
            storage_type: self.storage_type,
            buffer_type: BufferType::ARRAY,
            attrib_divisor: self.attribute_divisor,
        };
        VertexArrayBuilder::default()
            .with_index_buffer(IndexBufferBuilder::default().with_data(self.index_buffer))
            .with_vertex_buffer(
                DataBufferBuilder::default()
                    .with_layout(self.layout)
                    .with_data(vertex_data)
                    .with_config(config),
            )
    }
}

impl<T: MeshBuildStep> MeshBufferBuilder<T> {
    fn consume<N: MeshBuildStep>(self) -> MeshBufferBuilder<N> {
        MeshBufferBuilder::<N> {
            vertices: self.vertices,
            primative_type: self.primative_type,
            storage_type: self.storage_type,
            shading_strategy: self.shading_strategy,
            index_buffer: self.index_buffer,
            layout: self.layout,
            attribute_divisor: self.attribute_divisor,
            _marker: std::marker::PhantomData::<N>::default(),
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn hydrate(mut self) -> MeshBufferBuilder<HydratedBuilderStep> {
        if self.index_buffer.is_empty() {
            self.index_buffer = (0..self.num_vertices() as u32).collect()
        }
        // Compute per-face
        // TODO: If the index-buffer is not built from 2 lines above,
        // I need to expand out to a flat array to get accurate per-face values.
        // Otherwise I'll only have the NTB of the last face the vertex is involved in.
        for i in 0..self.index_buffer.len() / 3 {
            let ii = i * 3;
            let (normal, tangent, bitangent) = self.compute_face_basis(ii);
            for vert_i in ii..ii+3 {
                if self.shading_strategy != ShadingStrategy::Preset {
                    self.vertices[vert_i].normal = normal;
                }
                // TODO: Use gram-schmidt to renormalize if the normal is Preset, since
                // the tangent/bitangent probably are not orthogonal to the normal that was
                // manually set. I'll need to think of how to do this intelligently.
                self.vertices[vert_i].tangent = tangent;
                self.vertices[vert_i].bitangent = bitangent;
            }
        }
        if self.shading_strategy == ShadingStrategy::PerVertex {
            let flattened_vertices: Vec<Vertex> = self.index_buffer.iter().map(|&i| self.vertices[i as usize].with_base_index(i as usize)).collect();
            let kd_tree = KdTree::new(flattened_vertices, 8);
            for i in 0..self.index_buffer.len() {
                let faces = kd_tree.query_near(&self.vertices[i].position, 0.0001);
                self.vertices[i].normal = (faces
                    .iter()
                    .fold(Vec3F::zero(), |acc, &face_i| acc + kd_tree.data()[face_i].normal)
                    / (faces.len() as f64))
                    .normalize();
                    self.vertices[i].tangent = (faces
                    .iter()
                    .fold(Vec3F::zero(), |acc, &face_i| acc + kd_tree.data()[face_i].tangent)
                    / (faces.len() as f64))
                    .normalize();
                    self.vertices[i].bitangent = (faces
                    .iter()
                    .fold(Vec3F::zero(), |acc, &face_i| acc + kd_tree.data()[face_i].bitangent)
                    / (faces.len() as f64))
                    .normalize();
            }
        }
        self.consume()
    }


    fn compute_face_basis(&self, i: usize) -> (Vec3F, Vec3F, Vec3F) {
        let a = &self.vertices[i];
        let b = &self.vertices[i + 1];
        let c = &self.vertices[i + 2];
        let edge1 = b.position - a.position;
        let edge2 = c.position - a.position;
        let duv1 = b.uv - a.uv;
        let duv2 = c.uv - a.uv;
        let normal = edge1.cross(edge2).normalize();
        let f = 1.0f64 / (duv1.x * duv2.y - duv2.x * duv1.y);
        let tangent = Vec3F::new(
            f * (duv2.y * edge1.x - duv1.y * edge2.x),
            f * (duv2.y * edge1.y - duv1.y * edge2.y),
            f * (duv2.y * edge1.z - duv1.y * edge2.z),
        );
        let bitangent = Vec3F::new(
            f * (-duv2.x * edge1.x + duv1.x * edge2.x),
            f * (-duv2.x * edge1.y + duv1.x * edge2.y),
            f * (-duv2.x * edge1.z + duv1.x * edge2.z),
        );
        (normal, tangent, bitangent)
    }
}


/*
            let i = e * 3;
            // Triangles wind in a counter-clockwise order.
            let (normal_vec, tangent_vec, bitangent_vec) = {
                let a = &builder.vertices[i];
                let b = &builder.vertices[i + 1];
                let c = &builder.vertices[i + 2];
                let edge1 = b.position - a.position;
                let edge2 = c.position - a.position;
                let duv1 = b.uv - a.uv;
                let duv2 = c.uv - a.uv;
                let normal = edge1.cross(edge2).normalize();
                let f = 1.0f64 / (duv1.x * duv2.y - duv2.x * duv1.y);
                let tangent = Vec3F::new(
                    f * (duv2.y * edge1.x - duv1.y * edge2.x),
                    f * (duv2.y * edge1.y - duv1.y * edge2.y),
                    f * (duv2.y * edge1.z - duv1.y * edge2.z),
                );
                let bitangent = Vec3F::new(
                    f * (-duv2.x * edge1.x + duv1.x * edge2.x),
                    f * (-duv2.x * edge1.y + duv1.x * edge2.y),
                    f * (-duv2.x * edge1.z + duv1.x * edge2.z),
                );
                (normal, tangent, bitangent)
            };
            if builder.vertices[i].normal.magnitude2() < 1e-6f64 {
                builder.vertices[i].normal = normal_vec;
                builder.vertices[i + 1].normal = normal_vec;
                builder.vertices[i + 2].normal = normal_vec;
            }
            builder.vertices[i].tangent = tangent_vec;
            builder.vertices[i].bitangent = bitangent_vec;
            builder.vertices[i + 1].tangent = tangent_vec;
            builder.vertices[i + 1].bitangent = bitangent_vec;
            builder.vertices[i + 2].tangent = tangent_vec;
            builder.vertices[i + 2].bitangent = bitangent_vec;
*/
