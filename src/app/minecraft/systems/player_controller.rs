use app::minecraft::{ChunkComponent, ChunkGrid, FlyingPlayerState, PlayerStateMachine};
use cgmath::prelude::*;
use ecs::components::{Camera, EntityTargetComponent, EventReceiver, MeshComponent, Player};
use ecs::SystemDelegate;
use events::{Event, EventChannel, EventPayload, KeyCode, StatefulEventChannel, StatelessEventChannel, WindowEvent};
use gui::*;
use physics::{Gravity, RigidBody, TransformComponent};
use specs::prelude::*;
use utils::{random, Mat4F, QuatF, Timer, TimerLike, Timestep, Vec2F, Vec3F};

const IMPULSE: f32 = 0.2f32;

#[derive(SystemData)]
pub struct PlayerControllerSystemData<'a> {
  player: ReadStorage<'a, Player>,
  chunks: Read<'a, ChunkGrid>,
  chunk_storage: ReadStorage<'a, ChunkComponent>,
  camera: WriteStorage<'a, Camera>,
  rigid_body: WriteStorage<'a, RigidBody>,
  transform: WriteStorage<'a, TransformComponent>,
  event_receiver: ReadStorage<'a, EventReceiver>,
  event_channel: Write<'a, StatelessEventChannel<WindowEvent>>,
  timestep: Read<'a, Timestep>,
}

pub struct PlayerController {
  player_state_machine: Box<dyn PlayerStateMachine + Send + Sync>,
}

impl<'a> SystemDelegate<'a> for PlayerController {
  type SystemData = PlayerControllerSystemData<'a>;

  fn run(&mut self, mut s: Self::SystemData) {
    let mut needs_transition = false;
    for (_p, mut camera, transform, rigid_body, events) in (
      &s.player,
      &mut s.camera,
      &mut s.transform,
      &mut s.rigid_body,
      &s.event_receiver,
    )
      .join()
    {
      let mut next_transform = transform.clone();
      s.event_channel.for_each(&events.0, |evt| {
        if self
          .player_state_machine
          .handle_event(evt, &mut next_transform, rigid_body)
        {
          needs_transition = true;
        }
      });
      self.refresh_camera(&transform, &mut camera);
      if needs_transition {
        self.player_state_machine = self.player_state_machine.transition();
      }
      let mut colliding = false;
      if let Some(chunk_id) = s.chunks.get_entity_from_coord(&next_transform.translation) {
        if let Some(chunk) = s.chunk_storage.get(*chunk_id) {
          if chunk.collides(&next_transform.translation) {
            colliding = true;
          }
        }
      }
      if !colliding {
        transform.copy_from(next_transform);
      }
    }
  }

  fn update_debugger(&mut self, s: &mut Self::SystemData, debugger: &mut DebugPanel) {
    for (_p, rigid_body, transform) in (&s.player, &mut s.rigid_body, &s.transform).join() {
      debugger.panel.lines[1] = Box::from(LabeledText::new("Position", &to_string!(transform.translation)));
      debugger.panel.lines[2] = Box::from(LabeledText::new("Velocity", &to_string!(rigid_body.velocity)));
      debugger.panel.lines[3] = Box::from(LabeledText::new("Facing", &to_string!(transform.front())));
    }
  }

  fn setup(&mut self, world: &mut World) {
    let receiver = {
      let mut listener = world.write_resource::<StatelessEventChannel<WindowEvent>>();
      EventReceiver(listener.register_with_subs(&[
        WindowEvent::new(Event::KeyDown(KeyCode::W)),
        WindowEvent::new(Event::KeyDown(KeyCode::A)),
        WindowEvent::new(Event::KeyDown(KeyCode::S)),
        WindowEvent::new(Event::KeyDown(KeyCode::D)),
        WindowEvent::new(Event::KeyDown(KeyCode::E)),
        WindowEvent::new(Event::KeyDown(KeyCode::LeftShift)),
        WindowEvent::new(Event::KeyDown(KeyCode::Space)),
        WindowEvent::new(Event::MouseMoved),
      ]))
    };
    let pos = Vec3F::new(4f32, 20f32, 2f32);
    let mut tc = TransformComponent::new(pos, Vec3F::new(1f32, 1f32, 1f32), QuatF::zero());
    tc.rotation = Vec3F::unit_y() * 90f32;
    world.register::<Player>();
    world.register::<RigidBody>();
    world.register::<Gravity>();
    world.register::<TransformComponent>();
    world.register::<EventReceiver>();
    world.register::<Camera>();
    world.register::<MeshComponent>();
    world.register::<EntityTargetComponent>();
    world
      .create_entity()
      .with(Player)
      .with(EntityTargetComponent::default())
      // .with(Gravity)
      .with(tc)
      .with(Camera::default())
      .with(RigidBody::new_stationary())
      .with(receiver)
      .build();
  }

  fn setup_debug_panel(&mut self, _world: &mut World) -> Option<DebugPanel> {
    let mut gui = DebugPanel::new("Player Controller");
    gui.panel.push(Box::from(LabeledText::new("Pressed Buttons", "")));
    gui.panel.push(Box::from(LabeledText::new("Position", "")));
    gui.panel.push(Box::from(LabeledText::new("Velocity", "")));
    gui.panel.push(Box::from(LabeledText::new("Facing", "")));
    Some(gui)
  }
}

impl PlayerController {
  fn refresh_camera(&self, t: &TransformComponent, cam: &mut Camera) {
    let location = cgmath::Point3::<f32>::new(t.translation.x, t.translation.y, t.translation.z);
    let pov = t.translation + t.front();
    let center = cgmath::Point3::<f32>::new(pov.x, pov.y, pov.z);
    let up = Vec3F::unit_y();
    let matrix = Mat4F::look_at(location, center, up);
    cam.set_matrix(matrix);
  }
}

impl Default for PlayerController {
  fn default() -> Self {
    Self {
      player_state_machine: Box::new(FlyingPlayerState),
    }
  }
}
