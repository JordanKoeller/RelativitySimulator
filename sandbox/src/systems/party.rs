use specs::prelude::*;

use engine::{
  ecs::{MonoBehavior, PrefabBuilder, SystemUtilities, WorldProxy},
  events::{Event, EventChannel, KeyCode, ReceiverID, StatelessEventChannel, WindowEvent},
  gui::{widgets::*, ControlPanelBuilder, SystemDebugger},
  net::{ConnectionId, ConnectionParameters, NetActor, NetActorHandle},
  physics::TransformComponent,
  utils::Vec3F,
};

use crate::prefabs::{Cube, CubeState};

#[derive(Default)]
pub struct Party {
  key_event_id: Option<ReceiverID>,
  network: Option<NetActorHandle>,
  net_role: Option<NodeRole>,
}

impl<'a> MonoBehavior<'a> for Party {
  type SystemData = (
    Write<'a, StatelessEventChannel<WindowEvent>>,
    ReadStorage<'a, ConnectionId>,
    WriteStorage<'a, TransformComponent>,
  );

  fn run(&mut self, api: SystemUtilities<'a>, (evts, connection_storage, mut transform_storage): Self::SystemData) {
    if self.net_role.is_none() {
      self.handle_choose_role(&evts, &api);
    }
    self.handle_new_connections(&api);
    if let Some(net) = self.network.as_mut() {
      while let Some(message) = net.read_message_raw() {
        for (c_id, _transform) in (&connection_storage, &mut transform_storage).join() {
          if c_id == message.connection_id() {
            println!("Synchonrizing state!");
            // Extract out the new transform component
            // Update transform storage accordingly.
          }
        }
      }
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

impl Party {
  fn handle_choose_role<'a>(&mut self, evts: &StatelessEventChannel<WindowEvent>, api: &SystemUtilities<'a>) {
    self.key_event_id.as_ref().map(|r_id| {
      evts.for_each(r_id, |evt| match evt.code {
        Event::KeyDown(KeyCode::H) => {
          let net = self.network.as_ref().unwrap();
          let host_id = net.create_connection(ConnectionParameters::new_tcp_host("localhost", 8080), *r_id);
          self.net_role = Some(NodeRole::Host { connection_id: host_id });
        }
        Event::KeyDown(KeyCode::C) => {
          let net = self.network.as_ref().unwrap();
          let client_id = net.create_connection(ConnectionParameters::new_tcp_client("localhost", 8080), *r_id);
          self.net_role = Some(NodeRole::Client {
            connection_id: client_id,
          });
        }
        _ => {}
      })
    });
    {
      let mut panel = self.get_write_panel(&api);
      panel.set_str("Role", self.get_role_string().to_string());
    }
  }

  fn handle_new_connections<'a>(&mut self, api: &SystemUtilities<'a>) {
    if let Some((role, net)) = self.net_role.as_ref().zip(self.network.as_mut()) {
      if let Some(new_connections) = net.get_new_connections(role.id()) {
        new_connections.iter().for_each(|cxn| {
          let mut builder = Cube::default();
          let mut state = CubeState::new(
            2.0,
            Vec3F::new(16f32, 12f32, 14f32),
            "resources/debug/brickwall.jpg",
            "resources/debug/bricks_tangent.png",
          );
          builder.build(api, state);
          {
            let mut panel = self.get_write_panel(&api);
            panel.set_str("Connection Status", "Connected!".to_string());
          }
        });
      }
    }
  }

  fn get_role_string(&self) -> &str {
    match &self.net_role {
      Some(role) => match role {
        NodeRole::Client { .. } => "Client",
        NodeRole::Host { .. } => "Host",
      },
      None => "Undecided",
    }
  }
}

impl<'a> SystemDebugger<'a> for Party {
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

enum NodeRole {
  Client { connection_id: ConnectionId },
  Host { connection_id: ConnectionId },
}

impl NodeRole {
  pub fn id(&self) -> &ConnectionId {
    match self {
      NodeRole::Client { connection_id } => connection_id,
      NodeRole::Host { connection_id } => connection_id,
    }
  }
}
