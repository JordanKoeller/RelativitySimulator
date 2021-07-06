use gl;
use std::os::raw::c_void;
use std::ptr;
use debug::*;

use super::{IndexBuffer, DataBuffer};

#[derive(Debug, Clone)]
pub struct VertexArray {
  id: u32,
  index_buffer: IndexBuffer,
  vertex_buffer: DataBuffer,
  pub instancing_buffer: Option<DataBuffer>,
}

impl VertexArray {
  pub fn new(vertex_buffer: DataBuffer, index_buffer: IndexBuffer) -> VertexArray {
    VertexArray {
      id: 0,
      vertex_buffer: vertex_buffer,
      index_buffer,
      instancing_buffer: None,
    }
  }

  pub fn refresh(&mut self) {
    unsafe {
      gl::CreateVertexArrays(1, &mut self.id);
      gl::BindVertexArray(self.id);
    }
    self.vertex_buffer.init();
    self.vertex_buffer.refresh(0);
    if let Some(instancing_buffer) = &mut self.instancing_buffer {
      println!("REfreshing instancing buffer {}", instancing_buffer.len());
      instancing_buffer.init();
      instancing_buffer.refresh(self.vertex_buffer.num_attributes());
    }
    self.index_buffer.refresh();
    self.unbind();
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
        instance_count as i32
      );
    }
    // gl_check_error!(&format!("count = {} instancecounty = {}", self.index_buffer.len(),instance_count));
  }

}
