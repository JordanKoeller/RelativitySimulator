use cgmath::Rad;
use std::ops::Deref;

use crate::utils::{
    math::*,
    types::{Mat4F, Vec3F},
};

use crate::physics::components::TransformComponent;

pub struct TransformStack {
    stack: Vec<Mat4F>,
}

// Basic functions (push, pop, etc.)
impl TransformStack {
    pub fn push(&mut self, matrix: &Mat4F) -> &Mat4F {
        self.stack.push(matrix.clone());
        self.peek()
    }

    pub fn pop(&mut self) -> Mat4F {
        if self.stack.len() < 2 {
            identity()
        } else {
            self.stack.pop().unwrap()
        }
    }

    pub fn peek(&self) -> &Mat4F {
        &self.stack[self.stack.len() - 1]
    }
}

// Higher-level api with things like pushing scale, translate, etc.
impl TransformStack {
    pub fn push_translate(&mut self, pos: Vec3F) -> &Mat4F {
        self.push(&translate(pos))
    }

    pub fn push_scale(&mut self, s: f32) -> &Mat4F {
        self.push(&scale(s))
    }

    pub fn push_nonunif_scale(&mut self, f: Vec3F) -> &Mat4F {
        self.push(&nonunif_scale(f))
    }

    pub fn push_euler<A: Into<Rad<f32>>>(&mut self, angle: A, axis: Vec3F) -> &Mat4F {
        let matrix = Mat4F::from_axis_angle(axis, angle);
        self.push(&matrix)
    }

    pub fn reset_to(&mut self, matrix: &Mat4F) -> &Mat4F {
        self.clear();
        self.push(matrix)
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.stack.push(identity());
    }
}

impl Default for TransformStack {
    fn default() -> Self {
        let mut stack = Vec::new();
        stack.push(identity());
        Self { stack }
    }
}
