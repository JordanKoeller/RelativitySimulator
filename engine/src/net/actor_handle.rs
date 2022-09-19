use std::collections::HashMap;
use std::io::{Error, IoSlice};
use std::result::Result;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

use serde::{de::DeserializeOwned, Serialize};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{
  tcp::{OwnedReadHalf, OwnedWriteHalf},
  TcpListener, TcpStream,
};
use tokio::runtime;
use tokio::spawn as dispatch;
use tokio::sync::mpsc::{
  channel as async_channel, unbounded_channel, Receiver as AsyncReceiver, Sender as AsyncSender, UnboundedReceiver,
  UnboundedSender,
};
use tokio::sync::watch::{channel as watch_channel, Receiver as WatchReceiver, Sender as WatchSender};

use super::{
  ActorMessage, Connection, ConnectionId, ConnectionManager, ConnectionParameters, ConnectionStatus,
  DuplexConnectionId, EgressMessage, Envelope, GenericConnectionId, HostConnectionId, NetEvent, NewConnectionMessage,
  SocketType,
};
use crate::events::ReceiverId;

const HEADER_SIZE: u32 = 4;

pub struct NetActorHandle {
  connections: Arc<ConnectionManager>,
  ingress_channel: UnboundedSender<ActorMessage>,
  egress_channel: Receiver<NetEvent>,
  new_messages: HashMap<DuplexConnectionId, Vec<Envelope>>,
  new_connections: HashMap<HostConnectionId, Vec<DuplexConnectionId>>,
  events: HashMap<GenericConnectionId, Vec<NetEvent>>,
}

impl NetActorHandle {
  pub(crate) fn new(
    connections: Arc<ConnectionManager>,
    ingress_channel: UnboundedSender<ActorMessage>,
    egress_channel: Receiver<NetEvent>,
  ) -> Self {
    Self {
      connections,
      ingress_channel,
      egress_channel,
      new_messages: HashMap::new(),
      new_connections: HashMap::new(),
      events: HashMap::new(),
    }
  }

  pub fn connection_manager(&self) -> &Arc<ConnectionManager> {
    &self.connections
  }

  pub fn new_host(&self, cx: ConnectionParameters, receiver_id: ReceiverId) -> HostConnectionId {
    let host_id = HostConnectionId::new(receiver_id, self.connection_manager().get_channel_id());
    self
      .connection_manager()
      .set_channel_status(*host_id.channel(), ConnectionStatus::Pending);
    self.send_to_actor(ActorMessage::HostConnection(cx.to_connection(host_id.clone().into())));
    host_id
  }

  pub fn new_duplex(&self, cx: ConnectionParameters, receiver_id: ReceiverId) -> DuplexConnectionId {
    let duplex_id = DuplexConnectionId::new(receiver_id, self.connection_manager().get_channel_id());
    self
      .connection_manager()
      .set_channel_status(*duplex_id.channel(), ConnectionStatus::Pending);
    self.send_to_actor(ActorMessage::ConnectTo(cx.to_connection(duplex_id.clone().into())));
    duplex_id
  }

  pub fn send<M: Serialize + DeserializeOwned>(&self, cx: &DuplexConnectionId, message: M) {
    let data = serde_json::to_vec(&message).unwrap();
    self.send_to_actor(ActorMessage::SendMessage(Envelope::new(data, cx.clone().into())));
  }

  pub fn send_raw(&self, cx: &DuplexConnectionId, data: Vec<u8>) {
    self.send_to_actor(ActorMessage::SendMessage(Envelope::new(data, cx.clone().into())));
  }

  pub fn broadcast<M: Serialize + DeserializeOwned>(&self, _cx: &HostConnectionId, _message: M) {
    panic!("Not implemented!");
    // let data = serde_json::to_vec(&message).unwrap();
    // self.send_to_actor(ActorMessage::SendMessage(Envelope::new(data, cx.clone().into())));
  }

  pub fn get_connections(&self, cx: &HostConnectionId) -> Vec<DuplexConnectionId> {
    self.new_connections.get(cx).unwrap_or(&Vec::new()).clone()
  }

