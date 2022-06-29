use std::mem::size_of;
use std::os::raw::c_void;

use gl;

use super::BufferConfig;
use super::BufferLayout;
use super::Bufferable;
use super::{buff_ptr, buff_sz, unsafe_cast, unsafe_cast_mut};

const FLOAT_SIZE: usize = 4usize;

#[derive(Clone, Debug)]
pub struct DataBuffer {
    id: u32,
    pub layout: BufferLayout,
    data: Vec<f32>,
    config: BufferConfig,
    delta_range: Option<(usize, usize)>, // bound: bool,
}

impl DataBuffer {
    pub fn new(data: Vec<f32>, layout: BufferLayout, config: BufferConfig, id: u32) -> Self {
        // assert_eq!(size_of::<T>(), layout.stride() as usize);
        Self {
            id,
            data,
            layout,
            config,
            delta_range: None,
        }
    }

    pub fn refresh(&mut self, attrib_start: u32) {
        self.bind();
        unsafe {
            gl::BufferData(
                self.config.buffer_type.to_gl_enum(),
                buff_sz(&self.data),
                buff_ptr(&self.data),
                self.config.storage_type.to_gl_enum(),
            );
        }
        let stride = self.layout.stride();
        for &(i, offset, attrib) in self.layout.ind_offset_attrib().iter() {
            let attrib_length = attrib.width() / attrib.num_calls();
            let attrib_index = i as u32 + attrib_start;
            // for iteration in 0..attrib.num_calls() {
            unsafe {
                gl::EnableVertexAttribArray(attrib_index);
                gl::VertexAttribPointer(
                    attrib_index,
                    attrib_length as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as i32,
                    offset as *const u32 as *const c_void,
                );
                gl::VertexAttribDivisor(attrib_index, self.config.attrib_divisor);
            }
            // }
        }
    }

    pub fn num_attributes(&self) -> u32 {
        self.layout.ind_offset_attrib().len() as u32
    }

    pub fn init(&mut self) {
        if self.id == u32::MAX {
            unsafe {
                gl::GenBuffers(1, &mut self.id);
            }
        }
    }

    pub fn sync_gpu(&mut self) {
        self.bind();
        if let Some((start, end)) = self.delta_range {
            unsafe {
                let slice_ref = self.data.get_unchecked(start..end);
                gl::BufferSubData(
                    self.config.buffer_type.to_gl_enum(),
                    (size_of::<f32>() * start) as isize,
                    (size_of::<f32>() * (end - start)) as isize,
                    &slice_ref[0] as *const f32 as *const c_void,
                )
            }
        }
    }

    pub fn bind(&self) {
        if self.id == u32::MAX {
            panic!("Tried to bind a buffer that has not been initialized!");
        }
        unsafe {
            gl::BindBuffer(self.config.buffer_type.to_gl_enum(), self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.config.buffer_type.to_gl_enum(), 0);
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
            self.id = u32::MAX;
        }
    }

    pub fn as_view<T>(&mut self) -> BufferView<'_, T> {
        BufferView::from(self)
    }
}

pub struct BufferView<'a, T> {
    stride: usize,
    buffer_ref: &'a mut DataBuffer,
    delta_min: usize,
    delta_max: usize,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T> BufferView<'a, T> {
    fn from(buffer_ref: &'a mut DataBuffer) -> Self {
        let stride = size_of::<T>();
        if stride as u32 != buffer_ref.layout.stride() {
            println!(
                "Cannot create view into DataBuffer because view type of {} does not have the proper stride",
                std::any::type_name::<T>()
            );
            println!(
                "Buffer has stride {}, but passed in has stride {}",
                buffer_ref.layout.stride(),
                stride
            );
            panic!("");
        }
        let delta_max = buffer_ref.data.len();
        Self {
            stride: stride / FLOAT_SIZE,
            buffer_ref,
            delta_max: delta_max,
            delta_min: 0,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn get(&self, index: usize) -> &T {
        let offset = self.stride * index;
        unsafe { unsafe_cast(&self.buffer_ref.data[offset]) }
    }

    pub fn set(&mut self, index: usize) -> &mut T {
        let min = self.stride * index;
        let max = self.stride * (index + 1);
        self.delta_min = 0;
        self.delta_max = self.buffer_ref.data.len();
        unsafe { unsafe_cast_mut(&mut self.buffer_ref.data[min]) }
    }

    pub fn len(&self) -> usize {
        self.buffer_ref.data.len() / self.stride
    }
}

impl<'a, T> Drop for BufferView<'a, T> {
    fn drop(&mut self) {
        let old_range_opt = self.buffer_ref.delta_range.clone();
        if let Some((old_min, old_max)) = old_range_opt {
            self.buffer_ref.delta_range = Some((old_min.min(self.delta_min), old_max.max(self.delta_max)));
        } else {
            self.buffer_ref.delta_range = Some((self.delta_min, self.delta_max));
        }
    }
}
