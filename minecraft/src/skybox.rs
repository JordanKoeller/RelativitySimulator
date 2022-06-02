use engine::renderer::{AttributeType, BufferLayout, DataBuffer, IndexBuffer, TextureLike, Uniform, VertexArray};
use engine::renderer::{CubeMap, Drawable};

use engine::ecs::Material;

pub struct Skybox {
    texture_name: String,
}

impl Skybox {
    #[allow(dead_code)]
    pub fn new(texture: &str) -> Skybox {
        Skybox {
            texture_name: texture.to_string(),
        }
    }
}

impl Drawable for Skybox {
    fn vertex_array(&self) -> VertexArray {
        let layout = BufferLayout::new(vec![AttributeType::Float3]);
        let vert_buff = DataBuffer::static_buffer(&SKYBOX_VERTICES, layout);
        let ind_buff = IndexBuffer::create(SKYBOX_INDICES.to_vec());
        VertexArray::new(vert_buff, ind_buff)
    }

    fn material(&self) -> Material {
        let mut material = Material::new();
        let cube_map = CubeMap::from_file(&self.texture_name);
        // cube_map.refresh();
        material.unknown_uniform("skybox", Uniform::CubeMap(cube_map));
        material
    }

    fn shader_name(&self) -> String {
        "skybox".to_string()
    }
}

static SKYBOX_VERTICES: [f64; 108] = [
    // positions       // normals        // texture coords
    -1.0f64, 1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, 1.0f64, -1.0f64, -1.0f64, 1.0f64, -1.0f64, -1.0f64, 1.0f64,
    1.0f64, -1.0f64, -1.0f64, 1.0f64, -1.0f64, -1.0f64, -1.0f64, 1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, 1.0f64,
    -1.0f64, -1.0f64, 1.0f64, -1.0f64, -1.0f64, 1.0f64, 1.0f64, -1.0f64, -1.0f64, 1.0f64, 1.0f64, -1.0f64, -1.0f64,
    1.0f64, -1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, -1.0f64, 1.0f64, -1.0f64,
    -1.0f64, -1.0f64, -1.0f64, 1.0f64, -1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64,
    -1.0f64, 1.0f64, -1.0f64, -1.0f64, 1.0f64, -1.0f64, 1.0f64, -1.0f64, 1.0f64, 1.0f64, -1.0f64, 1.0f64, 1.0f64,
    1.0f64, 1.0f64, 1.0f64, 1.0f64, -1.0f64, 1.0f64, 1.0f64, -1.0f64, 1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64,
    -1.0f64, -1.0f64, 1.0f64, 1.0f64, -1.0f64, -1.0f64, 1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, 1.0f64, 1.0f64,
    -1.0f64, 1.0f64,
];

static SKYBOX_INDICES: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];
