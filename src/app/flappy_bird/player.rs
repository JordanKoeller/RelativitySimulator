use cgmath::prelude::*;
use specs::prelude::*;

use ecs::components::{EventReceiver, Gravity, Kinetics, Player, Position, Rotation};
use ecs::entity::EntityCrudEvent;
use events::{Event, EventChannel, KeyCode, StatefulEventChannel, StatelessEventChannel, WindowEvent};
use renderer::{Drawable, DrawableState};
use shapes::Sprite;

use gui::GuiInputPanel;
use utils::{Vec2F, Vec3F};

use app::flappy_bird::PlayerTailParticleState;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
  type SystemData = (
    ReadStorage<'a, Player>,
    WriteStorage<'a, Kinetics>,
    ReadStorage<'a, EventReceiver>,
    WriteStorage<'a, Rotation>,
    Write<'a, StatelessEventChannel<WindowEvent>>,
  );

  fn run(&mut self, (p_storage, mut k_storage, e_storage, mut r_storage, channel): Self::SystemData) {
    for (_p, kinetics, events, rotation) in (&p_storage, &mut k_storage, &e_storage, &mut r_storage).join() {
      channel.for_each(&events.0, |evt| {
        match evt.code {
          Event::KeyDown(KeyCode::Space) => kinetics.velocity = Vec3F::unit_y() * 5f32 + Vec3F::unit_x() * 2f32,
          _ => panic!(format!("Received a subbed event {:?} with no handlr!", evt.code)),
        };
      });
      let front_vec = kinetics.velocity.normalize();
      rotation.0 = Vec2F::new(front_vec.x, front_vec.y);
    }
  }

  fn setup(&mut self, world: &mut World) {
    let receiver = {
      let mut listener = world.write_resource::<StatelessEventChannel<WindowEvent>>();
      EventReceiver(listener.register_with_subs(&[WindowEvent::new(Event::KeyDown(KeyCode::Space))]))
    };
    let sprite = Sprite::new("resources/flappy_bird/spaceship.jpg");
    world.register::<Player>();
    world.register::<Rotation>();
    world.register::<Kinetics>();
    world.register::<EventReceiver>();
    world.register::<Position>();
    world.register::<Gravity>();
    world.register::<DrawableState>();
    world
      .create_entity()
      .with(Player)
      .with(Rotation(Vec2F::unit_x()))
      .with(Kinetics::default())
      .with(receiver)
      .with(Position(Vec3F::unit_y()))
      .with(Gravity)
      .with(sprite.state())
      // .with(CanCollide {radius: 1f32})
      // .with(GuiInputPanel::new("Player"))
      .build();
  }
}
