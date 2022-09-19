use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

use super::{Connection, GenericConnectionId, ConnectionParameters};

#[derive(Debug, Clone)]
pub struct Envelope {
  pub data: Vec<u8>,
  pub connection_id: GenericConnectionId,
}

impl Envelope {
  pub fn new(data: Vec<u8>, connection_id: GenericConnectionId) -> Self {
    Self { data, connection_id }
  }

  pub fn connection_id(&self) -> &GenericConnectionId {
    &self.connection_id
  }

  pub fn get_as<T>(&self) -> T
  where
    for<'a> T: Serialize + Deserialize<'a>,
  {
    serde_json::from_slice(&self.data).ok().unwrap()
  }
}



#[derive(Debug)]
pub enum EgressMessage {
  NewConnection(Connection, GenericConnectionId), // New connection descriptor and its host connection's Id.
  RxMessage(Envelope),                     // Data accepted over the wire and the connection Id.
}

impl EgressMessage {
  pub fn new_connection(connection: Connection, id: GenericConnectionId) -> Self {
    Self::NewConnection(connection, id)
  }

  pub fn new_rx_message(data: Vec<u8>, connection_id: GenericConnectionId) -> Self {
    Self::RxMessage(Envelope::new(data, connection_id))
  }
}

pub struct NewConnectionMessage {
  connection: Connection,
  parent_connection_id: Option<GenericConnectionId>,
}

impl NewConnectionMessage {
  pub fn new_subconnection(connection: Connection, parent_connection_id: GenericConnectionId) -> Self {
    Self {
      connection,
      parent_connection_id: Some(parent_connection_id),
    }
  }

  pub fn new(connection: Connection) -> Self {
    Self {
      connection,
      parent_connection_id: None,
    }
  }

  // Return parent connection's ID and the connection itself
  pub fn unpack(self) -> (Option<GenericConnectionId>, Connection) {
    (self.parent_connection_id, self.connection)
  }
}
