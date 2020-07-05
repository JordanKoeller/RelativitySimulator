use cgmath;
use cgmath::prelude::*;
use cgmath::Vector3;

type Point3 = cgmath::Point3<f32>;
type Vec3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

pub trait Camera {
  fn position(&self) -> Point3;
  fn front(&self) -> Vec3;
  fn set_front(&mut self, v: Vec3);
  fn up(&self) -> Vec3;
  fn set_up(&mut self, v: Vec3);
  fn right(&self) -> Vec3;
  fn set_right(&mut self, v: Vec3);
  fn world_up(&self) -> Vec3;
  fn yaw(&self) -> f32;
  fn set_yaw(&mut self, v: f32);
  fn pitch(&self) -> f32;
  fn set_pitch(&mut self, v: f32);
  fn camera_speed(&self) -> f32;
  fn mouse_sensitivity(&self) -> f32;
  fn zoom(&self) -> f32;

  fn vec_pos(&self) -> Vec3 {
    Vec3{x: self.position().x, y: self.position().y, z: self.position().z}
  }

  fn get_view_matrix(&self) -> Matrix4 {
    Matrix4::look_at(self.position(), self.position() + self.front(), self.up())
  }

  fn rotate(&mut self, xoffset: f32, yoffset: f32) {
    self.set_yaw(self.yaw() + xoffset);
    self.set_pitch(self.pitch() + yoffset);

    // Make sure that when pitch is out of bounds, screen doesn't get flipped
    if self.pitch() > 89.0 {
      self.set_pitch(89.0);
    }
    if self.pitch() < -89.0 {
      self.set_pitch(-89.0);
    }

    // Update Front, Right and Up Vectors using the updated Eular angles
    self.update_camera_vectors();
  }

  fn update_camera_vectors(&mut self) {
    // Calculate the new Front vector
    let front = Vector3 {
      x: self.yaw().to_radians().cos() * self.pitch().to_radians().cos(),
      y: self.pitch().to_radians().sin(),
      z: self.yaw().to_radians().sin() * self.pitch().to_radians().cos(),
    };
    self.set_front(front.normalize());
    // Also re-calculate the Right and Up vector
    self.set_right(self.front().cross(self.world_up()).normalize()); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
    self.set_up(self.right().cross(self.front()).normalize());
  }



}
