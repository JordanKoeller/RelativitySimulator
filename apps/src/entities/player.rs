use specs::prelude::*;
use cgmath::{One, Zero};

use ecs::*;
use events::{EventChannel, StatelessEventChannel, WindowEvent, Event, KeyCode};
use utils::{Vec3F, Vec2F, QuatF};
use gui::GuiInputPanel;
use physics::{TransformComponent, RigidBody, CanCollide};

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
    .with(TransformComponent::new(pos, Vec3F::new(1f64, 1f64, 1f64), QuatF::zero()))
    .with(RigidBody::new_stationary())
    .with(receiver)
    .with(CanCollide {radius: 1f64})
    .with(GuiInputPanel::new("Player"))
    .build();
}

