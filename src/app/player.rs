use super::super::glfw::{Action, Key};

use cgmath::prelude::*;
use utils::*;

use events::*;
use renderer::Camera;

use super::PlayerMotionDelegate;

pub struct Player {
  position: Vec3F,

  // Camera vectors
  front: Vec3F,
  up: Vec3F,
  right: Vec3F,

  // Euler Angles
  yaw: f32,
  pitch: f32,

  // Camera options
  zoom: f32,

  // Uniform Manager
  motion: PlayerMotionDelegate,

  receiver_state: EventReceiverState<Self>,
}

impl Player {
  pub fn set_position(&mut self, pos: Vec3F) {
    self.position = pos;
  }

  pub fn new(pos: Vec3F, front: Vec3F, dispatcher: MutRef<dyn EventDispatcher>) -> Player {
    let mut ret = Player::default(dispatcher);
    ret.set_position(pos);
    ret.set_front(front);
    ret
  }

  pub fn apply_user_shift(&mut self, shift: Vec3F) {
    self.set_position(self.position() + shift);
  }
}

impl Player {
  pub fn default(dispatcher: MutRef<dyn EventDispatcher>) -> Player {
    let motion = PlayerMotionDelegate::default();
    let receiver = EventReceiverState::new(dispatcher, 0);
    let mut player = Player {
      position: Vec3F::new(0.0, 0.0, 0.0),
      // velocity: Vec3F::zero(),
      // acceleration: Vec3F::zero(),
      front: Vec3F::new(0.0, 0.0, -1.0),
      up: Vec3F::zero(),
      right: Vec3F::zero(),
      // world_up: Vec3F::unit_y(),
      yaw: -90.0,
      pitch: 0.0,
      zoom: 45.0,
      motion,
      receiver_state: receiver,
    };
    player.update_camera_vectors();
    player.set_events();
    player
  }
}

impl Camera for Player {
  fn position(&self) -> &Vec3F {
    &self.position
  }
  fn front(&self) -> Vec3F {
    self.front
  }
  fn set_front(&mut self, v: Vec3F) {
    self.front = v;
  }
  fn up(&self) -> Vec3F {
    self.up
  }
  fn set_up(&mut self, v: Vec3F) {
    self.up = v;
  }
  fn right(&self) -> Vec3F {
    self.right
  }
  fn set_right(&mut self, v: Vec3F) {
    self.right = v;
  }
  fn yaw(&self) -> &f32 {
    &self.yaw
  }
  fn set_yaw(&mut self, v: f32) {
    self.yaw = v;
  }
  fn pitch(&self) -> &f32 {
    &self.pitch
  }
  fn set_pitch(&mut self, v: f32) {
    self.pitch = v;
  }
  fn zoom(&self) -> &f32 {
    &self.zoom
  }

  fn velocity(&self) -> &Vec3F {
    &self.motion.velocity
  }
}

// type PlayerEventHandler = (KeyDown, fn(&mut Player));

impl WithEventReceiver for Player {
  fn state(&self) -> &EventReceiverState<Self> {
    &self.receiver_state
  }
  fn state_mut(&mut self) -> &mut EventReceiverState<Self> {
    &mut self.receiver_state
  }
}

// EVENTS
impl Player {

  // impl Updatable for Player {
  pub fn update(&mut self, dt: f32) {
    self.process_all_events();
    self.motion.update(dt);
    let velocity = self.motion.velocity();
    self.set_position(velocity * dt + self.position());
    self.update_camera_vectors();
  }
}

impl Player {
  pub fn set_events(&mut self) {
    self.subscribe_to(Event::KeyPressed(KeyCode::W), |player, _| {
      let facing = player.front();
      player.motion.apply_user_acceleration(facing.normalize_to(1.0));
    });
    self.subscribe_to(Event::KeyPressed(KeyCode::S), |player, _| {
      let facing = player.front();
      player.motion.apply_user_acceleration(facing.normalize_to(-1.0));
    });
    self.subscribe_to(Event::KeyPressed(KeyCode::A), |player, _| {
      let facing = player.right();
      player.motion.apply_user_acceleration(facing.normalize_to(-1.0));
    });
    self.subscribe_to(Event::KeyPressed(KeyCode::D), |player, _| {
      let facing = player.right();
      player.motion.apply_user_acceleration(facing.normalize_to(1.0));
    });
    self.subscribe_to(Event::KeyPressed(KeyCode::F), |player, _| {
      player.motion.apply_brakes();
    });
    self.subscribe_to(Event::KeyPressed(KeyCode::LeftShift), |player, _| {
      player.apply_user_shift(-Vec3F::unit_y().normalize_to(0.05));
    });
    self.subscribe_to(Event::KeyPressed(KeyCode::Space), |player, _| {
      player.apply_user_shift(Vec3F::unit_y().normalize_to(0.05));
    });
    self.subscribe_to(Event::MouseMoved, |player, data| {
      if let (_, Some(EventPayload::MouseMove(mouse_move))) = data {
        player.rotate(mouse_move.x * 0.05, mouse_move.y * 0.05);
      } else {
        panic!("Nonsensical event payload passed to Player on MouseMoved");
      }

    });
  }
}
