use cgmath::prelude::*;

use super::super::{
    AttributeType, BufferConfig, BufferLayout, BufferStorageLevel, BufferType, Bufferable, DataBufferBuilder,
    IndexBufferBuilder, VertexArrayBuilder,
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
#[repr(C)]
pub struct Vertex {
    pub position: Vec3F,
    // The rest will be filled programatically
    pub normal: Vec3F,
    pub tangent: Vec3F,
    pub bitangent: Vec3F,
    pub uv: Vec2F,
}

impl HasPosition for Vertex {
    fn position(&self) -> &Vec3F {
        &self.position
    }
}

impl Bufferable for Vertex {
    fn into_buffer(&self, buffer: &mut [f32]) {
        buffer[0] = self.position.x;
        buffer[1] = self.position.y;
        buffer[2] = self.position.z;
        buffer[3] = self.normal.x;
        buffer[4] = self.normal.y;
        buffer[5] = self.normal.z;
        buffer[6] = self.tangent.x;
        buffer[7] = self.tangent.y;
        buffer[8] = self.tangent.z;
        buffer[9] = self.bitangent.x;
        buffer[10] = self.bitangent.y;
        buffer[11] = self.bitangent.z;
        buffer[12] = self.uv.x;
        buffer[13] = self.uv.y;
    }

    fn num_elems() -> usize {
        14usize
    }

    fn from_slice(b: &[f32]) -> Self {
        Self {
            position: Vec3F::new(b[0], b[1], b[2]),
            normal: Vec3F::new(b[3], b[4], b[5]),
            tangent: Vec3F::new(b[6], b[7], b[8]),
            bitangent: Vec3F::new(b[9], b[10], b[11]),
            uv: Vec2F::new(b[12], b[13]),
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Vec3F::zero(),
            normal: Vec3F::zero(),
            tangent: Vec3F::zero(),
            bitangent: Vec3F::zero(),
            uv: Vec2F::zero(),
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
        let max_ind_value = self.index_buffer.iter().fold(0u32, |acc, &i| i.max(acc));
        self.vertices = (0..max_ind_value + 1).map(|_| Vertex::default()).collect();
        self.consume::<SettingVerticesStep>()
    }

    pub fn next(self) -> MeshBufferBuilder<AddingVerticesStep> {
        self.consume::<AddingVerticesStep>()
    }
}

pub type MeshBuilder = MeshBufferBuilder<NewBuilderStep>;

impl MeshBufferBuilder<AddingVerticesStep> {
    pub fn push_vertex(&mut self, x: f32, y: f32, z: f32) -> usize {
        let vertex = Vertex {
            position: Vec3F::new(x as f32, y as f32, z as f32),
            uv: Vec2F::zero(),
            // The rest will be filled programatically
            normal: Vec3F::zero(),
            tangent: Vec3F::zero(),
            bitangent: Vec3F::zero(),
        };
        let i = self.vertices.len();
        self.vertices.push(vertex);
        i
    }

    pub fn push_vertex_flat(&mut self, x: f32, y: f32, z: f32, u: f32, v: f32) -> usize {
        let vertex = Vertex {
            position: Vec3F::new(x as f32, y as f32, z as f32),
            uv: Vec2F::new(u as f32, v as f32),
            // The rest will be filled programatically
            normal: Vec3F::zero(),
            tangent: Vec3F::zero(),
            bitangent: Vec3F::zero(),
        };
        let i = self.vertices.len();
        self.vertices.push(vertex);
        i
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
        self.index_buffer.len()
    }

    pub fn hydrate(mut self) -> MeshBufferBuilder<HydratedBuilderStep> {
        if self.index_buffer.is_empty() {
            self.index_buffer = (0..self.vertices.len() as u32).collect()
        }
        // Compute per-face
        // TODO: If the index-buffer is not built from 2 lines above,
        // I need to expand out to a flat array to get accurate per-face values.
        // Otherwise I'll only have the NTB of the last face the vertex is involved in.
        for i in 0..self.index_buffer.len() / 3 {
            let ii = i as usize * 3;
            let a_i = self.index_buffer[ii] as usize;
            let b_i = self.index_buffer[ii + 1] as usize;
            let c_i = self.index_buffer[ii + 2] as usize;
            let (normal, tangent, bitangent) = self.compute_face_basis(a_i, b_i, c_i);
            for &vert_i in &[a_i, b_i, c_i] {
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
            let flattened_vertices: Vec<Vertex> = self
                .index_buffer
                .iter()
                .map(|&i| self.vertices[i as usize].clone())
                .collect();
            let kd_tree = KdTree::new(flattened_vertices, 8);
            for i in 0..self.index_buffer.len() {
                let v_i = self.index_buffer[i] as usize;
                let faces = kd_tree.query_near(&self.vertices[v_i].position, 0.0001);
                self.vertices[v_i].normal = (faces
                    .iter()
                    .fold(Vec3F::zero(), |acc, &face_i| acc + kd_tree.data()[face_i].normal)
                    / (faces.len() as f32))
                    .normalize();
                self.vertices[v_i].tangent = (faces
                    .iter()
                    .fold(Vec3F::zero(), |acc, &face_i| acc + kd_tree.data()[face_i].tangent)
                    / (faces.len() as f32))
                    .normalize();
                self.vertices[v_i].bitangent = (faces
                    .iter()
                    .fold(Vec3F::zero(), |acc, &face_i| acc + kd_tree.data()[face_i].bitangent)
                    / (faces.len() as f32))
                    .normalize();
            }
        }
        self.consume()
    }

    fn compute_face_basis(&self, a_i: usize, b_i: usize, c_i: usize) -> (Vec3F, Vec3F, Vec3F) {
        let a = &self.vertices[a_i];
        let b = &self.vertices[b_i];
        let c = &self.vertices[c_i];
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
            if builder.vertices[i].normal.magnitude2() < 1e-6f32 {
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
