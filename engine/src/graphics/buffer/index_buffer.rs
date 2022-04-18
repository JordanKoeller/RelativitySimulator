use gl;

use super::{buff_ptr, buff_sz};


////////////////////
/// INDEX BUFFER
////////////////////
///
#[derive(Clone, Debug)]
pub struct IndexBuffer {
    id: u32,
    data: Vec<u32>,
    // bound: bool,
}

impl IndexBuffer {
    pub fn new(data: Vec<u32>) -> Self {
        Self { id: u32::MAX, data }
    }

    pub fn refresh(&mut self) {
        if self.id == u32::MAX {
            unsafe {
                gl::GenBuffers(1, &mut self.id);
            }
        }
        self.init();
    }

    fn init(&mut self) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                buff_sz(&self.data),
                buff_ptr(&self.data),
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    // pub fn unbind(&self) {
    //   unsafe {
    //     gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    //   }
    // }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[derive(Default)]
pub struct IndexBufferBuilder {
    data: Vec<u32>
}

impl IndexBufferBuilder {
    pub fn set_data(mut self, data: Vec<u32>) -> Self {
        self.data = data;
        self
    }

    pub fn build(self) -> IndexBuffer {
        let buf = IndexBuffer::new(self.data);
        buf
    }
}