use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

use super::{Shader, ShaderId, TextureBinder, TextureId, Uniform};
use crate::utils::Vec3F;

#[derive(Debug, Clone, Default, Component)]
#[storage(VecStorage)]
pub struct MaterialComponent {
    uniforms: Vec<(String, Uniform)>,
}

impl MaterialComponent {
    pub fn new() -> Self {
        Self { uniforms: Vec::new() }
    }

    pub fn ambient(&mut self, v: Vec3F) {
        self.upsert_uniform("ambient".to_string(), Uniform::Vec3(v));
    }

    pub fn diffuse(&mut self, v: Vec3F) {
        self.upsert_uniform("diffuse".to_string(), Uniform::Vec3(v));
    }

    pub fn specular(&mut self, v: Vec3F) {
        self.upsert_uniform("specular".to_string(), Uniform::Vec3(v));
    }
    #[allow(dead_code)]
    pub fn shininess(&mut self, v: f32) {
        self.upsert_uniform("shininess".to_string(), Uniform::Float(v));
    }
    #[allow(dead_code)]
    pub fn dissolve(&mut self, v: f32) {
        self.upsert_uniform("dissolve".to_string(), Uniform::Float(v));
    }
    #[allow(dead_code)]
    pub fn optical_density(&mut self, v: f32) {
        self.upsert_uniform("optical_density".to_string(), Uniform::Float(v));
    }
    #[allow(dead_code)]
    pub fn diffuse_texture(&mut self, v: TextureId) {
        self.upsert_uniform("diffuse_texture".to_string(), Uniform::Texture(v));
    }

    pub fn ambient_texture(&mut self, v: TextureId) {
        self.upsert_uniform("ambient_texture".to_string(), Uniform::Texture(v));
    }

    pub fn specular_texture(&mut self, v: TextureId) {
        self.upsert_uniform("specular_texture".to_string(), Uniform::Texture(v));
    }

    pub fn normal_texture(&mut self, v: TextureId) {
        self.upsert_uniform("normal_texture".to_string(), Uniform::Texture(v));
    }
    #[allow(dead_code)]
    pub fn shininess_texture(&mut self, v: TextureId) {
        self.upsert_uniform("shininess_texture".to_string(), Uniform::Texture(v));
    }

    #[allow(dead_code)]
    pub fn dissolve_texture(&mut self, v: TextureId) {
        self.upsert_uniform("dissolve_texture".to_string(), Uniform::Texture(v));
    }

    pub fn unknown_uniform(&mut self, name: &str, uniform: Uniform) {
        self.upsert_uniform(name.to_string(), uniform);
    }
    pub fn uniforms(&self) -> &Vec<(String, Uniform)> {
        &self.uniforms
    }

    fn upsert_uniform(&mut self, name: String, value: Uniform) {
        let mut flag = true;
        for i in 0..self.uniforms.len() {
            if flag && &self.uniforms[i].0 == &name {
                self.uniforms[i].1 = value.clone();
                flag = false;
                break;
            }
        }
        if flag {
            self.uniforms.push((name, value));
        }
    }

    fn get_by_name(&self, name: &str) -> Option<&Uniform> {
        self.uniforms
            .iter()
            .find(|(unif, _)| name == unif)
            .map(|(_, value)| value)
    }

    pub fn bind_to(&self, shader: &Shader, textures: &mut TextureBinder, debug: bool) {
        if debug {
            println!("Begin Material Binding=======");
        }
        for (unif_name, unif) in self.uniforms() {
            match unif {
                Uniform::Texture(tex) => {
                    let slot = textures.get_slot(tex.id()).0;
                    shader.set_texture(slot, unif_name, tex);
                    if debug {
                        println!("{} TEXTURE => {:?}", unif_name, slot);
                    }
                }
                Uniform::CubeMap(tex) => {
                    let slot = textures.get_slot(tex.id()).0;
                    shader.set_texture(slot, unif_name, tex);
                    if debug {
                        println!("{} CUBEMAP => {:?}", unif_name, slot);
                    }
                }
                _ => {
                    shader.set_uniform(&unif_name, &unif);
                    if debug {
                        println!("{} => {:?}", unif_name, unif);
                    }
                }
            }
        }
        if debug {
            println!("End Material Binding=======");
        }
    }
}
