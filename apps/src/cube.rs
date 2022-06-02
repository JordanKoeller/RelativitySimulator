use cgmath::prelude::InnerSpace;
use renderer::{AttributeType, BufferLayout, DataBuffer, IndexBuffer, VertexArray};
use renderer::{Drawable, Texture};
use specs::prelude::*;
use specs::{Component, VecStorage};
use std::ops::Deref;

use physics::{Collision, CollisionSummary};

use physics::TransformComponent;
use utils::{swizzle_down, swizzle_up, Mat3F, Vec3F, Vec4F};

use ecs::Material;

pub struct TexturedCube {
    filename: String,
}

impl Drawable for TexturedCube {
    fn vertex_array(&self) -> VertexArray {
        let layout = BufferLayout::new(vec![
            AttributeType::Float3,
            AttributeType::Float3,
            AttributeType::Float2,
        ]);
        let vert_buff = DataBuffer::static_buffer(&TEXTURE_CUBE_VERTICES, layout);
        let ind_buff = IndexBuffer::create(TEXTURE_CUBE_INDICES.to_vec());
        VertexArray::new(vert_buff, ind_buff)
    }
    fn material(&self) -> Material {
        let mut material = Material::new();
        material.diffuse_texture(Texture::from_file(&self.filename));
        material
    }

    fn shader_name(&self) -> String {
        "lorentz".to_string()
    }
}

pub struct Cube {
    material: Material,
}

impl Cube {
    pub fn new(material: Material) -> Cube {
        Cube { material }
    }
}

impl Drawable for Cube {
    fn vertex_array(&self) -> VertexArray {
        let layout = BufferLayout::new(vec![
            AttributeType::Float3,
            AttributeType::Float3,
            AttributeType::Float2,
        ]);
        let vert_buff = DataBuffer::static_buffer(&TEXTURE_CUBE_VERTICES, layout);
        let ind_buff = IndexBuffer::create(TEXTURE_CUBE_INDICES.to_vec());
        VertexArray::new(vert_buff, ind_buff)
    }
    fn material(&self) -> Material {
        self.material.clone()
    }

    fn shader_name(&self) -> String {
        "lorentz".to_string()
    }
}

pub struct FaceCube {
    pub c: Cube,
}

impl Drawable for FaceCube {
    fn vertex_array(&self) -> VertexArray {
        self.c.vertex_array()
    }
    fn material(&self) -> Material {
        self.c.material()
    }

    fn shader_name(&self) -> String {
        "face_cube".to_string()
    }
}



pub static TEXTURE_CUBE_VERTICES: [f64; 288] = [
    // positions       // normals        // texture coords
    -0.5f64, 0f64, -0.5f64, 0.0f64, 0.0f64, -1.0f64, 0.0f64, 0.0f64, 0.5f64, 0f64, -0.5f64, 0.0f64, 0.0f64, -1.0f64,
    1.0f64, 0.0f64, 0.5f64, 1f64, -0.5f64, 0.0f64, 0.0f64, -1.0f64, 1.0f64, 1.0f64, 0.5f64, 1f64, -0.5f64, 0.0f64,
    0.0f64, -1.0f64, 1.0f64, 1.0f64, -0.5f64, 1f64, -0.5f64, 0.0f64, 0.0f64, -1.0f64, 0.0f64, 1.0f64, -0.5f64, 0f64,
    -0.5f64, 0.0f64, 0.0f64, -1.0f64, 0.0f64, 0.0f64, -0.5f64, 0f64, 0.5f64, 0.0f64, 0.0f64, 1.0f64, 0.0f64, 0.0f64,
    0.5f64, 0f64, 0.5f64, 0.0f64, 0.0f64, 1.0f64, 1.0f64, 0.0f64, 0.5f64, 1f64, 0.5f64, 0.0f64, 0.0f64, 1.0f64, 1.0f64,
    1.0f64, 0.5f64, 1f64, 0.5f64, 0.0f64, 0.0f64, 1.0f64, 1.0f64, 1.0f64, -0.5f64, 1f64, 0.5f64, 0.0f64, 0.0f64,
    1.0f64, 0.0f64, 1.0f64, -0.5f64, 0f64, 0.5f64, 0.0f64, 0.0f64, 1.0f64, 0.0f64, 0.0f64, -0.5f64, 1f64, 0.5f64,
    -1.0f64, 0.0f64, 0.0f64, 1.0f64, 0.0f64, -0.5f64, 1f64, -0.5f64, -1.0f64, 0.0f64, 0.0f64, 1.0f64, 1.0f64, -0.5f64,
    0f64, -0.5f64, -1.0f64, 0.0f64, 0.0f64, 0.0f64, 1.0f64, -0.5f64, 0f64, -0.5f64, -1.0f64, 0.0f64, 0.0f64, 0.0f64,
    1.0f64, -0.5f64, 0f64, 0.5f64, -1.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, -0.5f64, 1f64, 0.5f64, -1.0f64, 0.0f64,
    0.0f64, 1.0f64, 0.0f64, 0.5f64, 1f64, 0.5f64, 1.0f64, 0.0f64, 0.0f64, 1.0f64, 0.0f64, 0.5f64, 1f64, -0.5f64,
    1.0f64, 0.0f64, 0.0f64, 1.0f64, 1.0f64, 0.5f64, 0f64, -0.5f64, 1.0f64, 0.0f64, 0.0f64, 0.0f64, 1.0f64, 0.5f64,
    0f64, -0.5f64, 1.0f64, 0.0f64, 0.0f64, 0.0f64, 1.0f64, 0.5f64, 0f64, 0.5f64, 1.0f64, 0.0f64, 0.0f64, 0.0f64,
    0.0f64, 0.5f64, 1f64, 0.5f64, 1.0f64, 0.0f64, 0.0f64, 1.0f64, 0.0f64, -0.5f64, 0f64, -0.5f64, 0.0f64, -1.0f64,
    0.0f64, 0.0f64, 1.0f64, 0.5f64, 0f64, -0.5f64, 0.0f64, -1.0f64, 0.0f64, 1.0f64, 1.0f64, 0.5f64, 0f64, 0.5f64,
    0.0f64, -1.0f64, 0.0f64, 1.0f64, 0.0f64, 0.5f64, 0f64, 0.5f64, 0.0f64, -1.0f64, 0.0f64, 1.0f64, 0.0f64, -0.5f64,
    0f64, 0.5f64, 0.0f64, -1.0f64, 0.0f64, 0.0f64, 0.0f64, -0.5f64, 0f64, -0.5f64, 0.0f64, -1.0f64, 0.0f64, 0.0f64,
    1.0f64, -0.5f64, 1f64, -0.5f64, 0.0f64, 1.0f64, 0.0f64, 0.0f64, 1.0f64, 0.5f64, 1f64, -0.5f64, 0.0f64, 1.0f64,
    0.0f64, 1.0f64, 1.0f64, 0.5f64, 1f64, 0.5f64, 0.0f64, 1.0f64, 0.0f64, 1.0f64, 0.0f64, 0.5f64, 1f64, 0.5f64, 0.0f64,
    1.0f64, 0.0f64, 1.0f64, 0.0f64, -0.5f64, 1f64, 0.5f64, 0.0f64, 1.0f64, 0.0f64, 0.0f64, 0.0f64, -0.5f64, 1f64,
    -0.5f64, 0.0f64, 1.0f64, 0.0f64, 0.0f64, 1.0f64,
];

pub static TEXTURE_CUBE_INDICES: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];
