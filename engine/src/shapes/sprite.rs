use cgmath::prelude::*;
use crate::ecs::{components::Material, MyBuilder};
use crate::renderer::{AttributeType, BufferLayout, DataBuffer, Drawable, IndexBuffer, Texture, TextureLike, VertexArray};
use specs::prelude::*;
use crate::utils::{Vec2F, Vec3F};

#[repr(C)]
#[derive(Clone)]
struct SpriteVertex {
    pub position: Vec3F,
    pub uv: Vec2F,
}

impl SpriteVertex {
    pub const fn new(a: f32, b: f32, c: f32, d: f32, e: f32) -> Self {
        Self {
            position: Vec3F::new(a, b, c),
            uv: Vec2F::new(d, e),
        }
    }
}

static QUAD_VERTICES: [f32; 32] = [
    0.5f32, 0.5f32, 0.0f32, 0f32, 0f32, -1f32, 1.0f32, 1.0f32, // top right
    0.5f32, -0.5f32, 0.0f32, 0f32, 0f32, -1f32, 1.0f32, 0.0f32, // bottom right
    -0.5f32, -0.5f32, 0.0f32, 0f32, 0f32, -1f32, 0.0f32, 0.0f32, // bottom left
    -0.5f32, 0.5f32, 0.0f32, 0f32, 0f32, -1f32, 0.0f32, 1.0f32, // top left
];

static QUAD_INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

pub struct Sprite {
    material: Material,
    vertex_array: VertexArray,
    instanced: bool,
    // aspect_ratio: Vec2F,
}

impl Sprite {
    pub fn new(texture: &str, instanced: bool) -> Self {
        let mut mat = Material::new();
        let tex = Texture::from_file(texture);
        // tex.refresh();
        let layout = BufferLayout::new(vec![
            AttributeType::Float3,
            AttributeType::Float3,
            AttributeType::Float2,
        ]);
        let vertex_buff = DataBuffer::static_buffer(&QUAD_VERTICES, layout);

        let ind_buff = IndexBuffer::create(QUAD_INDICES.to_vec());
        let vertex_array = VertexArray::new(vertex_buff, ind_buff);
        mat.diffuse_texture(tex);
        Self {
            material: mat,
            vertex_array: vertex_array,
            instanced,
            // aspect_ratio: Vec2F::new(1f32, 1f32),
        }
    }
}

impl Drawable for Sprite {
    fn shader_name(&self) -> String {
        if self.instance_attributes().is_some() {
            "instanced".to_string()
        } else {
            "default_texture".to_string()
        }
    }
    fn vertex_array(&self) -> VertexArray {
        self.vertex_array.clone()
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn instance_attributes(&self) -> Option<Vec<(String, AttributeType)>> {
        if self.instanced {
            Some(vec![
                ("model".to_string(), AttributeType::Mat4),
                ("diffuse_texture".to_string(), AttributeType::Int),
                // ("ambient".to_string(), AttributeType::Float3),
            ])
        } else {
            None
        }
    }
}
