use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};


use std::ffi::{CStr, CString};

use crate::renderer::{AttributeType, Mesh, Shader, TextureBinder, VertexArray};
use crate::utils::*;

use crate::renderer::{Texture, TextureLike, Uniform, DEBUG_TEXTURE, WHITE_TEXTURE};

#[derive(Debug, Clone, Default)]
pub struct Material {
    // Boilerplate implementation at end of file.
    uniforms: Vec<(CString, Uniform)>,
}
impl Component for Material {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct MeshComponent {
    pub mesh: Mesh,
    generation: u32,
    needs_refresh: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DrawableId(pub usize, pub usize);

impl Component for DrawableId {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl MeshComponent {
    pub fn new(va: VertexArray, shader_name: String) -> Self {
        Self {
            mesh: Mesh::new(va, shader_name),
            generation: 0u32,
            needs_refresh: true,
        }
    }

    pub fn from(m: Mesh) -> Self {
        Self {
            mesh: m,
            generation: 0u32,
            needs_refresh: false,
        }
    }

    pub fn refresh(&mut self) {
        if self.needs_refresh {
            self.mesh.vao.refresh();
            self.generation += 1;
            self.needs_refresh = false;
        }
    }
}

impl Material {
    pub fn ambient(&mut self, v: Vec3F) {
        self.upsert_uniform(c_str!("ambient"), Uniform::Vec3(v));
    }

    pub fn diffuse(&mut self, v: Vec3F) {
        self.upsert_uniform(c_str!("diffuse"), Uniform::Vec3(v));
    }

    pub fn specular(&mut self, v: Vec3F) {
        self.upsert_uniform(c_str!("specular"), Uniform::Vec3(v));
    }
    #[allow(dead_code)]
    pub fn shininess(&mut self, v: f32) {
        self.upsert_uniform(c_str!("shininess"), Uniform::Float(v));
    }
    #[allow(dead_code)]
    pub fn dissolve(&mut self, v: f32) {
        self.upsert_uniform(c_str!("dissolve"), Uniform::Float(v));
    }
    #[allow(dead_code)]
    pub fn optical_density(&mut self, v: f32) {
        self.upsert_uniform(c_str!("optical_density"), Uniform::Float(v));
    }
    #[allow(dead_code)]
    pub fn diffuse_texture(&mut self, v: Texture) {
        self.upsert_uniform(c_str!("diffuse_texture"), Uniform::Texture(v));
    }

    pub fn ambient_texture(&mut self, v: Texture) {
        self.upsert_uniform(c_str!("ambient_texture"), Uniform::Texture(v));
    }

    pub fn specular_texture(&mut self, v: Texture) {
        self.upsert_uniform(c_str!("specular_texture"), Uniform::Texture(v));
    }

    pub fn normal_texture(&mut self, v: Texture) {
        self.upsert_uniform(c_str!("normal_texture"), Uniform::Texture(v));
    }
    #[allow(dead_code)]
    pub fn shininess_texture(&mut self, v: Texture) {
        self.upsert_uniform(c_str!("shininess_texture"), Uniform::Texture(v));
    }

    #[allow(dead_code)]
    pub fn dissolve_texture(&mut self, v: Texture) {
        self.upsert_uniform(c_str!("dissolve_texture"), Uniform::Texture(v));
    }

    pub fn unknown_uniform(&mut self, name: &str, uniform: Uniform) {
        let c_str = CString::new(name).expect("Could not convert string to cstring");
        self.upsert_uniform(&c_str, uniform);
    }
    pub fn uniforms(&self) -> &Vec<(CString, Uniform)> {
        &self.uniforms
    }

    pub fn new() -> Material {
        let mut ret = Material { uniforms: Vec::new() };
        ret.diffuse_texture(WHITE_TEXTURE.clone());
        ret.ambient_texture(WHITE_TEXTURE.clone());
        ret.specular_texture(WHITE_TEXTURE.clone());

        ret.diffuse(Vec3F::new(1f32, 1f32, 1f32));
        ret.ambient(Vec3F::new(1f32, 1f32, 1f32));
        ret.specular(Vec3F::new(1f32, 1f32, 1f32));
        ret.normal_texture(WHITE_TEXTURE.clone());
        // ret.unknown_uniform("debug_texture", Uniform::Texture(DEBUG_TEXTURE.clone()));
        ret
    }

    fn upsert_uniform(&mut self, c_str: &CStr, value: Uniform) {
        let c_string = CString::from(c_str);
        let mut flag = true;
        for i in 0..self.uniforms.len() {
            if flag && self.uniforms[i].0 == c_string {
                self.uniforms[i].1 = value.clone();
                flag = false;
                break;
            }
        }
        if flag {
            self.uniforms.push((c_string, value));
        }
    }

    fn get_by_name(&self, name: &str) -> Option<&Uniform> {
        let c_name = CString::new(name).unwrap();
        self.uniforms
            .iter()
            .find(|(unif, _)| &c_name == unif)
            .map(|(_, value)| value)
    }

    pub fn refresh(&mut self) {
        self.uniforms.iter_mut().for_each(|(_, uniform)| match uniform {
            Uniform::CubeMap(c) => c.refresh(),
            Uniform::Texture(t) => t.refresh(),
            _ => {}
        });
    }

    pub fn serialize_into(
        &self,
        collector: &mut [f32],
        order: &Vec<(String, AttributeType)>,
        texture_binder: &mut TextureBinder,
        shader: &Shader,
    ) {
        let mut offset: usize = 0;
        for i in 1..order.len() {
            if let Some(uniform) = self.get_by_name(&order[i].0) {
                let elem_width = order[i].1.width() as usize;
                unsafe {
                    uniform.serialize_into(
                        &mut collector[offset..offset + elem_width],
                        &order[i].0,
                        texture_binder,
                        shader,
                    );
                }
                offset += elem_width;
            }
        }
    }

    pub fn bind_to(&self, shader: &Shader, textures: &mut TextureBinder) {
        for (unif_name, unif) in self.uniforms() {
            match unif {
                Uniform::Texture(tex) => {
                    shader.set_texture(textures.get_slot(tex.id()).0, unif_name, tex);
                }
                Uniform::CubeMap(tex) => {
                    shader.set_texture(textures.get_slot(tex.id()).0, unif_name, tex);
                }
                _ => shader.set_uniform(&unif_name, &unif),
            }
        }
    }
}
