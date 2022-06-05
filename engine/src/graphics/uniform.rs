use crate::utils::*;
use cgmath::Matrix;
use std::ffi::{c_void, CStr, CString};

use crate::debug::*;

use super::{Shader, TextureBinder, TextureId};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Uniform {
    Uint(u32),
    Int(i32),
    IntArray(Vec<i32>),
    Float(f64),
    Vec2(Vec2F),
    Vec3(Vec3F),
    Vec4(Vec4F),
    Mat3(Mat3F),
    Mat4(Mat4F),
    Bool(bool),
    Texture(TextureId),
    CubeMap(TextureId),
    UniformBuffer(UniformBuffer),
}

impl Uniform {
    pub unsafe fn serialize_into(
        &self,
        collector: &mut [f32],
        name: &str,
        textures: &mut TextureBinder,
        shader: &Shader,
    ) {
        match self {
            Uniform::Int(elem) => collector[0] = *(elem as *const i32 as *const c_void as *const f32),
            Uniform::Float(elem) => collector[0] = *elem as f32,
            Uniform::Vec2(v) => {
                collector[0] = v.x as f32;
                collector[1] = v.y as f32;
            }
            Uniform::Vec3(v) => {
                collector[0] = v.x as f32;
                collector[1] = v.y as f32;
                collector[2] = v.z as f32;
            }
            Uniform::Vec4(v) => {
                collector[0] = v.x as f32;
                collector[1] = v.y as f32;
                collector[2] = v.z as f32;
                collector[3] = v.w as f32;
            }
            Uniform::Mat3(m) => {
                let m_sz = 9;
                let m_f32 = m.cast::<f32>().unwrap();
                let m_ptr = {
                    let ptr = m_f32.as_ptr();
                    std::slice::from_raw_parts(ptr, m_sz)
                };
                for i in 0..m_sz {
                    collector[i] = m_ptr[i];
                }
            }
            Uniform::Mat4(m) => {
                let m_sz = 16;
                let m_32 = m.cast::<f32>().unwrap();
                let m_ptr = {
                    let ptr = m_32.as_ptr();
                    std::slice::from_raw_parts(ptr, m_sz)
                };
                for i in 0..m_sz {
                    collector[i] = m_ptr[i];
                }
            }
            Uniform::Texture(texture) => {
                let slot = textures.bind(&shader, &name, texture);
                // let slot = textures.get_slot(texture.id());
                // texture.bind(slot);
                // step_debug!(format!("Texture {:?} bound to {}", texture.source_string(), slot));
                // if let Some(slot) = slot_opt {
                collector[0] = *(&slot as *const u32 as *const c_void as *const f32);
                // } else {
                //   println!("Could not find an available bind point for texture");
                // }
            }
            Uniform::CubeMap(texture) => {
                // let slot = textures.get_slot(texture.id());
                // texture.bind(slot);
                let slot = textures.bind(&shader, &name, texture);
                collector[0] = *(&slot as *const u32 as *const c_void as *const f32);
            }
            _ => println!("Uniform of type {:?} not supported", self),
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum UniformType {
    Int,
    IntArray,
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Mat4,
    Bool,
    Texture,
    UniformBuffer,
}

#[allow(dead_code)]
pub enum UniformLifecycle {
    Frame,
    Runtime,
}

#[derive(Clone, Debug)]
pub struct UniformBuffer {}

impl UniformBuffer {}
