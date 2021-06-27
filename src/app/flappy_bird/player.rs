use cgmath::prelude::*;
use specs::prelude::*;
use rand::{thread_rng, Rng};

use ecs::components::{EventReceiver, Gravity, Kinetics, Player, Position, Rotation, Transform};
use ecs::entity::EntityCrudEvent;
use ecs::CanCollide;
use events::{Event, EventChannel, KeyCode, StatefulEventChannel, StatelessEventChannel, WindowEvent};
use renderer::{Drawable, DrawableState};
use shapes::Sprite;
use gui::{GuiInputPanel, LabeledText, LineBreak};

use utils::{Vec2F, Vec3F, Timestep, Mat4F, random};

use app::flappy_bird::PlayerTailParticleState;

pub struct PlayerSystem {
  time_since_spawn: f32,
}

impl Default for PlayerSystem {
  fn default() -> Self {
    Self {
      time_since_spawn: 1e6
    }
  }
}

impl PlayerSystem {
  fn spawn_tail_particle(&self, position: Vec3F, lifetime: f32, spawner: &mut StatefulEventChannel<EntityCrudEvent, PlayerTailParticleState>) {
    let pi = 3.14159265;
    let theta = random::rand_float( 3f32 * pi / 2f32, 2f32 * pi);
    let theta = Vec2F::new(theta.cos(), theta.sin());
    let init_direction = Vec3F::new(theta.x, theta.y, 0f32).normalize_to(5f32);
    spawner.publish((
      EntityCrudEvent::Create,
      PlayerTailParticleState::new(position + Vec3F::unit_z(), lifetime, init_direction)));
  }
}

impl<'a> System<'a> for PlayerSystem {
  type SystemData = (
    ReadStorage<'a, Player>,
    ReadStorage<'a, Position>,
    WriteStorage<'a, Kinetics>,
    ReadStorage<'a, EventReceiver>,
    WriteStorage<'a, Rotation>,
    Write<'a, StatelessEventChannel<WindowEvent>>,
    Write<'a, StatefulEventChannel<EntityCrudEvent, PlayerTailParticleState>>,
    Read<'a, Timestep>,
    WriteStorage<'a, GuiInputPanel>,
  );

  fn run(&mut self, (p_storage, pos_storage, mut k_storage, e_storage, mut r_storage, channel, mut spawner, dt, mut debugger): Self::SystemData) {
    for (_p, position, kinetics, events, rotation, ui) in (&p_storage, &pos_storage, &mut k_storage, &e_storage, &mut r_storage, &mut debugger).join() {
      channel.for_each(&events.0, |evt| {
        match evt.code {
          Event::KeyDown(KeyCode::Space) => {
            kinetics.velocity = Vec3F::unit_y() * 7f32;
            for _ in 0..10 {
              self.spawn_tail_particle(position.0, 2f32, &mut spawner);
            }
          },
          _ => panic!(format!("Received a subbed event {:?} with no handler!", evt.code)),
        };
      });
      let front_vec = kinetics.velocity.normalize();
      rotation.0 = Vec2F::new(front_vec.x, front_vec.y);
      let new_time = self.time_since_spawn + dt.0;
      if new_time > 0.1f32 {
        self.spawn_tail_particle(position.0, 2f32, &mut spawner);
        self.time_since_spawn = 0f32;
      } else {
        self.time_since_spawn = new_time;
      }
      if ui.empty() {
        ui.push(Box::from(LineBreak));
        ui.push(Box::from(LabeledText::new(&to_string!(position.0), "Player Pos")));
      } else {
        ui.lines[1] = Box::from(LabeledText::new(&to_string!(position.0), "Player Pos"));
      }
    }
  }

  fn setup(&mut self, world: &mut World) {
    let receiver = {
      let mut listener = world.write_resource::<StatelessEventChannel<WindowEvent>>();
      EventReceiver(listener.register_with_subs(&[WindowEvent::new(Event::KeyDown(KeyCode::Space))]))
    };
    let sprite = Sprite::new("resources/flappy_bird/ship.png");
    let pos = Vec3F::unit_x() * 4f32;
    world.register::<Player>();
    world.register::<Rotation>();
    world.register::<Kinetics>();
    world.register::<EventReceiver>();
    world.register::<Position>();
    world.register::<Gravity>();
    world.register::<DrawableState>();
    world.register::<Transform>();
    world.register::<GuiInputPanel>();
    world.register::<CanCollide>();
    world
      .create_entity()
      .with(Player)
      .with(Rotation(Vec2F::unit_x()))
      .with(Kinetics::default())
      .with(receiver)
      .with(Position(pos))
      .with(Transform(Mat4F::from_translation(pos)))
      .with(Gravity)
      .with(sprite.state())
      .with(CanCollide {radius: 0.5f32})
      .with(GuiInputPanel::new("Player"))
      .build();
  }
}
