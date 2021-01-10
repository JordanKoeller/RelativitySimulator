use utils::*;

use renderer::{Texture, CubeMap};


#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Uniform {
    Int(i32),
    IntArray(Vec<i32>),
    Float(f32),
    Vec2(Vec2F),
    Vec3(Vec3F),
    Vec4(Vec4F),
    Mat3(Mat3F),
    Mat4(Mat4F),
    Bool(bool),
    Texture(Texture),
    CubeMap(CubeMap),
    UniformBuffer(UniformBuffer),
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum UniformType {
    Int,
    IntArray,
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Mat4,
    Bool,
    Texture,
    UniformBuffer
}

#[allow(dead_code)]
pub enum UniformLifecycle {
  Frame,
  Runtime,

}

#[derive(Clone, Debug)]
pub struct UniformBuffer {

}

impl UniformBuffer {

}