use cgmath::prelude::*;

use events::{Event, EventPayload, KeyCode, WindowEvent};
use physics::{RigidBody, TransformComponent};
use utils::Vec3F;

pub trait PlayerStateMachine {
  fn handle_event(&self, evt: &WindowEvent, transform: &mut TransformComponent, rigid_body: &mut RigidBody) -> bool;

  fn transition(&self) -> Box<dyn PlayerStateMachine + Send + Sync>;
}

pub struct FlyingPlayerState;
pub struct WalkingPlayerState;

impl PlayerStateMachine for FlyingPlayerState {
  fn handle_event(&self, evt: &WindowEvent, transform: &mut TransformComponent, _rigid_body: &mut RigidBody) -> bool {
    let init_rotation = transform.clone();
    let mut ret = false;
    match evt.code {
      Event::KeyDown(KeyCode::W) => transform.translation += init_rotation.front().normalize_to(0.1f32),
      Event::KeyDown(KeyCode::A) => transform.translation -= init_rotation.right().normalize_to(0.1f32),
      Event::KeyDown(KeyCode::S) => transform.translation -= init_rotation.front().normalize_to(0.1f32),
      Event::KeyDown(KeyCode::D) => transform.translation += init_rotation.right().normalize_to(0.1f32),
      Event::KeyDown(KeyCode::E) => ret = true,
      Event::KeyDown(KeyCode::LeftShift) => transform.translation -= init_rotation.up().normalize_to(0.1f32),
      Event::KeyDown(KeyCode::Space) => transform.translation += init_rotation.up().normalize_to(0.1f32),
      Event::MouseMoved => {
        if let Some(payload) = &evt.payload {
          match payload {
            EventPayload::MouseMove(vec) => transform.rotate(vec.x * 0.05, vec.y * 0.05),
            _ => panic!("Received a payload of {:?} on MouseMoved event!", payload),
          }
        }
      }
      _ => panic!(
        "Received an event that the player controller does not listen for! {:?}",
        evt
      ),
    };
    ret
  }

  fn transition(&self) -> Box<dyn PlayerStateMachine + Send + Sync> {
    Box::from(WalkingPlayerState)
  }
}

impl PlayerStateMachine for WalkingPlayerState {
  fn handle_event(&self, evt: &WindowEvent, transform: &mut TransformComponent, rigid_body: &mut RigidBody) -> bool {
    let mut needs_transition = false;

    match evt.code {
      Event::KeyDown(KeyCode::W) => rigid_body.velocity =  transform.front().normalize_to(10f32),
      Event::KeyDown(KeyCode::A) => rigid_body.velocity = -transform.right().normalize_to(10f32),
      Event::KeyDown(KeyCode::S) => rigid_body.velocity = -transform.front().normalize_to(10f32),
      Event::KeyDown(KeyCode::D) => rigid_body.velocity =  transform.right().normalize_to(10f32),
      Event::KeyDown(KeyCode::E) => needs_transition = true,
      Event::KeyDown(KeyCode::Space) => rigid_body.velocity = Vec3F::unit_y().normalize_to(10f32),
      Event::KeyDown(KeyCode::LeftShift) => rigid_body.velocity = - Vec3F::unit_y().normalize_to(10f32),
      Event::MouseMoved => {
        if let Some(payload) = &evt.payload {
          match payload {
            EventPayload::MouseMove(vec) => transform.rotate(vec.x * 0.05, vec.y * 0.05),
            _ => panic!("Received a payload of {:?} on MouseMoved event!", payload),
          }
        }
      }
      _ => panic!(
        "Received an event that the player controller does not listen for! {:?}",
        evt
      ),
    };
    needs_transition
  }

  fn transition(&self) -> Box<dyn PlayerStateMachine + Send + Sync> {
    Box::from(FlyingPlayerState)
  }
}