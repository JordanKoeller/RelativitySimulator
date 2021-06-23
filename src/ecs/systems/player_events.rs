use events::*;
use specs::prelude::*;

use ecs::components::{EventReceiver, Kinetics, Player, Position, Rotation};
use utils::{Vec3F};

use cgmath::prelude::*;
pub struct PlayerEvents;

impl<'a> System<'a> for PlayerEvents {
  type SystemData = (
    WriteStorage<'a, Position>,
    WriteStorage<'a, Rotation>,
    WriteStorage<'a, Kinetics>,
    ReadStorage<'a, Player>,
    ReadStorage<'a, EventReceiver>,
    Write<'a, StatelessEventChannel<WindowEvent>>,
  );

  fn run(
    &mut self,
    (mut pos_storage, mut rot_storage, mut kinetic_storage, _player_storage, evt_receiver, evt_channel): Self::SystemData,
  ) {

    for (_position, rotation, kinetics, receiver_id) in (&mut pos_storage, &mut rot_storage, &mut kinetic_storage, &evt_receiver).join() {
      evt_channel
        .for_each(&receiver_id.0, |window_event| {
          match window_event.code {
          Event::KeyDown(KeyCode::W) => kinetics.acceleration += rotation.front(),
          Event::KeyDown(KeyCode::A) => kinetics.acceleration -= rotation.right(),
          Event::KeyDown(KeyCode::S) => kinetics.acceleration -= rotation.front(),
          Event::KeyDown(KeyCode::D) => kinetics.acceleration += rotation.right(),
          Event::KeyDown(KeyCode::Space) => kinetics.acceleration += rotation.world_up(),
          Event::KeyDown(KeyCode::LeftShift) => kinetics.acceleration -= rotation.world_up(),
          Event::KeyDown(KeyCode::F) => {
            if kinetics.velocity.magnitude2() < 0.1 {
              kinetics.velocity = Vec3F::new(0f32, 0f32, 0f32);
            } else {
              let brake_direction = -kinetics.velocity.normalize();
              kinetics.velocity += kinetics.velocity.magnitude() * 0.05 * brake_direction;
            }
          }
          Event::MouseMoved => {
            if let Some(payload) = &window_event.payload {
              match payload {
                EventPayload::MouseMove(vec) => {
                  rotation.rotate(vec.x * 0.05, vec.y * 0.05)
                },
                _ => panic!(format!("Received a payload of {:?} on MouseMoved event!", payload))
              }
            }
          },
          _ => panic!(format!("Received a subbed event {:?} with no hanlder!", window_event.code))
        }
        });
    }
  }
}
