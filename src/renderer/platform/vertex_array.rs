use gl;
use std::os::raw::c_void;
use std::ptr;

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
    self.vertex_buffer.refresh();
    self.index_buffer.refresh();
    // if let Some(instancing_buffer) = &mut self.instancing_buffer {
    //   instancing_buffer.refresh();
    // }
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

  // pub fn refresh_vertex_buffer(&self, buff: &DataBuffer) {
  //   self.bind();
  //   let stride = buff.layout.stride();
  //   for &(i, offset, attrib) in buff.layout.ind_offset_attrib().iter() {
  //     // println!("Setting Attribute {} {} {} {}", i, attrib.width(), stride, offset);
  //     unsafe {
  //       gl::EnableVertexAttribArray(i as u32);
  //       gl::VertexAttribPointer(
  //         i as u32,
  //         attrib.width() as i32,
  //         gl::FLOAT,
  //         gl::FALSE,
  //         stride as i32,
  //         offset as *const u32 as *const c_void,
  //       );
  //     }
  //   }
  // }

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
