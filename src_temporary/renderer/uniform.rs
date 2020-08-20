use utils::*;

#[derive(Debug)]
#[derive(Clone)]
pub enum UniformValue {
    Float(f32),
    Bool(bool),
    UInt(u32),
    Int(i32),
    Vec2(Vec2F),
    Vec3(Vec3F),
    Mat2(Mat2F),
    Mat3(Mat3F),
    Mat4(Mat4F),
}

#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub enum UniformType {
    Float,
    Bool,
    UInt,
    Int,
    Vec2,
    Vec3,
    Mat2,
    Mat3,
    Mat4
}

pub struct UniformManager {
    uniforms: std::collections::HashMap<String, UniformValue>
}

impl UniformManager {
    pub fn new() -> UniformManager {
        UniformManager {
            uniforms: std::collections::HashMap::new()
        }
    }

    pub fn set(&mut self, name: &str, value: UniformValue) {
        self.uniforms.insert(name.to_string(), value);
    }

    pub fn get_all(&self) -> impl Iterator<Item = (&String, &UniformValue)> {
        self.uniforms.iter()
    }

    pub fn get(&self, name: &str) -> Option<&UniformValue> {
        self.uniforms.get(name)
    }


}