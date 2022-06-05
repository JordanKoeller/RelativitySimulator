use super::types::*;
use cgmath;
use cgmath::One;

pub fn translate(pos: Vec3F) -> Mat4F {
    Mat4F::from_translation(pos)
}

#[allow(dead_code)]
pub fn scale(factor: f64) -> Mat4F {
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
    Vec4F::new(v.x, v.y, v.z, 1f64)
}

pub fn swizzle_up_with(v: &Vec3F, value: f64) -> Vec4F {
    Vec4F::new(v.x, v.y, v.z, value)
}

pub fn swizzle_down(v: &Vec4F) -> Vec3F {
    Vec3F::new(v.x, v.y, v.z)
}

pub fn lerp(low1: f64, high1: f64, low2: f64, high2: f64, value: f64) -> f64 {
    let d1 = high1 - low1;
    let d2 = high2 - low2;
    low2 + d2 * (value - low1) / d1
}

pub fn avg(a: f64, b: f64) -> f64 {
    (a + b) / 2f64
}

pub struct Averager {
    sum: f64,
    count: u32,
    window_size: u32,
}

impl Averager {
    pub fn new(window_size: u32) -> Self {
        Self {
            sum: 0f64,
            count: 0u32,
            window_size,
        }
    }

    pub fn push_value(&mut self, value: f64) {
        if self.count == self.window_size {
            self.sum = value;
            self.count = 1;
        } else {
            self.sum += value;
            self.count += 1;
        }
    }
    pub fn get_avg(&self) -> f64 {
        if self.count == 0 {
            0f64
        } else {
            self.sum / (self.count as f64)
        }
    }

    pub fn reset(&mut self) {
        self.count = 0u32;
        self.sum = 0f64;
    }
}
