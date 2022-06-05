use crate::debug::*;
use gl;
use std::os::raw::c_void;
use std::ptr;

use super::{DataBuffer, IndexBuffer};
use crate::utils::RwAssetRef;

#[derive(Debug, Clone)]
pub struct VertexArray {
    id: RwAssetRef<u32>,
    index_buffer: IndexBuffer,
    vertex_buffer: DataBuffer,
    pub instancing_buffer: Option<DataBuffer>,
}

impl VertexArray {
    pub fn new(vertex_buffer: DataBuffer, index_buffer: IndexBuffer, id: RwAssetRef<u32>) -> VertexArray {
        VertexArray {
            id,
            vertex_buffer: vertex_buffer,
            index_buffer,
            instancing_buffer: None,
        }
    }

    pub fn id(&self) -> u32 {
        *self.id.get()
    }

    pub fn refresh(&mut self) {
        let mut id = self.id();
        unsafe {
            gl::CreateVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
        }
        self.vertex_buffer.init();
        self.vertex_buffer.refresh(0);
        if let Some(instancing_buffer) = &mut self.instancing_buffer {
            instancing_buffer.init();
            instancing_buffer.refresh(self.vertex_buffer.num_attributes());
        }
        self.index_buffer.refresh();
        self.unbind();
        self.id.set(id);
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id());
        }
    }

    pub fn add_instancing_buffer(&mut self, vbo: DataBuffer, auto_refresh: bool) {
        if let Some(old_vbo) = &mut self.instancing_buffer {
            old_vbo.destroy();
        }
        self.instancing_buffer = Some(vbo);
        if auto_refresh {
            self.refresh();
        }
    }

    pub fn draw(&self, elem_type: &gl::types::GLenum) {
        unsafe {
            gl::DrawElements(
                *elem_type,
                self.index_buffer.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }

    pub fn draw_instanced(&self, elem_type: &gl::types::GLenum, instance_count: usize) {
        unsafe {
            gl::DrawElementsInstanced(
                *elem_type,
                self.index_buffer.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
                instance_count as i32,
            );
        }
    }
}
