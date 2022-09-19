use tokio::net::TcpStream;
use super::{GenericConnectionId, Envelope, Connection, ConnectionParameters};


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SocketType {
  TCPClient,
  TCPHost,
  UDP,
  Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectionStatus {
  Unknown,
  Initialized,
  Pending,
  Stable,
  Dropped,
  Uninitialized,
}

#[derive(Debug)]
pub(crate) enum ActorMessage {
  HostConnection(Connection), // Set up a server host for client to connect to.
  AcceptConnection {
    stream: TcpStream,
    host: GenericConnectionId,
    new_client: ConnectionParameters,
  }, // Accept a client trying to connect as a server. Pass back server ConnectionId. Bool indicates if channel ID should be reused.
  FinalizeConnection {
    stream: TcpStream,
    connection: Connection,
  }, // The host accepted your connection. Set up Tx/Rx.
  ConnectTo(Connection),        // Request a connection as a client to a server
  SendMessage(Envelope),        // Message To Send, and connection Id
  DropConnection(GenericConnectionId), // Drop connection with id
  Shutdown,                     // Shutdown the networking actor
}


#[derive(Debug, Clone)]
pub enum NetEvent {
  Connected {
    connection_id: GenericConnectionId,
  },
  Message {
    connection_id: GenericConnectionId,
    envelope: Envelope,
  },
  NewClient {
    host_id: GenericConnectionId,
    client: Connection,
  },
  Disconnected {
    connection_id: GenericConnectionId,
  },
}

impl NetEvent {
  pub fn id(&self) -> GenericConnectionId {
    match self {
      Self::Connected { connection_id } => connection_id.clone(),
      Self::Message { connection_id, .. } => connection_id.clone(),
      Self::NewClient { host_id, .. } => host_id.clone(),
      Self::Disconnected { connection_id } => connection_id.clone(),
    }
  }

  pub fn message(self) -> Option<Envelope> {
    match self {
      Self::Message { envelope, .. } => Some(envelope),
      _ => None,
    }
  }
}
