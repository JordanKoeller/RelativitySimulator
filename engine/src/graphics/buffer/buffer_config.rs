use gl;

#[derive(Clone, Debug)]
pub struct BufferConfig {
    pub storage_type: gl::types::GLenum,
    pub buffer_type: gl::types::GLenum,
    pub attrib_divisor: u32,
}

impl BufferConfig {
    pub fn ubo() -> Self {
        Self {
            storage_type: gl::DYNAMIC_DRAW,
            buffer_type: gl::UNIFORM_BUFFER,
            attrib_divisor: 0,
        }
    }
    pub fn static_vbo() -> Self {
        Self {
            storage_type: gl::STATIC_DRAW,
            buffer_type: gl::ARRAY_BUFFER,
            attrib_divisor: 0,
        }
    }

    pub fn dynamic_vbo() -> Self {
        Self {
            storage_type: gl::DYNAMIC_DRAW,
            buffer_type: gl::ARRAY_BUFFER,
            attrib_divisor: 0,
        }
    }

    pub fn instancing_buffer() -> Self {
        Self {
            storage_type: gl::DYNAMIC_DRAW,
            buffer_type: gl::ARRAY_BUFFER,
            attrib_divisor: 1,
        }
    }

    pub fn uniform_buffer() -> Self {
        Self {
            storage_type: gl::DYNAMIC_DRAW,
            buffer_type: gl::UNIFORM_BUFFER,
            attrib_divisor: 0,
        }
    }
    pub fn static_buffer() -> Self {
        Self {
            storage_type: gl::STATIC_DRAW,
            buffer_type: gl::ARRAY_BUFFER,
            attrib_divisor: 0,
        }
    }

    pub fn dynamic_buffer() -> Self {
        Self {
            storage_type: gl::DYNAMIC_DRAW,
            buffer_type: gl::ARRAY_BUFFER,
            attrib_divisor: 0,
        }
    }
}