use utils::*;

#[derive(Debug, Clone, Copy)]
pub enum UniformValue {
    Float(f32),
    Bool(bool),
    Int(i32),
    Vec2(Vec2F),
    Vec3(Vec3F),
    Mat2(Mat2F),
    Mat3(Mat3F),
    Mat4(Mat4F),
}

impl UniformValue {
    pub fn to_type(&self) -> UniformType {
        match self {
            UniformValue::Float(_) => UniformType::Float,
            UniformValue::Bool(_) => UniformType::Bool,
            UniformValue::Int(_) => UniformType::Int,
            UniformValue::Vec2(_) => UniformType::Vec2,
            UniformValue::Vec3(_) => UniformType::Vec3,
            UniformValue::Mat2(_) => UniformType::Mat2,
            UniformValue::Mat3(_) => UniformType::Mat3,
            UniformValue::Mat4(_) => UniformType::Mat4,
          }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum UniformType {
    Float,
    Bool,
    Int,
    Vec2,
    Vec3,
    Mat2,
    Mat3,
    Mat4,
}


impl UniformType {
    pub fn from_value(v: &UniformValue) -> UniformType {
        v.to_type()
    }
}

#[derive(Clone)]
pub struct UniformManager {
    uniforms: std::collections::HashMap<String, UniformValue>,
}

impl UniformManager {
    pub fn new() -> UniformManager {
        UniformManager {
            uniforms: std::collections::HashMap::new(),
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