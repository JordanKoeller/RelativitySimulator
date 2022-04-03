use std::ops::Deref;

use cgmath::prelude::*;
use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

use utils::*;

use renderer::LIGHT_SPEED;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Camera {
    perspective: Mat4F,
    velocity: Vec3F,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            perspective: Mat4F::identity(),
            velocity: Vec3F::new(0f32, 0f32, 0f32),
        }
    }
}

impl Camera {
    pub fn new(pers: Mat4F, vel: Vec3F) -> Self {
        Self {
            perspective: pers,
            velocity: vel,
        }
    }
    pub fn projection_matrix(&self, dims: &Vec2F) -> Mat4F {
        cgmath::perspective(cgmath::Deg(45f32), dims.x / dims.y, 0.1, 10000.0)
    }

    pub fn view_matrix(&self) -> &Mat4F {
        &self.perspective
    }

    pub fn beta(&self) -> f32 {
        self.velocity.magnitude() / LIGHT_SPEED
    }

    pub fn beta2(&self) -> f32 {
        self.velocity.magnitude2() / LIGHT_SPEED / LIGHT_SPEED
    }

    pub fn gamma(&self) -> f32 {
        (1.0 - self.beta2()).powf(-0.5)
    }

    pub fn velocity_basis_matrix(&self) -> Mat3F {
        if self.beta() == 0.0 {
            cgmath::Matrix3::<f32>::identity()
        } else {
            let vel_norm = self.velocity.normalize();
            let right = vel_norm.cross(Vec3F::unit_y()).normalize();
            let up = right.cross(vel_norm);
            cgmath::Matrix3::<f32>::from_cols(vel_norm, right, up).transpose()
        }
    }

    pub fn velocity_inverse_basis_matrix(&self) -> Mat3F {
        self.velocity_basis_matrix().invert().expect("Could not invert matrix")
    }

    pub fn set_matrix(&mut self, mat: Mat4F) {
        self.perspective = mat;
    }
}
