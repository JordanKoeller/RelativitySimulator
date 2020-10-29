// use specs::{Builder, Component, ReadStorage, Read, RunNow, System, VecStorage, World, WorldExt, WriteStorage, Write};
use events::*;
use specs::prelude::*;
// use ecs::components::*;
use ecs::components::{EventReceiver, Kinetics, Player, Position, Rotation};
use utils::{Mat4F, Vec2F, Vec3F};

use cgmath::prelude::*;
pub struct PlayerEvents;

impl<'a> System<'a> for PlayerEvents {
  type SystemData = (
    WriteStorage<'a, Position>,
    WriteStorage<'a, Rotation>,
    WriteStorage<'a, Kinetics>,
    ReadStorage<'a, Player>,
    ReadStorage<'a, EventReceiver>,
    Write<'a, EventChannel<WindowEvent>>,
  );

  fn run(
    &mut self,
    (mut pos_storage, mut rot_storage, mut kinetic_storage, player_storage, evt_receiver, mut evt_channel): Self::SystemData,
  ) {
    use specs::Join;

    for (position, rotation, kinetics, receiver_id) in (&mut pos_storage, &mut rot_storage, &mut kinetic_storage, &evt_receiver).join() {
      evt_channel
        .read(&receiver_id.0)
        .for_each(move |window_event: &WindowEvent| {
          match window_event.code {
          Event::KeyDown(KeyCode::W) => kinetics.acceleration += rotation.front(),
          Event::KeyDown(KeyCode::A) => kinetics.acceleration -= rotation.right(),
          Event::KeyDown(KeyCode::S) => kinetics.acceleration -= rotation.front(),
          Event::KeyDown(KeyCode::D) => kinetics.acceleration += rotation.right(),
          Event::KeyDown(KeyCode::Space) => kinetics.acceleration += rotation.world_up(),
          Event::KeyDown(KeyCode::LeftShift) => kinetics.acceleration -= rotation.world_up(),
          Event::KeyDown(KeyCode::F) => {
            if kinetics.velocity.magnitude() < 0.1 {
              kinetics.velocity = Vec3F::new(0f32, 0f32, 0f32);
            } else {
              let brake_direction = -kinetics.velocity.normalize();
              kinetics.velocity += kinetics.velocity.magnitude() * 0.05 * brake_direction;
            }
          }
          Event::MouseMoved => {
            if let Some(payload) = &window_event.payload {
              match payload {
                EventPayload::MouseMove(vec) => rotation.rotate(vec.x * 0.05, vec.y * 0.05),
                _ => panic!(format!("Received a payload of {:?} on MouseMoved event!", payload))
              }
            }
            // rotation.rotate(window_event.payload.unwrap().0.x * 0.05, window_event.payload.unwrap().y * 0.05)
          },
          _ => panic!(format!("Received a subbed event {:?} with no hanlder!", window_event.code))
        }
        });
    }
  }
}

// event_mgr.0.subscribe_to(
//   Event::KeyDown(KeyCode::W),
//   |(rot, kin), _| {
//     let facing = rot.front();
//     kin.acceleration += facing;
//   },
//   &mut (*evts),
// );
// event_mgr.0.subscribe_to(
//   Event::KeyDown(KeyCode::S),
//   |(rot, kin), _| {
//     let facing = rot.front();
//     kin.acceleration -= facing;
//   },
//   &mut (*evts),
// );
// event_mgr.0.subscribe_to(
//   Event::KeyDown(KeyCode::A),
//   |(rot, kin), _| {
//     let facing = rot.right();
//     kin.acceleration -= facing;
//   },
//   &mut (*evts),
// );
// event_mgr.0.subscribe_to(
//   Event::KeyDown(KeyCode::D),
//   |(rot, kin), _| {
//     let facing = rot.right();
//     kin.acceleration += facing;
//   },
//   &mut (*evts),
// );
// event_mgr.0.subscribe_to(
//   Event::KeyDown(KeyCode::F),
//   |(rot, kin), _| {
//     if kin.velocity.magnitude() < 0.1 {
//       kin.velocity = Vec3F::new(0f32, 0f32, 0f32);
//     } else {
//       let brake_direction = - kin.velocity.normalize();
//       kin.velocity += kin.velocity.magnitude() * 0.05 * brake_direction;
//     }
//   },
//   &mut (*evts),
// );
// event_mgr.0.subscribe_to(
//   Event::KeyDown(KeyCode::LeftShift),
//   |(rot, kin), _| {
//     kin.acceleration -= Vec3F::unit_y();
//   },
//   &mut (*evts),
// );
// event_mgr.0.subscribe_to(
//   Event::KeyDown(KeyCode::Space),
//   |(rot, kin), _| {
//     kin.acceleration -= Vec3F::unit_y();
//   },
//   &mut (*evts),
// );
// event_mgr.0.subscribe_to(
//   Event::MouseMoved,
//   |(rot, kin), data| {
//     if let (_, Some(EventPayload::MouseMove(mouse_move))) = data {
//       rot.rotate(mouse_move.x * 0.05, mouse_move.y * 0.05);
//     } else {
//       panic!("Nonsensical event payload passed to Player on MouseMoved");
//     }
//   },
//   &mut (*evts),
// )
