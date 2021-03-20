use std::ops::Deref;
use cgmath::Rad;
use utils::{
  types::{Mat4F, Vec3F},
  math::*
};

use ecs::components::Transform;

pub struct TransformStack {
  stack: Vec<Transform>,
}


// Basic functions (push, pop, etc.)
impl TransformStack {
  pub fn push(&mut self, matrix: &Transform) -> &Transform {
    self.stack.push(Transform(matrix.deref() * self.peek().deref()));
    self.peek()
  }

  pub fn pop(&mut self) -> Transform {
    if self.stack.len() < 2 {
      Transform(identity())
    } else {
      self.stack.pop().unwrap()
    }
  }

  pub fn peek(&self) -> &Transform {

    &self.stack[self.stack.len() - 1]
  }
}


// Higher-level api with things like pushing scale, translate, etc.
impl TransformStack {
  pub fn push_translate(&mut self, pos: Vec3F) -> &Mat4F {
    self.push(&Transform(translate(pos)))
  }

  pub fn push_scale(&mut self, s: f32) -> &Mat4F {
    self.push(&Transform(scale(s)))
  }

  pub fn push_nonunif_scale(&mut self, f: Vec3F) -> &Mat4F {
    self.push(&Transform(nonunif_scale(f)))
  }

  pub fn push_euler<A: Into<Rad<f32>>>(&mut self, angle: A, axis: Vec3F) -> &Mat4F {
    let matrix = Mat4F::from_axis_angle(axis, angle);
    self.push(&Transform(matrix))
  }

  pub fn reset_to(&mut self, matrix: &Transform) -> &Transform {
    self.clear();
    self.push(matrix)
  }

  pub fn clear(&mut self) {
    self.stack.clear();
    self.stack.push(Transform(identity()));
  }
}


impl Default for TransformStack {
  fn default() -> Self {
    let mut stack = Vec::new();
    stack.push(Transform(identity()));
    Self {
      stack,
    }
  }
}