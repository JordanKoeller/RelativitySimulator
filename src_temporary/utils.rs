use std::time::SystemTime;
use cgmath;


#[allow(dead_code)]
pub fn elapsed(start_time: &SystemTime) -> String {
    let elapsed = start_time.elapsed().unwrap();
    format!("{}s {:.*}ms", elapsed.as_secs(), 1, elapsed.subsec_nanos() as f64 / 1_000_000.0)
}



pub type Vec2F = cgmath::Vector2::<f32>;
pub type Vec3F = cgmath::Vector3::<f32>;
pub type Mat4F = cgmath::Matrix4::<f32>;
pub type Mat3F = cgmath::Matrix3::<f32>;
pub type Mat2F = cgmath::Matrix2::<f32>;
