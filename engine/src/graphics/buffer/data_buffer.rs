use std::mem::size_of;
use std::os::raw::c_void;

use gl;

use super::BufferConfig;
use super::BufferLayout;
use super::{buff_ptr, buff_sz};

#[derive(Clone, Debug)]
pub struct DataBuffer {
    id: u32,
    pub layout: BufferLayout,
    data: Vec<f32>,
    config: BufferConfig,
    // bound: bool,
}

impl DataBuffer {
    pub fn new(data: Vec<f32>, layout: BufferLayout, config: BufferConfig, id: u32) -> Self {
        // assert_eq!(size_of::<T>(), layout.stride() as usize);
        Self {
            id,
            data,
            layout,
            config,
        }
    }

    pub fn refresh(&mut self, attrib_start: u32) {
        self.bind();
        unsafe {
            gl::BufferData(
                self.config.buffer_type.to_gl_enum(),
                buff_sz(&self.data),
                buff_ptr(&self.data),
                self.config.storage_type.to_gl_enum(),
            );
        }
        let stride = self.layout.stride();
        for &(i, offset, attrib) in self.layout.ind_offset_attrib().iter() {
            let attrib_length = attrib.width() / attrib.num_calls();
            let attrib_index = i as u32 + attrib_start;
            // for iteration in 0..attrib.num_calls() {
            unsafe {
                gl::EnableVertexAttribArray(attrib_index);
                gl::VertexAttribPointer(
                    attrib_index,
                    attrib_length as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as i32,
                    offset as *const u32 as *const c_void,
                );
                gl::VertexAttribDivisor(attrib_index, self.config.attrib_divisor);
            }
            // }
        }
    }

    pub fn num_attributes(&self) -> u32 {
        self.layout.ind_offset_attrib().len() as u32
    }

    pub fn init(&mut self) {
        if self.id == u32::MAX {
            unsafe {
                gl::GenBuffers(1, &mut self.id);
            }
        }
    }

    pub fn set_sub_buffer(&self, start: usize, end: usize) {
        self.bind();
        unsafe {
            let slice_ref = self.data.get_unchecked(start..end);
            gl::BufferSubData(
                self.config.buffer_type.to_gl_enum(),
                (size_of::<f32>() * start) as isize,
                (size_of::<f32>() * (end - start)) as isize,
                &slice_ref[0] as *const f32 as *const c_void,
            )
        }
    }

    pub fn bind(&self) {
        if self.id == u32::MAX {
            panic!("Tried to bind a buffer that has not been initialized!");
        }
        unsafe {
            gl::BindBuffer(self.config.buffer_type.to_gl_enum(), self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.config.buffer_type.to_gl_enum(), 0);
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
            self.id = u32::MAX;
        }
    }
}
