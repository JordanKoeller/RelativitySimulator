use cgmath;
use cgmath::One;
// use cgmath::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

pub type Vec2F = cgmath::Vector2<f64>;
pub type Vec2I = cgmath::Vector2<i32>;
pub type Vec3F = cgmath::Vector3<f64>;
pub type Vec4F = cgmath::Vector4<f64>;
pub type Mat4F = cgmath::Matrix4<f64>;
pub type Mat3F = cgmath::Matrix3<f64>;
pub type QuatF = cgmath::Quaternion<f64>;
pub type DegF = cgmath::Deg<f64>;
pub type Mat2F = cgmath::Matrix2<f64>;
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
