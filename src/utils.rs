use std::time::SystemTime;
use cgmath;
use cgmath::prelude::*;


#[allow(dead_code)]
pub fn elapsed(start_time: &SystemTime) -> String {
    let elapsed = start_time.elapsed().unwrap();
    format!("{}s {:.*}ms", elapsed.as_secs(), 1, elapsed.subsec_nanos() as f64 / 1_000_000.0)
}



pub type Vec2F = cgmath::Vector2::<f32>;
pub type Vec3F = cgmath::Vector3::<f32>;
pub type Vec4F = cgmath::Vector4::<f32>;
pub type Mat4F = cgmath::Matrix4::<f32>;
pub type Mat3F = cgmath::Matrix3::<f32>;
pub type Mat2F = cgmath::Matrix2::<f32>;
pub type Color = Vec3F;

pub fn translate(pos: Vec3F) -> Mat4F {
    Mat4F::from_translation(pos)
}


// Observable Pattern aspects

// pub struct Observer<'a, T> {

// }

// impl<'a, T> Observer<'a, T> {
//     pub fn subscribe(&mut self, observable: &mut Observable<T>) {

//     }
// }

// pub struct Observable<'a, T> {
//     value: T,
//     observers: Vec<&'a mut Observer<'a, T>>
// }

// impl<'a, T> Observable<'a, T> {
//     pub fn add_subscriber(&mut self, observer:&'a mut Observer<'a, T>) {
//         self.observers.push(observer);
//     }
// }