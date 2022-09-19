use either::Either;
use serde::{Deserialize, Serialize};
use specs::prelude::*;

use engine::{
  ecs::{components::Player, Camera, Guid, MonoBehavior, PrefabBuilder, SystemUtilities, WorldProxy},
  events::{Event, EventChannel, KeyCode, ReceiverId, StatelessEventChannel, WindowEvent},
  gui::{widgets::*, ControlPanelBuilder, SystemDebugger},
  net::{ConnectionParameters, DuplexContext, HostContext, NetActor, NetActorHandle},
  physics::TransformComponent,
};

use crate::prefabs::{Cube, CubeState};

pub struct Multiplayer {
  key_event_id: Option<ReceiverId>,
  network: Option<NetActorHandle>,
  tx_rx: Either<HostContext, DuplexContext>,
  role_set: bool,
}

impl Default for Multiplayer {
  fn default() -> Self {
    Self {
      key_event_id: None,
      network: None,
      tx_rx: Either::Right(DuplexContext::default()),
      role_set: false,
    }
  }
}

impl<'a> MonoBehavior<'a> for Multiplayer {
  type SystemData = (
    Write<'a, StatelessEventChannel<WindowEvent>>,
    WriteStorage<'a, TransformComponent>,
    ReadStorage<'a, Camera>,
    ReadStorage<'a, Guid>,
    ReadStorage<'a, Player>,
  );

  fn run(
    &mut self,
    api: SystemUtilities<'a>,
    (evts, mut s_transform, s_cam, s_guid, s_player): Self::SystemData,
  ) {
    if let Some(net) = self.network.as_mut() {
      net.process_events();
    }
    if !self.role_set {
      self.handle_choose_role(&evts, &api);
    }
    let mut new_tx_rx = None;
    match &self.tx_rx {
      Either::Left(host_ctx) => {
        host_ctx.on_connect(&self.network, |client_id| {
          new_tx_rx = Some(Either::Right(DuplexContext::new(client_id)));
        });
      }
      Either::Right(client_ctx) => {
        client_ctx.on_message(&self.network, |message: PlayerStateEnvelope| {
          if let Some(msg_entity) = api.lookup_guid(&message.id) {
            // This entity is already in the system
            s_transform.get_mut(*msg_entity).map(|transform| {
              *transform = TransformComponent::from_buffer(message.transform_vector);
              transform.set_facing(message.facing.into());
            });
          } else {
            // This is a new entity for this system
            self.spawn_avatar(&api, message);
          }
        });
        for (transform, camera, guid, _p) in (&s_transform, &s_cam, &s_guid, &s_player).join() {
          let message = PlayerStateEnvelope {
            id: *guid,
            transform_vector: transform.matrix_buffer(),
            facing: camera.front().into(),
          };
          client_ctx.send(&self.network, message);
        }
      }
    }
    if new_tx_rx.is_some() {
      self.tx_rx = new_tx_rx.unwrap();
    }
  }

  fn setup(&mut self, mut world: WorldProxy) {
    Self::SystemData::setup(&mut world);
    self.register_debugger(&world);
    let id = {
      let mut listener = world.write_resource::<StatelessEventChannel<WindowEvent>>();
      listener.register_with_subs(&[
        WindowEvent::new(Event::KeyDown(KeyCode::H)),
        WindowEvent::new(Event::KeyDown(KeyCode::C)),
      ])
    };
    let (net, handle) = NetActor::create();
    net.execute();
    self.key_event_id = Some(id);
    self.network = Some(handle);
  }
}

impl<'a> SystemDebugger<'a> for Multiplayer {
  fn create_panel(&self) -> ControlPanelBuilder {
    ControlPanelBuilder::default()
      .with_title("Network")
      .push_line("Role", LabeledText::new("Undecided", "Role"))
      .push_line(
        "Connection Status",
        LabeledText::new("Disconnected", "Connection Status"),
      )
  }
}

impl Multiplayer {
  fn handle_choose_role<'a>(&mut self, evts: &StatelessEventChannel<WindowEvent>, api: &SystemUtilities<'a>) {
    let mut name = None;
    self.key_event_id.as_ref().map(|r_id| {
      evts.for_each(r_id, |evt| match evt.code {
        Event::KeyDown(KeyCode::H) => {
          let net = self.network.as_ref().unwrap();
          let host_id = net.new_host(ConnectionParameters::new("localhost", 8080), *r_id);
          self.tx_rx = Either::Left(HostContext::new(host_id));
          name = Some("host".to_string());
          self.role_set = true;
        }
        Event::KeyDown(KeyCode::C) => {
          let net = self.network.as_ref().unwrap();
          let client_id = net.new_duplex(ConnectionParameters::new("localhost", 8080), *r_id);
          self.tx_rx = Either::Right(DuplexContext::new(client_id));
          name = Some("Client".to_string());
          self.role_set = true;
        }
        _ => {}
      })
    });
    if let Some(n) = name {
      let mut panel = self.get_write_panel(&api);
      panel.set_str("Role", n);
    }
  }

  fn spawn_avatar<'a>(&self, api: &SystemUtilities<'a>, message: PlayerStateEnvelope) {
    let mut builder = Cube::default();
    let transform = TransformComponent::from_buffer(message.transform_vector);
    let state = CubeState::new(
      2.0,
      transform.translation,
      "resources/debug/brickwall.jpg",
      "resources/debug/bricks_tangent.png",
    );
    let avatar = builder.build(api, state);
    api.add_component(&avatar, message.id);
    {
      let mut panel = self.get_write_panel(&api);
      panel.set_str("Connection Status", "Connected!".to_string());
    }
  }
}

#[derive(Serialize, Deserialize)]
struct PlayerStateEnvelope {
  id: Guid,
  transform_vector: [f32; 10],
  facing: [f32; 3],
}