  pub fn get_events<T: ConnectionId>(&self, cx: &T) -> Vec<NetEvent> {
    let id = GenericConnectionId::new(*cx.receiver(), *cx.channel(), *cx.socket_type());
    self.events.get(&id).unwrap_or(&Vec::new()).clone()
  }

  pub fn get_messages<'a, M: Serialize + DeserializeOwned>(&'a self, cx: &DuplexConnectionId) -> Vec<M> {
    let mut ret: Vec<M> = Vec::new();
    if let Some(messages) = self.new_messages.get(cx) {
      for envelope in messages.iter() {
        if let Some(message) = serde_json::from_slice(&envelope.data).ok() {
          ret.push(message);
        }
      }
    }
    ret
  }

  pub fn get_messages_raw(&self, cx: &DuplexConnectionId) -> Vec<Envelope> {
    self.new_messages.get(cx).unwrap_or(&Vec::new()).clone()
  }

  /*
  Ingress all pending messages from the Network Actor.

  This method must be called to poopulate the NetHandle with new messages and connections, retrievable through
  `get_messages` and `get_connections`, respectively.
  */
  pub fn process_events(&mut self) {
    self.new_messages.clear();
    self.new_connections.clear();
    self.events.clear();
    for event in self.egress_channel.try_iter() {
      match event {
        NetEvent::Message {
          connection_id,
          envelope,
        } => {
          let duplex_id: DuplexConnectionId = connection_id.into();
          if self.new_messages.contains_key(&duplex_id) {
            self.new_messages.get_mut(&duplex_id).unwrap().push(envelope);
          } else {
            self.new_messages.insert(duplex_id, vec![envelope]);
          }
        }
        NetEvent::NewClient { host_id, client } => {
          let duplex_id: DuplexConnectionId = client.id().into();
          let host_id = host_id.into();
          if self.new_connections.contains_key(&host_id) {
            self.new_connections.get_mut(&host_id).unwrap().push(duplex_id);
          } else {
            self.new_connections.insert(host_id, vec![duplex_id]);
          }
        }
        NetEvent::Connected { connection_id } => {
          println!("Handling NetEvent::Connected {:?}", connection_id);
          let connection_id = connection_id.into();
          if self.events.contains_key(&connection_id) {
            self.events.get_mut(&connection_id).unwrap().push(event);
          } else {
            self.events.insert(connection_id, vec![event]);
          }
        }
        _ => println!("Net Event {:?} is not supported!", event),
      }
    }
  }

  // pub fn drop_duplex(&mut self, cx: DuplexConnectionId);
  // pub fn close_listener(&mut self, cx: HostConnectionId);
  // pub fn drop_broadcast(&mut self, cx: HostConnectionId);

  pub fn shutdown(&self) {
    self.ingress_channel.send(ActorMessage::Shutdown).ok();
  }

  pub fn get_connection_status<T: ConnectionId>(&self, id: &T) -> ConnectionStatus {
    self.connections.get_channel_status(*id.channel())
  }

  fn send_to_actor(&self, message: ActorMessage) {
    self.ingress_channel.send(message).ok();
  }
}

#[cfg(test)]
mod test {
  use super::super::NetActor;
  use super::*;

  #[test]
  fn can_make_netactor() {
    let (_actor, _handle) = NetActor::create();
  }

  #[test]
  fn can_start_netactor() {
    let (actor, handle) = NetActor::create();

    actor.execute();

    thread::sleep_ms(3000);

    handle.shutdown();
  }

  #[test]
  fn can_start_and_connect_host() {
    let (actor, mut handle) = NetActor::create();

    actor.execute();

    thread::sleep_ms(500);

    let host_id = handle.new_host(ConnectionParameters::new("localhost", 8080), 0);

    thread::sleep_ms(500);

    let client_id = handle.new_duplex(ConnectionParameters::new("localhost", 8080), 0);

    thread::sleep_ms(500);

    let msg = vec![42, 42, 42, 42];

    handle.process_events();
    let host_client = handle.get_connections(&host_id);

    assert_eq!(host_client.len(), 1);

    handle.send(&host_client[0], msg);

    thread::sleep_ms(500);

    handle.process_events();
    let response = handle.get_messages_raw(&client_id);

    assert_eq!(response.is_empty(), false);

    handle.shutdown();
  }

