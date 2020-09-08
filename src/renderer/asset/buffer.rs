use gl;
use std::mem::size_of;
use std::os::raw::c_void;

////////////////////
/// BUFFER OBJECTS
////////////////////

#[derive(Clone, Eq, PartialEq)]
pub struct VertexBuffer {
    id: u32,
    pub layout: BufferLayout,
}

#[derive(Clone, Eq, PartialEq)]
pub struct IndexBuffer {
    id: u32,
}

impl VertexBuffer {
    pub fn create(data: Vec<f32>, layout: BufferLayout) -> VertexBuffer {
        let mut ret = VertexBuffer { id: 0, layout };
        unsafe {
            gl::GenVertexArrays(1, &mut ret.id);
            ret.bind();
            gl::BufferData(gl::ARRAY_BUFFER, buff_sz(&data), buff_ptr(&data), gl::STATIC_DRAW);
        }

        ret
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}


impl Drop for VertexBuffer {
    fn drop(&mut self) {
        self.unbind();
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}

impl IndexBuffer {
    pub fn create(data: Vec<u32>) -> Self {
        let mut ret = Self { id: 0 };
        unsafe {
            gl::GenVertexArrays(1, &mut ret.id);
            ret.bind();
            gl::BufferData(gl::ARRAY_BUFFER, buff_sz(&data), buff_ptr(&data), gl::STATIC_DRAW);
        }
        ret
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        self.unbind();
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}

//////////////////////////////////
/// BUFFER LAYOUT ////////////////
//////////////////////////////////
#[derive(Clone, Eq, PartialEq, Copy)]
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

#[derive(Clone, Eq, PartialEq)]
pub struct BufferLayout(Vec<AttributeType>);

impl BufferLayout {
    pub fn stride(&self) -> u32 {
        self.0.iter().map(|x| x.width()).sum()
    }

    pub fn ind_offset_attrib(&self) -> Vec<(usize, u32, AttributeType)> {
        let mut summation = 0;
        self.0
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let v = summation.clone();
                summation += x.width();
                (i, v, x)
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
