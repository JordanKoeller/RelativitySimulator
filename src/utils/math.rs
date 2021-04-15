use cgmath;
use cgmath::One;

use super::types::*;

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

pub fn swizzle_down(v: &Vec4F) -> Vec3F {
  Vec3F::new(v.x, v.y, v.z)
}