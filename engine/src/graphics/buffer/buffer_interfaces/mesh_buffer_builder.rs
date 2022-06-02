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

#[derive(Eq, PartialEq)]
pub enum ShadingStrategy {
    PerVertex,
    PerFace,
}

pub trait MeshBuildStep {}

pub struct NewBuilderStep;
impl MeshBuildStep for NewBuilderStep {}

pub struct AddingVerticesStep;
impl MeshBuildStep for AddingVerticesStep {}

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
}

impl HasPosition for Vertex {
    fn position(&self) -> &Vec3F {
        &self.position
    }
}

pub struct MeshBufferBuilder<T: MeshBuildStep> {
    vertices: Vec<Vertex>,
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

    pub fn vertices(&mut self) -> &mut Vec<Vertex> {
        &mut self.vertices
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
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

    pub fn next(self) -> MeshBufferBuilder<AddingVerticesStep> {
        self.consume::<AddingVerticesStep>()
    }
}

pub type MeshBuilder = MeshBufferBuilder<NewBuilderStep>;

impl MeshBufferBuilder<AddingVerticesStep> {
    pub fn push_vertex(&mut self, x: f32, y: f32, z: f32) {
        let vertex = Vertex {
            position: Vec3F::new(x, y, z),
            uv: Vec2F::zero(),
            // The rest will be filled programatically
            normal: Vec3F::zero(),
            tangent: Vec3F::zero(),
            bitangent: Vec3F::zero(),
        };
        self.vertices.push(vertex);
    }

    pub fn push_vertex_flat(&mut self, x: f32, y: f32, z: f32, u: f32, v: f32) {
        let vertex = Vertex {
            position: Vec3F::new(x, y, z),
            uv: Vec2F::new(u, v),
            // The rest will be filled programatically
            normal: Vec3F::zero(),
            tangent: Vec3F::zero(),
            bitangent: Vec3F::zero(),
        };
        self.vertices.push(vertex);
    }

    pub fn hydrate_mock(self) -> MeshBufferBuilder<HydratedBuilderStep> {
        let mut builder = self.consume::<HydratedBuilderStep>();
        for i in 0..builder.vertices.len() {
            builder.index_buffer.push(i as u32);
        }
        builder
    }

    pub fn hydrate(self) -> MeshBufferBuilder<HydratedBuilderStep> {
        let mut builder = self.consume::<HydratedBuilderStep>();
        for i in 0..builder.vertices.len() {
            builder.index_buffer.push(i as u32);
        }
        for e in 0..builder.vertices.len() / 3 {
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
                let f = 1.0f32 / (duv1.x * duv2.y - duv2.x * duv1.y);
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
            builder.vertices[i].normal = normal_vec;
            builder.vertices[i].tangent = tangent_vec;
            builder.vertices[i].bitangent = bitangent_vec;
            builder.vertices[i + 1].normal = normal_vec;
            builder.vertices[i + 1].tangent = tangent_vec;
            builder.vertices[i + 1].bitangent = bitangent_vec;
            builder.vertices[i + 2].normal = normal_vec;
            builder.vertices[i + 2].tangent = tangent_vec;
            builder.vertices[i + 2].bitangent = bitangent_vec;
        }
        if &builder.shading_strategy == &ShadingStrategy::PerVertex {
            let kd_tree = KdTree::new(builder.vertices.clone(), 8);
            for i in 0..builder.vertices.len() {
                let faces = kd_tree.query_near(&builder.vertices[i].position(), 0.0001);
                builder.vertices[i].normal = faces
                    .iter()
                    .fold(Vec3F::zero(), |acc, &face_i| acc + kd_tree.data()[face_i].normal)
                    / (faces.len() as f32);
                builder.vertices[i].tangent = faces
                    .iter()
                    .fold(Vec3F::zero(), |acc, &face_i| acc + kd_tree.data()[face_i].tangent)
                    / (faces.len() as f32);
                builder.vertices[i].bitangent = faces
                    .iter()
                    .fold(Vec3F::zero(), |acc, &face_i| acc + kd_tree.data()[face_i].bitangent)
                    / (faces.len() as f32);
            }
        }
        builder
    }
}

impl Into<VertexArrayBuilder> for MeshBufferBuilder<HydratedBuilderStep> {
    fn into(self) -> VertexArrayBuilder {
        let vertex_data: Vec<f32> = self
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
