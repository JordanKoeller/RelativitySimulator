use cgmath;
use cgmath::One;
// use cgmath::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;


pub type Vec2F = cgmath::Vector2<f32>;
pub type Vec2I = cgmath::Vector2<i32>;
pub type Vec3I = cgmath::Vector3<usize>;
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

pub type SyncMutRef<T> = Arc<Mutex<T>>;

#[derive(Debug, Default)]
pub struct Timestep {
  pub last_time: Duration,
  pub click: Duration,
  pub render_time: Duration,
}

impl Timestep {
  pub fn click_frame(&mut self, timestamp: Duration) {
    self.click = timestamp - self.last_time;
    self.last_time = timestamp;
  }

  pub fn set_render_time(&mut self, value: Duration) {
    self.render_time = value;
  }

  pub fn dt(&self) -> Duration {
    self.click
  }

  pub fn render_dt(&self) -> Duration {
    self.render_time
  }

  pub fn curr_time(&self) -> Duration {
    self.last_time
  }
}
// pub struct Timestep(pub f32, pub f32);
// impl Timestep {
//   pub fn set_click(&mut self, value: f32) {
//     self.0 = value;
//   }

//   pub fn set_render_time(&mut self, value: f32) {
//     self.1 = value;
//   }
// }

#[allow(dead_code, non_snake_case)]
pub fn GetMutRef<T>(v: T) -> MutRef<T> {
  Rc::new(RefCell::new(v))
}

#[allow(dead_code, non_snake_case)]
pub fn getSyncMutRef<T>(v: T) -> SyncMutRef<T> {
  Arc::new(Mutex::new(v))
}