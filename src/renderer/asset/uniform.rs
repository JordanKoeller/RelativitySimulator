use utils::*;

pub enum Uniform {
    Int(i32),
    IntArray(Vec<i32>),
    Float(f32),
    Vec2(Vec2F),
    Vec3(Vec3F),
    Vec4(Vec4F),
    Mat3(Mat3F),
    Mat4(Mat4F),
    Bool(bool)
}