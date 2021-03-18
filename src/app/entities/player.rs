use specs::prelude::*;

use ecs::*;
use events::{EventChannel, StatelessEventChannel, WindowEvent, Event, KeyCode};
use utils::{Vec3F, Vec2F};

pub fn create_player<'a>(pos: Vec3F, world: &'a mut World) {
  let receiver = {
    let mut listener = world.write_resource::<StatelessEventChannel<WindowEvent>>();
    EventReceiver(listener.register_with_subs(&[
      WindowEvent::new(Event::KeyDown(KeyCode::W)),
      WindowEvent::new(Event::KeyDown(KeyCode::A)),
      WindowEvent::new(Event::KeyDown(KeyCode::S)),
      WindowEvent::new(Event::KeyDown(KeyCode::D)),
      WindowEvent::new(Event::KeyDown(KeyCode::F)),
      WindowEvent::new(Event::KeyDown(KeyCode::LeftShift)),
      WindowEvent::new(Event::KeyDown(KeyCode::Space)),
      WindowEvent::new(Event::MouseMoved),
    ]))
  };
    world.create_entity()
    .with(Player)
    .with(Rotation(Vec2F::new(0f32, 0f32)))
    .with(Kinetics::default())
    .with(receiver)
    .with(Position(pos))
    .build();
}

