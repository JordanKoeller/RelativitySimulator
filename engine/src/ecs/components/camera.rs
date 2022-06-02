use std::ops::Deref;

use cgmath::prelude::*;
use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

use crate::utils::*;

use crate::renderer::LIGHT_SPEED;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Camera {
    perspective: Mat4F,
    position: Vec3F,
    velocity: Vec3F,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            perspective: Mat4F::identity(),
            velocity: Vec3F::new(0f64, 0f64, 0f64),
            position: Vec3F::new(0f64, 0f64, 0f64),
        }
    }
}

impl Camera {
    pub fn new(pers: Mat4F, position: Vec3F, vel: Vec3F) -> Self {
        Self {
            perspective: pers,
            velocity: vel,
            position,
        }
    }
    pub fn projection_matrix(&self, dims: &Vec2F) -> Mat4F {
        cgmath::perspective(cgmath::Deg(45f64), dims.x / dims.y, 0.1, 10000.0)
    }

    pub fn view_matrix(&self) -> &Mat4F {
        &self.perspective
    }

    pub fn beta(&self) -> f64 {
        self.velocity.magnitude() / LIGHT_SPEED
    }

    pub fn beta2(&self) -> f64 {
        self.velocity.magnitude2() / LIGHT_SPEED / LIGHT_SPEED
    }

    pub fn gamma(&self) -> f64 {
        (1.0 - self.beta2()).powf(-0.5)
    }

    pub fn velocity_basis_matrix(&self) -> Mat3F {
        if self.beta() == 0.0 {
            cgmath::Matrix3::<f64>::identity()
        } else {
            let vel_norm = self.velocity.normalize();
            let right = vel_norm.cross(Vec3F::unit_y()).normalize();
            let up = right.cross(vel_norm);
            cgmath::Matrix3::<f64>::from_cols(vel_norm, right, up).transpose()
        }
    }

    pub fn velocity_inverse_basis_matrix(&self) -> Mat3F {
        self.velocity_basis_matrix().invert().expect("Could not invert matrix")
    }

    pub fn set_matrix(&mut self, mat: Mat4F) {
        self.perspective = mat;
    }

    pub fn position(&self) -> Vec3F {
        self.position
    }

    pub fn set_position(&mut self, pos: Vec3F) {
        self.position = pos;
    }
}
