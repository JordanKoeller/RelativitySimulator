use gl;
use std::mem::size_of;
use std::os::raw::c_void;

use super::{VertexBuffer, IndexBuffer};

pub struct VertexArray {
    id: u32,
    vertex_buffers: Vec<VertexBuffer>,
    index_buffer: IndexBuffer,
}

impl VertexArray {

    pub fn new(vertex_buffers: Vec<VertexBuffer>, index_buffer: IndexBuffer) -> VertexArray {
        let mut ret = VertexArray {
            id: 0, vertex_buffers: Vec::new(), index_buffer
        };
        unsafe {
            gl::CreateVertexArrays(1, &mut ret.id);
        }
        for vb in vertex_buffers {
            ret.add_vertex_buffer(vb);
        }
        ret.unbind();
        ret
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn add_vertex_buffer(&mut self, buff: VertexBuffer) {
        self.bind();
        buff.bind();
        let stride = buff.layout.stride();
        for &(i, offset, attrib) in buff.layout.ind_offset_attrib().iter() {
            unsafe {
                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(
                    i as u32,
                    attrib.width() as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as i32,
                    offset as *const u32 as *const c_void,
                );
            }
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        self.unbind();
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.id);
        }
    }
}