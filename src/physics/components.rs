use cgmath::prelude::*;
use cgmath::{Deg, Rad, Rotation3};
use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};
use utils::*;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct TransformComponent {
    pub translation: Vec3F,
    pub scale: Vec3F,
    pub rotation: Vec3F,
}

impl TransformComponent {
    pub fn new(translation: Vec3F, scale: Vec3F, _rotation: QuatF) -> Self {
        Self {
            translation,
            scale,
            rotation: Vec3F::zero(),
        }
    }

    pub fn identity() -> Self {
        Self {
            translation: Vec3F::zero(),
            scale: Vec3F::new(1f32, 1f32, 1f32),
            rotation: Vec3F::zero(),
        }
    }

    pub fn matrix(&self) -> Mat4F {
        Mat4F::from_translation(self.translation) * nonunif_scale(self.scale)
    }

    pub fn push_translation(&mut self, dr: Vec3F) {
        self.translation = dr + self.translation;
    }

    pub fn push_scale(&mut self, ds: Vec3F) {
        self.scale = Vec3F::new(self.scale.x * ds.x, self.scale.y * ds.y, self.scale.z * ds.z);
    }

    pub fn front(&self) -> Vec3F {
        Vec3F {
            x: self.rotation.y.to_radians().cos() * self.rotation.x.to_radians().cos(),
            y: self.rotation.x.to_radians().sin(),
            z: self.rotation.y.to_radians().sin() * self.rotation.x.to_radians().cos(),
        }
        .normalize()
    }

    pub fn right(&self) -> Vec3F {
        self.front().cross(self.world_up()).normalize()
    }

    pub fn up(&self) -> Vec3F {
        self.right().cross(self.front()).normalize()
    }

    pub fn world_up(&self) -> Vec3F {
        Vec3F::unit_y()
    }

    pub fn rotate(&mut self, xoffset: f32, yoffset: f32) {
        // println!("Rotating");
        self.rotation.y += xoffset;
        self.rotation.x += yoffset;
        // Make sure that when pitch is out of bounds, screen doesn't get flipped
        if self.rotation.x > 89.0 {
            self.rotation.x = 89f32;
        }
        if self.rotation.x < -89.0 {
            self.rotation.x = -89f32;
        }
        // Update Front, Right and Up Vectors using the updated Euler angles
    }
}

impl Default for TransformComponent {
    fn default() -> Self {
        Self::new(Vec3F::zero(), Vec3F::new(1f32, 1f32, 1f32), QuatF::zero())
    }
}

impl From<Mat4F> for TransformComponent {
    fn from(v: Mat4F) -> Self {
        let translation = Vec3F::new(v.w[0], v.w[1], v.w[2]);
        let scale = Vec3F::new(
            swizzle_down(&v.x).magnitude(),
            swizzle_down(&v.y).magnitude(),
            swizzle_down(&v.z).magnitude(),
        );
        let mut rotation_matrix = Mat3F::from_cols(swizzle_down(&v.x), swizzle_down(&v.y), swizzle_down(&v.z));
        rotation_matrix.x = rotation_matrix.x / scale.x;
        rotation_matrix.y = rotation_matrix.y / scale.y;
        rotation_matrix.z = rotation_matrix.z / scale.z;
        let rotation = QuatF::from(rotation_matrix);
        Self::new(translation, scale, rotation)
    }
}

#[derive(Component, Debug, Clone, Default)]
#[storage(NullStorage)]
pub struct Drag;

#[derive(Component, Debug, Clone, Default)]
#[storage(NullStorage)]
pub struct Gravity;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct RigidBody {
    pub velocity: Vec3F,
    pub acceleration: Vec3F,
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            velocity: Vec3F::zero(),
            acceleration: Vec3F::zero(),
        }
    }
}

impl RigidBody {
    pub fn new_stationary() -> Self {
        Self {
            velocity: Vec3F::zero(),
            acceleration: Vec3F::zero(),
        }
    }

    pub fn new(velocity: Vec3F, acceleration: Vec3F) -> Self {
        Self { velocity, acceleration }
    }
}
