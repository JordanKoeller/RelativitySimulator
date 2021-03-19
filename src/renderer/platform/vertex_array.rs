use gl;
use std::os::raw::c_void;
use std::ptr;

use super::{IndexBuffer, VertexBuffer};

#[derive(Debug, Clone)]
pub struct VertexArray {
  id: u32,
  vertex_buffers: Vec<VertexBuffer>,
  index_buffer: IndexBuffer,
}

impl VertexArray {
  pub fn new(vertex_buffers: Vec<VertexBuffer>, index_buffer: IndexBuffer) -> VertexArray {
    VertexArray {
      id: 0,
      vertex_buffers: vertex_buffers,
      index_buffer,
    }
  }

  pub fn refresh(&mut self) {
    unsafe {
      gl::CreateVertexArrays(1, &mut self.id);
      gl::BindVertexArray(self.id);
    }
    for vb_i in 0..self.vertex_buffers.len() {
      self.vertex_buffers[vb_i].refresh();
        self.refresh_vertex_buffer(&self.vertex_buffers[vb_i]);
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

  pub fn refresh_vertex_buffer(&self, buff: &VertexBuffer) {
    self.bind();
    let stride = buff.layout.stride();
    for &(i, offset, attrib) in buff.layout.ind_offset_attrib().iter() {
      // println!("Setting Attribute {} {} {} {}", i, attrib.width(), stride, offset);
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
}

// impl Drop for VertexArray {
//   fn drop(&mut self) {
//     self.unbind();
//     unsafe {
//       gl::DeleteVertexArrays(1, &mut self.id);
//     }
//   }
// }
