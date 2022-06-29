use gl;
use std::mem::size_of;
use std::os::raw::c_void;
use std::slice::SliceIndex;

//////////////////
/// Some pre-defined Buffer types
/////////////////

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

    pub fn num_calls(&self) -> u32 {
        match self {
            AttributeType::Mat4 => 4,
            _ => 1,
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
        let mut i = 0;
        self.0
            .iter()
            .flat_map(|&attribute| {
                let v: Vec<(usize, u32, AttributeType)> = (0..attribute.num_calls())
                    .into_iter()
                    .map(|_| {
                        let v = summation.clone();
                        summation += attribute.width() / attribute.num_calls();
                        let ret = (i, v * size_of::<f32>() as u32, attribute);
                        i += 1;
                        ret
                    })
                    .collect();
                v
            })
            .collect()
    }
}

impl From<&Vec<(String, AttributeType)>> for BufferLayout {
    fn from(atts: &Vec<(String, AttributeType)>) -> Self {
        Self(atts.iter().map(|x| x.1).collect())
    }
}

pub trait Bufferable {
    fn into_buffer(&self, buffer: &mut [f32]);

    fn from_slice(buffer: &[f32]) -> Self;

    fn num_elems() -> usize;
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
