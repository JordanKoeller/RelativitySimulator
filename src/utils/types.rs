use cgmath;
use cgmath::One;
// use cgmath::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
// use std::time::SystemTime;

// #[allow(dead_code)]
// pub fn elapsed(start_time: &SystemTime) -> String {
//   let elapsed = start_time.elapsed().unwrap();
//   format!(
//     "{}s {:.*}ms",
//     elapsed.as_secs(),
//     1,
//     elapsed.subsec_nanos() as f64 / 1_000_000.0
//   )
// }

pub type Vec2F = cgmath::Vector2<f32>;
pub type Vec2I = cgmath::Vector2<i32>;
pub type Vec3F = cgmath::Vector3<f32>;
pub type Vec4F = cgmath::Vector4<f32>;
pub type Mat4F = cgmath::Matrix4<f32>;
pub type Mat3F = cgmath::Matrix3<f32>;
pub type QuatF = cgmath::Quaternion<f32>;
pub type DegF = cgmath::Deg<f32>;
pub type Mat2F = cgmath::Matrix2<f32>;
pub type Color = Vec3F;

pub type Ref<T> = Rc<T>;
pub type MutRef<T> = Rc<RefCell<T>>;
pub type Mut<T> = RefCell<T>;

#[derive(Default)]
pub struct Timestep(pub f32, pub f32);
impl Timestep {
  pub fn set_click(&mut self, value: f32) {
    self.0 = value;
  }

  pub fn set_render_time(&mut self, value: f32) {
    self.1 = value;
  }
}

#[derive(Default)]
pub struct Running(pub bool);
impl Running {
  pub fn set_value(&mut self, value: bool) {
    self.0 = value;
  }
}


#[allow(dead_code, non_snake_case)]
pub fn GetMutRef<T>(v: T) -> MutRef<T> {
  Rc::new(RefCell::new(v))
}