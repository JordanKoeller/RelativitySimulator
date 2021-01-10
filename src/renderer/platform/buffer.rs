use gl;
use std::mem::size_of;
use std::os::raw::c_void;

////////////////////
/// BUFFER OBJECTS
////////////////////

#[derive(Clone, Debug)]
pub struct VertexBuffer {
  id: u32,
  pub layout: BufferLayout,
  data: Vec<f32>,
}

#[derive(Clone, Debug)]
pub struct IndexBuffer {
  id: u32,
  data: Vec<u32>,
}

////////////////////
/// VERTEX BUFFER
////////////////////

impl VertexBuffer {
  pub fn create(data: Vec<f32>, layout: BufferLayout) -> VertexBuffer {
    let mut id = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);
    }
    VertexBuffer { id, layout, data }
  }

  pub fn init(&self) {
    self.bind();
    unsafe {
      gl::BufferData(
        gl::ARRAY_BUFFER,
        buff_sz(&self.data),
        buff_ptr(&self.data),
        gl::STATIC_DRAW,
      );
    }
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }
  }

  // pub fn unbind(&self) {
  //   unsafe {
  //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  //   }
  // }
}

// impl Drop for VertexBuffer {
//   fn drop(&mut self) {
//     self.unbind();
//     unsafe {
//       gl::DeleteBuffers(1, &mut self.id);
//     }
//   }
// }

////////////////////
/// INDEX BUFFER
////////////////////

impl IndexBuffer {
  pub fn create(data: Vec<u32>) -> Self {
    let mut id = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);
    }
    Self { id, data }
  }

  pub fn init(&self) {
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

// impl Drop for IndexBuffer {
//   fn drop(&mut self) {
//     self.unbind();
//     unsafe {
//       gl::DeleteBuffers(1, &mut self.id);
//     }
//   }
// }

//////////////////////////////////
/// BUFFER LAYOUT ////////////////
//////////////////////////////////
#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq, Copy, Debug)]
pub enum AttributeType {
  Float,
  Float2,
  Float3,
  Float4,
  Mat3,
  Mat4,
  Int,
  Int2,
  Int3,
  Int4,
  Bool,
}

impl AttributeType {
  pub fn width(&self) -> u32 {
    match self {
      AttributeType::Float => 1,
      AttributeType::Float2 => 2,
      AttributeType::Float3 => 3,
      AttributeType::Float4 => 4,
      AttributeType::Mat3 => 9,
      AttributeType::Mat4 => 16,
      AttributeType::Int => 1,
      AttributeType::Int2 => 2,
      AttributeType::Int3 => 3,
      AttributeType::Int4 => 4,
      AttributeType::Bool => 1,
    }
  }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BufferLayout(Vec<AttributeType>);

impl BufferLayout {
  pub fn new(atts: Vec<AttributeType>) -> BufferLayout {
    BufferLayout(atts)
  }
  pub fn stride(&self) -> u32 {
    self.0.iter().map(|x| x.width()).sum::<u32>() * size_of::<f32>() as u32
  }

  pub fn ind_offset_attrib(&self) -> Vec<(usize, u32, AttributeType)> {
    let mut summation = 0;
    self
      .0
      .iter()
      .enumerate()
      .map(|(i, &x)| {
        let v = summation.clone();
        summation += x.width();
        (i, v * size_of::<f32>() as u32, x)
      })
      .collect()
  }
}

//////////////////////////////////
/// HELPER FUNCTIONS /////////////
//////////////////////////////////

fn buff_sz<T>(data: &Vec<T>) -> isize {
  (size_of::<T>() * data.len()) as isize
}

fn buff_ptr<T>(data: &Vec<T>) -> *const c_void {
  &data[0] as *const T as *const c_void
}
