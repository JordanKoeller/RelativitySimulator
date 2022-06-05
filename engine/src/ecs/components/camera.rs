use std::ops::Deref;

use cgmath::prelude::*;
use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

use crate::utils::*;

const DEG_89: cgmath::Rad<f64> = cgmath::Rad(1.5533430342749532f64);

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Camera {
    position: Vec3F,
    euler_angles: cgmath::Euler<cgmath::Rad<f64>>,
    fovy: cgmath::Rad<f64>,
    aspect_ratio: f64,
    near_distance: f64,
    far_distance: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3F::new(0f64, 0f64, 0f64),
            euler_angles: cgmath::Euler::new(cgmath::Rad(0f64), cgmath::Rad(0f64), cgmath::Rad(0f64)),
            fovy: cgmath::Deg(45f64).into(),
            aspect_ratio: 16f64 / 9f64,
            near_distance: 0.1f64,
            far_distance: 1000f64,
        }
    }
}

impl Camera {

    pub fn new(position: Vec3F, facing: Vec3F) -> Self {
        let facing = facing.normalize();
        let euler_angles = cgmath::Euler::new(
            cgmath::Rad(facing.y.sin()),
            cgmath::Rad(facing.z.atan2(facing.x)),
            cgmath::Rad(0f64),
        );
        let mut ret = Self::default();
        ret.position = position;
        ret.euler_angles = euler_angles;
        ret
    }

    // Camera matrices
    pub fn projection_matrix(&self) -> Mat4F {
        cgmath::perspective(self.fovy, self.aspect_ratio, self.near_distance, self.far_distance)
    }

    pub fn view_matrix(&self) -> Mat4F {
        let facing = self.front();
        let location = cgmath::Point3::<f64>::new(self.position.x, self.position.y, self.position.z);
        Mat4F::look_at_dir(location, facing, Vec3F::unit_y())
    }

    // Getters and Setters
    pub fn position(&self) -> Vec3F {
        self.position
    }

    pub fn push_translation(&mut self, delta: Vec3F) {
        self.position += delta;
    }

    pub fn push_rotation(&mut self, delta: cgmath::Euler<cgmath::Rad<f64>>) {
        self.euler_angles = cgmath::Euler::new(
            self.euler_angles.x + delta.x,
            self.euler_angles.y + delta.y,
            self.euler_angles.z + delta.z,
        );
        if self.euler_angles.x > DEG_89 {
            self.euler_angles.x = DEG_89;
        }
        if self.euler_angles.x < -DEG_89 {
            self.euler_angles.x = -DEG_89;
        }
    }

    pub fn front(&self) -> Vec3F {
        Vec3F::new(
            self.euler_angles.y.cos() * self.euler_angles.x.cos(),
            self.euler_angles.x.sin(),
            self.euler_angles.y.sin() * self.euler_angles.x.cos(),
        )
        .normalize()
    }

    pub fn right(&self) -> Vec3F {
        self.front().cross(Vec3F::unit_y()).normalize()
    }

    pub fn up(&self) -> Vec3F {
        self.right().cross(self.front()).normalize()
    }
}
