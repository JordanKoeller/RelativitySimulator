// use cgmath::prelude::*;

// use mechanics::{Updatable, PlayerMotionDelegate};
// use renderer::Camera;
// use renderer::{UniformManager, UniformValue};

// use utils::*;

// pub struct Player {
//   position: Vec3F,
//   // pub velocity: Vec3F,
//   // pub acceleration: Vec3F,
//   // Camera vectors
//   front: Vec3F,
//   up: Vec3F,
//   right: Vec3F,
//   // world_up: Vec3F,
//   // Euler Angles
//   yaw: f32,
//   pitch: f32,
//   // Camera options
//   zoom: f32,
//   // Uniform Manager
//   uniforms: UniformManager,
//   pub motion: PlayerMotionDelegate
//   // pub width: f32,
//   // pub height: f32,
// }

// impl Player {
//   pub fn set_position(&mut self, pos: Vec3F) {
//     self.position = pos;
//     let p = UniformValue::Vec3(pos);
//     self.uniform_manager_mut().set("camera_pos", p);
//   }

//   pub fn new(pos: Vec3F, front: Vec3F) -> Player {
//     let mut ret = Player::default();
//     ret.set_position(pos);
//     ret.set_front(front);
//     ret
//   }

//   pub fn apply_user_shift(&mut self, shift: Vec3F) {
//     self.set_position(self.position() + shift);
//   }
// }

// impl Default for Player {
//   fn default() -> Player {
//     let mut cam = Player {
//       position: Vec3F::new(0.0, 0.0, 0.0),
//       // velocity: Vec3F::zero(),
//       // acceleration: Vec3F::zero(),
//       front: Vec3F::new(0.0, 0.0, -1.0),
//       up: Vec3F::zero(),
//       right: Vec3F::zero(),
//       // world_up: Vec3F::unit_y(),
//       yaw: -90.0,
//       pitch: 0.0,
//       zoom: 45.0,
//       uniforms: UniformManager::new(),
//       motion: PlayerMotionDelegate::default()
//     };
//     cam.update_camera_vectors();
//     cam
//   }
// }

// impl Camera for Player {
//   fn position(&self) -> Vec3F {
//     self.position
//   }
//   fn front(&self) -> Vec3F {
//     self.front
//   }
//   fn set_front(&mut self, v: Vec3F) {
//     self.front = v;
//   }
//   fn up(&self) -> Vec3F {
//     self.up
//   }
//   fn set_up(&mut self, v: Vec3F) {
//     self.up = v;
//   }
//   fn right(&self) -> Vec3F {
//     self.right
//   }
//   fn set_right(&mut self, v: Vec3F) {
//     self.right = v;
//   }
//   fn world_up(&self) -> Vec3F {
//     Vec3F::unit_y()
//   }
//   fn yaw(&self) -> f32 {
//     self.yaw
//   }
//   fn set_yaw(&mut self, v: f32) {
//     self.yaw = v;
//   }
//   fn pitch(&self) -> f32 {
//     self.pitch
//   }
//   fn set_pitch(&mut self, v: f32) {
//     self.pitch = v;
//   }
//   fn zoom(&self) -> f32 {
//     self.zoom
//   }
//   fn uniform_manager(&self) -> &UniformManager {
//     &self.uniforms
//   }
//   fn uniform_manager_mut(&mut self) -> &mut UniformManager {
//     &mut self.uniforms
//   }


// }

// impl Updatable for Player {
//   fn update(&mut self, dt: f32) {
//     self.motion.update(dt);
//     let velocity = self.motion.velocity();
//     self.set_position(velocity*dt + self.position());
//     self.update_camera_vectors();
//   }
// }
