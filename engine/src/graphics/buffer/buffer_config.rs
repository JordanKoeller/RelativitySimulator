use gl;

#[derive(Clone, Debug)]
pub enum BufferStorageLevel {
  STATIC,
  DYNAMIC,
}

impl BufferStorageLevel {
  pub fn to_gl_enum(&self) -> gl::types::GLenum {
    match self {
      BufferStorageLevel::DYNAMIC => gl::DYNAMIC_DRAW,
      BufferStorageLevel::STATIC => gl::STATIC_DRAW,
    }
  }
}

#[derive(Clone, Debug)]
pub enum BufferType {
  UNIFORM,
  ARRAY,
}

impl BufferType {
  pub fn to_gl_enum(&self) -> gl::types::GLenum {
    match self {
      BufferType::UNIFORM => gl::UNIFORM_BUFFER,
      BufferType::ARRAY => gl::ARRAY_BUFFER,
    }
  }
}

#[derive(Clone, Debug)]
pub struct BufferConfig {
  pub storage_type: BufferStorageLevel,
  pub buffer_type: BufferType,
  pub attrib_divisor: u32,
}

impl BufferConfig {
  pub fn ubo() -> Self {
    Self {
      storage_type: BufferStorageLevel::DYNAMIC,
      buffer_type: BufferType::UNIFORM,
      attrib_divisor: 0,
    }
  }
  pub fn static_vbo() -> Self {
    Self {
      storage_type: BufferStorageLevel::STATIC,
      buffer_type: BufferType::ARRAY,
      attrib_divisor: 0,
    }
  }

  pub fn dynamic_vbo() -> Self {
    Self {
      storage_type: BufferStorageLevel::DYNAMIC,
      buffer_type: BufferType::ARRAY,
      attrib_divisor: 0,
    }
  }

  pub fn instancing_buffer() -> Self {
    Self {
      storage_type: BufferStorageLevel::DYNAMIC,
      buffer_type: BufferType::ARRAY,
      attrib_divisor: 1,
    }
  }

  pub fn uniform_buffer() -> Self {
    Self {
      storage_type: BufferStorageLevel::DYNAMIC,
      buffer_type: BufferType::UNIFORM,
      attrib_divisor: 0,
    }
  }
  pub fn static_buffer() -> Self {
    Self {
      storage_type: BufferStorageLevel::STATIC,
      buffer_type: BufferType::ARRAY,
      attrib_divisor: 0,
    }
  }

  pub fn dynamic_buffer() -> Self {
    Self {
      storage_type: BufferStorageLevel::DYNAMIC,
      buffer_type: BufferType::ARRAY,
      attrib_divisor: 0,
    }
  }
}
