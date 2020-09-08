use std::ffi::{CStr, CString};


use gl;
use cgmath::prelude::*;

use super::uniform::Uniform;

pub struct Shader {
    id: u32,
    name: String
}

impl Shader {

    pub fn new(name: &str, shader_body: &str) -> Shader {
        let mut ret = Shader {
            id: 0,
            name: name.to_string()
        };

        ret
    }

    pub fn bind(&self) {
        unsafe {   
            gl::UseProgram(self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {   
            gl::UseProgram(0);
        }
    }

    pub fn set_uniform(&self, name: &CStr, unif: Uniform) {
        unsafe {
            let loc = gl::GetUniformLocation(self.id, name.as_ptr());
            match unif {
                Uniform::Int(v)   => gl::Uniform1i(loc, v),
                Uniform::Float(v) => gl::Uniform1f(loc, v),
                Uniform::Vec2(v) => gl::Uniform2f(loc, v.x, v.y),
                Uniform::Vec3(v) => gl::Uniform3f(loc, v.x, v.y, v.z),
                Uniform::Vec4(v) => gl::Uniform4f(loc, v.x, v.y, v.z, v.w),
                Uniform::Mat3(v) => gl::UniformMatrix3fv(loc, 1, gl::FALSE, v.as_ptr()),
                Uniform::Mat4(v) => gl::UniformMatrix4fv(loc, 1, gl::FALSE, v.as_ptr()),
                Uniform::Bool(v) => gl::Uniform1i(loc, v as i32),
                Uniform::IntArray(arr) => gl::Uniform1iv(loc, arr.len() as i32, &arr[0] as *const i32),
            }
            // gl::Uniform1i(lo, )
        }
    }

    // fn compile(&mut self) {

    // }
}

pub struct ShaderLibrary {

}