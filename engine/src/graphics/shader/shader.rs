use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;

use cgmath::prelude::*;

use crate::graphics::TextureId;
use crate::graphics::Uniform;
use crate::utils::RwAssetRef;

use super::shader_delegates::{ShaderBinder, UniformSlots};
use super::shader_preprocessor;

pub struct Shader {
    binder: Box<dyn ShaderBinder>,
    id: RwAssetRef<u32>,
    element_type: gl::types::GLenum,
    uniform_slots: UniformSlots,
}

impl Shader {
    pub fn new(
        binder: Box<dyn ShaderBinder>,
        id: RwAssetRef<u32>,
        element_type: gl::types::GLenum,
        uniform_slots: UniformSlots,
    ) -> Self {
        Self {
            binder,
            id,
            element_type,
            uniform_slots,
        }
    }

    pub fn bind(&self) {
        self.binder.bind(*self.id.get());
    }
    pub fn unbind(&self) {
        self.binder.unbind(*self.id.get());
    }

    pub fn set_uniform(&self, name: &CStr, unif: &Uniform) {
        let uniform_slot = self.uniform_slots.get_slot(name, *self.id.get());
        set_unif_helper(unif, uniform_slot);
    }

    pub fn set_texture(&self, slot: u32, name: &CStr, texture: &TextureId) {
        texture.bind(slot);
        let unif = Uniform::Int(slot as i32);
        self.set_uniform(name, &unif);
    }

    pub fn id(&self) -> u32 {
        *self.id.get()
    }
}

unsafe impl Sync for Shader {}
unsafe impl Send for Shader {}

fn set_unif_helper(unif: &Uniform, loc: i32) {
    unsafe {
        match unif {
            Uniform::Int(v) => gl::Uniform1i(loc, v.clone()),
            Uniform::Float(v) => gl::Uniform1f(loc, v.clone()),
            Uniform::Vec2(v) => gl::Uniform2f(loc, v.x, v.y),
            Uniform::Vec3(v) => gl::Uniform3f(loc, v.x, v.y, v.z),
            Uniform::Vec4(v) => gl::Uniform4f(loc, v.x, v.y, v.z, v.w),
            Uniform::Mat3(v) => gl::UniformMatrix3fv(loc, 1, gl::FALSE, v.as_ptr()),
            Uniform::Mat4(v) => gl::UniformMatrix4fv(loc, 1, gl::FALSE, v.as_ptr()),
            Uniform::Bool(v) => gl::Uniform1i(loc, v.clone() as i32),
            Uniform::IntArray(arr) => gl::Uniform1iv(loc, arr.len() as i32, &arr[0] as *const i32),
            _ => panic!("Please set texture uniforms through the set_texture method"),
        }
    }
}
