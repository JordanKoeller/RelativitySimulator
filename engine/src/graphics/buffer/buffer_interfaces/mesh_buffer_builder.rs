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

struct NewBuilderStep;
impl MeshBuildStep for NewBuilderStep {}

struct AddingVerticesStep;
impl MeshBuildStep for AddingVerticesStep {}

struct HydratedBuilderStep;
impl MeshBuildStep for HydratedBuilderStep {}

#[derive(Clone)]
pub struct Vertex {
    position: Vec3F,
    uv: Vec2F,
    // The rest will be filled programatically
    normal: Vec3F,
    tangent: Vec3F,
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

impl MeshBufferBuilder<AddingVerticesStep> {
    pub fn push_vertex(&mut self, position: Vec3F, uv: Vec2F) {
        let vertex = Vertex {
            position,
            uv,
            // The rest will be filled programatically
            normal: Vec3F::zero(),
            tangent: Vec3F::zero(),
        };
        self.vertices.push(vertex);
    }

    pub fn hydrate(self) -> MeshBufferBuilder<HydratedBuilderStep> {
        let mut builder = self.consume::<HydratedBuilderStep>();
        for i in 0..builder.vertices.len() {
            builder.index_buffer.push(i as u32);
        }
        for i in 0..builder.vertices.len() / 3 {
            // Triangles wind in a counter-clockwise order.
            let normal_vec = {
                let a = &builder.vertices[i];
                let b = &builder.vertices[i + 1];
                let c = &builder.vertices[i + 2];
                let a_to_b = b.position - a.position;
                let b_to_c = c.position - a.position;
                a_to_b.cross(b_to_c).normalize()
            };
            builder.vertices[i].normal = normal_vec;
            builder.vertices[i + 1].normal = normal_vec;
            builder.vertices[i + 2].normal = normal_vec;
        }
        if &builder.shading_strategy == &ShadingStrategy::PerVertex {
            let kd_tree = KdTree::new(builder.vertices.clone(), 8);
            for i in 0..builder.vertices.len() {
                let faces = kd_tree.query_near(&builder.vertices[i].position(), 0.0001);
                let normal_avg = faces
                    .iter()
                    .fold(Vec3F::zero(), |acc, &face_i| acc + kd_tree.data()[face_i].normal)
                    / (faces.len() as f32);
                builder.vertices[i].normal = normal_avg;
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
