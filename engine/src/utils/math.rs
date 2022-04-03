use super::types::*;
use cgmath;
use cgmath::One;

pub fn translate(pos: Vec3F) -> Mat4F {
    Mat4F::from_translation(pos)
}

#[allow(dead_code)]
pub fn scale(factor: f32) -> Mat4F {
    Mat4F::from_scale(factor)
}

#[allow(dead_code)]
pub fn nonunif_scale(factor: Vec3F) -> Mat4F {
    Mat4F::from_nonuniform_scale(factor.x, factor.y, factor.z)
}

#[allow(dead_code)]
pub fn identity() -> Mat4F {
    Mat4F::one()
}

pub fn swizzle_up(v: &Vec3F) -> Vec4F {
    Vec4F::new(v.x, v.y, v.z, 1f32)
}

pub fn swizzle_up_with(v: &Vec3F, value: f32) -> Vec4F {
    Vec4F::new(v.x, v.y, v.z, value)
}

pub fn swizzle_down(v: &Vec4F) -> Vec3F {
    Vec3F::new(v.x, v.y, v.z)
}

pub fn lerp(low1: f32, high1: f32, low2: f32, high2: f32, value: f32) -> f32 {
    let d1 = high1 - low1;
    let d2 = high2 - low2;
    low2 + d2 * (value - low1) / d1
}

pub fn avg(a: f32, b: f32) -> f32 {
    (a + b) / 2f32
}
