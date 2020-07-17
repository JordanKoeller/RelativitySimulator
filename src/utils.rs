use std::time::SystemTime;
use cgmath::{Vector2, Vector3};

pub type Vec2F = Vector2::<f32>;
pub type Vec3F = Vector3::<f32>;

#[allow(dead_code)]
pub fn elapsed(start_time: &SystemTime) -> String {
    let elapsed = start_time.elapsed().unwrap();
    format!("{}s {:.*}ms", elapsed.as_secs(), 1, elapsed.subsec_nanos() as f64 / 1_000_000.0)
}


pub struct Rectangle {
    tl: Vec2F,
    br: Vec2F,
}



impl Rectangle {
    pub fn area(&self) -> f32 {
        let da = self.dims();
        (da.x * da.y).abs()
    }

    pub fn dims(&self) -> Vec2F {
        let da = self.br - self.tl;
        Vec2F::new(da.x.abs(), da.y.abs())
    }
}