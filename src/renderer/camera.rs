use cgmath::prelude::*;
use utils::*;

use renderer::uniform::{UniformManager, UniformValue};

type Point3 = cgmath::Point3<f32>;
// type Vec3F = cgmath::Vector3<f32>;
// type Mat4F = cgmath::Mat4F<f32>;

pub trait Camera {
    fn position(&self) -> Vec3F;
    fn front(&self) -> Vec3F;
    fn set_front(&mut self, v: Vec3F);
    fn up(&self) -> Vec3F;
    fn set_up(&mut self, v: Vec3F);
    fn right(&self) -> Vec3F;
    fn set_right(&mut self, v: Vec3F);
    fn world_up(&self) -> Vec3F;
    fn yaw(&self) -> f32;
    fn set_yaw(&mut self, v: f32);
    fn pitch(&self) -> f32;
    fn set_pitch(&mut self, v: f32);
    fn zoom(&self) -> f32;
    fn uniform_manager(&self) -> &UniformManager;
    fn uniform_manager_mut(&mut self) -> &mut UniformManager;
    fn width(&self) -> f32 {
       crate::SCR_WIDTH as f32
    }
    fn height(&self) -> f32 {
        crate::SCR_HEIGHT as f32
    }
    fn pt_pos(&self) -> Point3 {
        Point3 {
            x: self.position().x,
            y: self.position().y,
            z: self.position().z,
        }
    }

    fn get_view_matrix(&self) -> Mat4F {
        Mat4F::look_at(self.pt_pos(), self.pt_pos() + self.front(), self.up())
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
        let front = Vec3F {
            x: self.yaw().to_radians().cos() * self.pitch().to_radians().cos(),
            y: self.pitch().to_radians().sin(),
            z: self.yaw().to_radians().sin() * self.pitch().to_radians().cos(),
        };
        self.set_front(front.normalize());
        // Also re-calculate the Right and Up vector
        self.set_right(self.front().cross(self.world_up()).normalize()); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
        self.set_up(self.right().cross(self.front()).normalize());
        let front = UniformValue::Vec3(self.front());
        let matrix = UniformValue::Mat4(self.get_view_matrix());
        self.uniform_manager_mut().set("camera_front", front);
        self.uniform_manager_mut().set("view", matrix);
        let projection: Mat4F = cgmath::perspective(cgmath::Deg(self.zoom()), self.width() / self.height(), 0.1, 10000.0);
        self.uniform_manager_mut().set("projection", UniformValue::Mat4(projection));

    }
}
