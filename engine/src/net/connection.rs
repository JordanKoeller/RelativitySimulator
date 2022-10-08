use specs::{Component, VecStorage};

use crate::events::ReceiverId;
use super::{GenericConnectionId, ConnectionId, SocketType};

#[derive(Debug, Clone)]
pub struct Connection {
  uri: String,
  port: u32,
  connection_id: GenericConnectionId,
}

impl Connection {
  pub fn new(parameters: ConnectionParameters, id: GenericConnectionId) -> Self {
    Self {
      uri: parameters.uri,
      port: parameters.port,
      connection_id: id,
    }
  }

  pub fn id(&self) -> GenericConnectionId {
    self.connection_id.clone()
  }

  pub fn connection_string(&self) -> String {
    format!("{}:{}", self.uri, self.port)
  }

  pub fn connection_type(&self) -> SocketType {
    *self.connection_id.socket_type()
  }
}

#[derive(Debug)]
pub struct ConnectionParameters {
  uri: String,
  port: u32,
}

impl ConnectionParameters {
  pub fn new(uri: &str, port: u32) -> Self {
    Self {
      uri: uri.to_string(),
      port,
    }
  }


  pub fn to_connection(self, connection_id: GenericConnectionId) -> Connection {
    Connection {
      uri: self.uri,
      port: self.port,
      connection_id,
    }
  }
}
