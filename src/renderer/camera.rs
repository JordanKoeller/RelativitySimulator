use cgmath::prelude::*;
use utils::*;
use renderer::LIGHT_SPEED;


pub trait Camera {
  fn front(&self) -> Vec3F;
  fn set_front(&mut self, v: Vec3F);
  fn up(&self) -> Vec3F;
  fn set_up(&mut self, v: Vec3F);
  fn right(&self) -> Vec3F;
  fn set_right(&mut self, v: Vec3F);
  fn yaw(&self) -> &f32;
  fn set_yaw(&mut self, v: f32);
  fn pitch(&self) -> &f32;
  fn set_pitch(&mut self, v: f32);
  fn zoom(&self) -> &f32;
  fn position(&self) -> &Vec3F;
  fn velocity(&self) -> &Vec3F;

  fn world_up(&self) -> Vec3F {
    Vec3F::unit_y()
  }

  fn view_matrix(&self) -> Mat4F {
    let pos = self.position();
    let pt_pos = cgmath::Point3::<f32>::new(pos.x, pos.y, pos.z);
    Mat4F::look_at(pt_pos, pt_pos + self.front(), self.up())
  }

  fn projection_matrix(&self, dims: &Vec2F) -> Mat4F {
    cgmath::perspective(cgmath::Deg(*self.zoom()), dims.x / dims.y, 0.1, 10000.0)
  }

  fn rotate(&mut self, xoffset: f32, yoffset: f32) {
    // println!("Rotating");
    self.set_yaw(self.yaw() + xoffset);
    self.set_pitch(self.pitch() + yoffset);
    // Make sure that when pitch is out of bounds, screen doesn't get flipped
    if *self.pitch() > 89.0 {
      self.set_pitch(89.0);
    }
    if *self.pitch() < -89.0 {
      self.set_pitch(-89.0);
    }
    // Update Front, Right and Up Vectors using the updated Euler angles
    self.update_camera_vectors();
  }

  fn update_camera_vectors(&mut self) {
    // Calculate the new Front vector
    let front = Vec3F {
      x: self.yaw().to_radians().cos() * self.pitch().to_radians().cos(),
      y: self.pitch().to_radians().sin(),
      z: self.yaw().to_radians().sin() * self.pitch().to_radians().cos(),
    };
    self.set_front(front.normalize());
    // Also re-calculate the Right and Up vector
    self.set_right(self.front().cross(self.world_up()).normalize()); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
    self.set_up(self.right().cross(self.front()).normalize());
  }

  fn beta(&self) -> f32 {
    self.velocity().magnitude() / LIGHT_SPEED
  }

  fn beta2(&self) -> f32 {
    self.velocity().magnitude2() / LIGHT_SPEED / LIGHT_SPEED
  }

  fn gamma(&self) -> f32 {
    (1.0 - self.beta2()).powf(-0.5)
  }

  fn velocity_basis_matrix(&self) -> Mat3F {
    if self.beta() == 0.0 {
      cgmath::Matrix3::<f32>::identity()
    } else {
      let vel_norm = self.velocity().normalize();
      let right = vel_norm.cross(self.world_up()).normalize();
      let up = right.cross(vel_norm);
      cgmath::Matrix3::<f32>::from_cols(vel_norm, right, up).transpose()
    }
  }

  fn velocity_inverse_basis_matrix(&self) -> Mat3F {
    let ret: cgmath::Matrix3<f32> = self.velocity_basis_matrix().invert().expect("Could not invert matrix");
    ret
  }

}