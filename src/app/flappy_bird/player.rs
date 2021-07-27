use std::time::Duration;
use cgmath::prelude::*;
use specs::prelude::*;
use rand::{thread_rng, Rng};

use ecs::components::{EventReceiver,Player, MeshComponent};
use ecs::entity::EntityCrudEvent;
use physics::CanCollide;
use events::{Event, EventChannel, KeyCode, StatefulEventChannel, StatelessEventChannel, WindowEvent};
use renderer::{Drawable, Mesh};
use shapes::Sprite;
use gui::{GuiInputPanel, LabeledText, LineBreak};

use utils::{Vec2F, Vec3F, Timestep, Mat4F, random, QuatF, Timer, TimerLike};

use app::flappy_bird::PlayerTailParticleState;

use physics::{TransformComponent, RigidBody, Gravity};

pub struct PlayerSystem {
  tail_spawn_timer: Timer,
}

impl Default for PlayerSystem {
  fn default() -> Self {
    Self {
      tail_spawn_timer: Timer::new(Duration::from_millis(100))
    }
  }
}

impl PlayerSystem {
  fn spawn_tail_particle(&self, position: Vec3F, lifetime: Duration, spawner: &mut StatefulEventChannel<EntityCrudEvent, PlayerTailParticleState>) {
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
    WriteStorage<'a, TransformComponent>,
    WriteStorage<'a, RigidBody>,
    ReadStorage<'a, EventReceiver>,
    Write<'a, StatelessEventChannel<WindowEvent>>,
    Write<'a, StatefulEventChannel<EntityCrudEvent, PlayerTailParticleState>>,
    Read<'a, Timestep>,
    WriteStorage<'a, GuiInputPanel>,
  );

  fn run(&mut self, (p_storage, mut transform_storage, mut rigid_storage, e_storage, channel, mut spawner, dt, mut debugger): Self::SystemData) {
    for (_p, transform, rigid_body, events, ui) in (&p_storage, &mut transform_storage, &mut rigid_storage, &e_storage, &mut debugger).join() {
      channel.for_each(&events.0, |evt| {
        match evt.code {
          Event::KeyDown(KeyCode::Space) => {
            rigid_body.velocity = Vec3F::unit_y() * 7f32;
            self.tail_spawn_timer.reset_interval(Duration::from_millis(3));
          },
          Event::KeyDown(KeyCode::LeftShift) => {
          }
          Event::KeyReleased(KeyCode::Space) => {
            self.tail_spawn_timer.reset_interval(Duration::from_millis(100));
          }
          _ => panic!("Received a subbed event {:?} with no handler!", evt.code)
        };
      });
      if transform.translation.y < -16f32 {
        transform.translation.y = -16f32;
      }
      if transform.translation.y > 16f32 {
        transform.translation.y = 16f32;
      }
      for _ in 0..self.tail_spawn_timer.start_poll_all(dt.curr_time()) {
        self.spawn_tail_particle(transform.translation, Duration::from_secs(2), &mut spawner);
      }
      if ui.empty() {
        ui.push(Box::from(LineBreak));
        ui.push(Box::from(LabeledText::new(&to_string!(transform.translation), "Player Pos")));
      } else {
        ui.lines[1] = Box::from(LabeledText::new(&to_string!(transform.translation), "Player Pos"));
      }
    }
  }

  fn setup(&mut self, world: &mut World) {
    let receiver = {
      let mut listener = world.write_resource::<StatelessEventChannel<WindowEvent>>();
      EventReceiver(listener.register_with_subs(&[
        WindowEvent::new(Event::KeyDown(KeyCode::Space)),
        WindowEvent::new(Event::KeyReleased(KeyCode::Space)),
        WindowEvent::new(Event::KeyDown(KeyCode::LeftShift))
        ]))
    };
    let sprite = Sprite::new("resources/flappy_bird/ship.png", false);
    let pos = Vec3F::unit_x() * 4f32;
    // world.setup::<Self::SystemData>();
    world.register::<Player>();
    world.register::<RigidBody>();
    world.register::<TransformComponent>();
    world.register::<EventReceiver>();
    world.register::<Gravity>();
    world.register::<MeshComponent>();
    world.register::<GuiInputPanel>();
    world.register::<CanCollide>();
    let mut tc = TransformComponent::new(pos, Vec3F::new(1f32, 1f32, 1f32), QuatF::zero());
    tc.rotation = Vec3F::unit_y() * 90f32;
    world
      .create_entity()
      .with(Player)
      .with(tc)
      .with(RigidBody::new_stationary())
      .with(receiver)
      .with(Gravity)
      .with(sprite.mesh_component())
      .with(sprite.material())
      .with(CanCollide {radius: 0.5f32})
      .with(GuiInputPanel::new("Player"))
      .build();
  }
}
