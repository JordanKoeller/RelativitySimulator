use std::ptr;

use gl;


#[derive(Copy, Clone)]
pub struct GLBuffer {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub num_elems: i32,
}

impl GLBuffer {
    pub fn draw(&self) {
        // draw mesh
        // println!("{}, {}", self.vao, self.num_elems);
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.num_elems, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }
}