  #[test]
  fn can_get_connection_satus() {
    let (actor, mut handle) = NetActor::create();

    actor.execute();
    let host_id = handle.new_host(ConnectionParameters::new("localhost", 8080), 0);
    assert_eq!(handle.get_connection_status(&host_id), ConnectionStatus::Pending);
    thread::sleep_ms(500);
    assert_eq!(handle.get_connection_status(&host_id), ConnectionStatus::Stable);
    let client_id = handle.new_duplex(ConnectionParameters::new("localhost", 8080), 0);
    assert_eq!(handle.get_connection_status(&client_id), ConnectionStatus::Pending);
    thread::sleep_ms(500);
    assert_eq!(handle.get_connection_status(&client_id), ConnectionStatus::Stable);

    thread::sleep_ms(500);

    handle.shutdown();
  }

  #[test]
  fn can_send_arbitrary_packet_sizes() {
    let (actor, mut handle) = NetActor::create();

    // Start connection
    actor.execute();
    thread::sleep_ms(500);
    let host_id = handle.new_host(ConnectionParameters::new("localhost", 8080), 0);
    thread::sleep_ms(500);
    let client_id = handle.new_duplex(ConnectionParameters::new("localhost", 8080), 0);
    thread::sleep_ms(500);

    // Create a message
    let mut msg: Vec<u8> = Vec::with_capacity(5usize);
    for i in 0..5 {
      msg.push(((i * 2 + 12) % 255) as u8);
    }

    handle.process_events();
    let host_client = handle.get_connections(&host_id);
    assert_eq!(host_client.len(), 1);

    // tx/rx
    handle.send_raw(&host_client[0], msg.clone());
    thread::sleep_ms(500);
    handle.process_events();
    let response = handle.get_messages_raw(&client_id);

    // assert
    assert_eq!(response.is_empty(), false);
    assert_eq!(response.first().unwrap().data, msg);

    //cleanup
    handle.shutdown();
  }

  #[test]
  fn can_send_serialized_data() {
    let (actor, mut handle) = NetActor::create();

    // Start connection
    actor.execute();
    thread::sleep_ms(500);
    let host_id = handle.new_host(ConnectionParameters::new("localhost", 8080), 0);
    thread::sleep_ms(500);
    let client_id = handle.new_duplex(ConnectionParameters::new("localhost", 8080), 0);
    thread::sleep_ms(500);

    // Create a message
    let mut msg: [f32; 12] = [0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32];
    for i in 0..12 {
      msg[i] = (i as f32).sin() + (i as f32).cos() * (i as f32);
    }

    handle.process_events();
    let host_client = handle.get_connections(&host_id);
    assert_eq!(host_client.len(), 1);

    // tx/rx
    handle.send(&host_client[0], msg.clone());
    thread::sleep_ms(500);
    handle.process_events();
    let response = handle.get_messages::<[f32; 12]>(&client_id);

    // assert
    assert_eq!(response.is_empty(), false);
    assert_eq!(response[0], msg);

    //cleanup
    handle.shutdown();
  }

  #[test]
  fn can_get_new_connection_notification() {
    let (actor, mut handle) = NetActor::create();

    // Start connection
    actor.execute();
    thread::sleep_ms(500);
    let host_id = handle.new_host(ConnectionParameters::new("localhost", 8080), 0);
    thread::sleep_ms(500);
    let client_id = handle.new_duplex(ConnectionParameters::new("localhost", 8080), 0);
    thread::sleep_ms(500);

    handle.process_events();
    let new_connections = handle.get_connections(&host_id);
    assert_eq!(new_connections.len(), 1);
  }
}
