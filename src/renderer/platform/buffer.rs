use gl;
use std::mem::size_of;
use std::os::raw::c_void;
use std::slice::SliceIndex;

use renderer::platform::BufferLayout;


// use super::GLBus;


////////////////////
/// BUFFER OBJECT IMPLS
////////////////////

#[derive(Clone, Debug)]
pub struct DataBuffer {
  id: u32,
  pub layout: BufferLayout,
  data: Vec<f32>,
  config: BufferConfig,
  // bound: bool,
}


#[derive(Clone, Debug)]
struct BufferConfig {
  storage_type: gl::types::GLenum,
  buffer_type: gl::types::GLenum,
  attrib_divisor: u32,
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
}

// type BufferConfig {

// }

//////////////////////////////////
/// DATA BUFFER 
//////////////////////////////////

//////////////////////////////////
/// Constructors
//////////////////////////////////
impl DataBuffer {
  pub fn static_buffer<E>(data: &[E], layout: BufferLayout) -> DataBuffer {
    Self::validate_construct(data, layout, BufferConfig::static_vbo(), u32::MAX)
  }

  pub fn dynamic_buffer<E>(data: &[E], layout: BufferLayout) -> DataBuffer {
    Self::validate_construct(data, layout, BufferConfig::dynamic_vbo(), u32::MAX)
  }

  pub fn instancing_buffer(layout: BufferLayout, num_elems: u32) -> DataBuffer {
    let mut reservation: Vec<f32> = Vec::with_capacity((num_elems * layout.stride()) as usize);
    reservation.push(0f32);
    Self::validate_construct(&reservation, layout, BufferConfig::instancing_buffer(), num_elems)
  } 

  pub fn ubo<E>(layout: BufferLayout) -> DataBuffer {
    let mut data: Vec<E> = Vec::new();
    data.reserve(layout.stride() as usize);
    Self::validate_construct(&data, layout, BufferConfig::ubo(), u32::MAX)
  }

  fn validate_construct<E>(data: &[E], layout: BufferLayout, config: BufferConfig, id: u32) -> Self {
    // assert_eq!(size_of::<T>(), layout.stride() as usize);
    let casted = unsafe {
      cast_slice::<E, f32>(data)
    };
    let vec = casted.to_vec();
    Self {
      id, data: vec, layout, config
    }
  }


}

//////////////////////////////////
/// Dynamic buffer access methods
//////////////////////////////////

type Elem = f32;
impl DataBuffer {

  pub fn read_slice(&self, start: usize, end: usize) -> &[f32] {
    unsafe {self.data.get_unchecked(start..end)}
  }

  pub fn splice_inplace<F: FnOnce(&mut [f32]) -> ()>(&mut self, start: usize, end: usize, f: F)
  {
    f(unsafe {self.data.get_unchecked_mut(start..end)});
    self.set_sub_buffer(start, end);
  }

  pub fn splice_as<E: Sized, F: FnOnce(&mut [E]) -> ()>(&mut self, start: usize, end: usize, f: F) {
    // let slice = unsafe {
    //   let raw_slice = self.data.get_unchecked_mut(start..end);
    //   let raw_ptr = &mut raw_slice[0] as *mut Elem as *mut c_void as *mut E;
    //   // raw_ptr.
    //   std::slice::from_raw_parts_mut(raw_ptr, end - start)
    // };
    let slice = unsafe {
      cast_slice_mut::<f32, E>(&mut self.data)
    };
    f(&mut slice[start..end]);
    let offset = start * size_of::<E>() / size_of::<f32>();
    let len = (end - start) * size_of::<E>() / size_of::<f32>();
    self.set_sub_buffer(offset, len);
  }

  pub fn zero_out<E: Sized>(&mut self, start: usize, end: usize) {
    let elem_sz = size_of::<E>() / size_of::<f32>();
    let offset = start * elem_sz;
    let length = (end - start) * elem_sz;
    self.splice_inplace(offset, length, move |slc| {
      for i in 0..slc.len() {
        slc[i] = 0f32;
      }
    });

  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  // pub fn write_slice(&mut self, start: usize, end: usize, slice: &[f32]) {
  //   self.data.splice(start..end, slice.iter().cloned());
  // } 
}

//////////////////////////////////
/// Piping to the GPU
//////////////////////////////////

impl DataBuffer {

  pub fn refresh(&mut self) {
    let stride = self.layout.stride();
    for &(i, offset, attrib) in self.layout.ind_offset_attrib().iter() {
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
        gl::VertexAttribDivisor(i as u32, self.config.attrib_divisor);
      }
    }
  }

  pub fn init(&mut self) {
    if self.id == u32::MAX {
      unsafe {
        gl::GenBuffers(1, &mut self.id);
      }
    }
    self.bind();
    unsafe {
      gl::BufferData(
        self.config.buffer_type,
        buff_sz(&self.data),
        buff_ptr(&self.data),
        self.config.storage_type,
      );
    }
  }

  pub fn set_sub_buffer(&self, start: usize, end: usize) {
    self.bind();
    unsafe {
      let slice_ref = self.data.get_unchecked(start..end);
      gl::BufferSubData(
        self.config.buffer_type,
        (size_of::<Elem>() * start) as isize,
        (size_of::<Elem>() * (end - start)) as isize,
        &slice_ref[0] as *const Elem as *const c_void,
      )
    }
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindBuffer(self.config.buffer_type, self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindBuffer(self.config.buffer_type, 0);
    }
  }

  pub fn destroy(&mut self) {
    unsafe {
      gl::DeleteBuffers(1, &self.id);
      self.id = u32::MAX;
    }
  }
}


// impl Drop for DataBuffer<T> {
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
/// 
#[derive(Clone, Debug)]
pub struct IndexBuffer {
  id: u32,
  data: Vec<u32>,
  // bound: bool,
}

impl IndexBuffer {
  pub fn create(data: Vec<u32>) -> Self {
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

//////////////////////////////////
/// HELPER FUNCTIONS /////////////
//////////////////////////////////

fn buff_sz<T>(data: &Vec<T>) -> isize {
  (size_of::<T>() * data.len()) as isize
}

fn buff_ptr<T>(data: &Vec<T>) -> *const c_void {
  &data[0] as *const T as *const c_void
}

unsafe fn cast_slice<F, T>(data: &[F]) -> &[T] {
    let raw_ptr = &data[0] as *const F as *const c_void as *const T;
    let old_sz = size_of::<F>();
    let new_sz = size_of::<T>();
    if new_sz < old_sz {
      std::slice::from_raw_parts(raw_ptr, data.len() * old_sz / new_sz)
    } else if old_sz < new_sz {
      std::slice::from_raw_parts(raw_ptr, data.len() * new_sz / old_sz)
    } else {
      std::slice::from_raw_parts(raw_ptr, data.len())
    }
}

unsafe fn cast_slice_mut<F, T>(data: &mut [F]) -> &mut [T] {
  let old_sz = size_of::<F>();
  let new_sz = size_of::<T>();
  let raw_ptr = &mut data[0] as *mut F as *mut c_void as *mut T;
  if new_sz < old_sz {
    std::slice::from_raw_parts_mut(raw_ptr, data.len() * old_sz / new_sz)
  } else if old_sz < new_sz {
    std::slice::from_raw_parts_mut(raw_ptr, data.len() * new_sz / old_sz)
  } else {
    std::slice::from_raw_parts_mut(raw_ptr, data.len())
  }
  // let sz_ratio = old_sz / new_sz;
}