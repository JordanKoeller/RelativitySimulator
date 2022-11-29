use cgmath;
use cgmath::One;
// use cgmath::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use std::ops::{Deref, DerefMut};

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

pub type SyncMutRef<T> = Arc<Mutex<T>>;

#[allow(dead_code, non_snake_case)]
pub fn GetMutRef<T>(v: T) -> MutRef<T> {
  Rc::new(RefCell::new(v))
}

#[allow(dead_code, non_snake_case)]
pub fn getSyncMutRef<T>(v: T) -> SyncMutRef<T> {
  Arc::new(Mutex::new(v))
}


pub struct Swap<T: Sized> {
  value: Option<T>
}

impl<T: Sized> Swap<T> {
  pub fn new(value: T) -> Self {
    Self {
      value: Some(value),
    }
  }

  pub fn swap_with<F: FnOnce(T) -> T>(&mut self, func: F) {
    let value = self.value.take().unwrap();
    let new_value = func(value);
    self.value = Some(new_value);
  }

  pub fn unwrap(self) -> T {
    self.value.unwrap()
  }
}

impl<T: Sized> std::ops::Deref for Swap<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
      self.value.as_ref().unwrap()
  }
}

impl<T: Sized> std::ops::DerefMut for Swap<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
      self.value.as_mut().unwrap()
  }
}

