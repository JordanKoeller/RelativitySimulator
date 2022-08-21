use tokio::net::TcpStream;

use super::{Connection, ConnectionId, ConnectionParameters};

#[derive(Debug)]
pub struct Envelope {
    pub data: Vec<u8>,
    pub connection_id: ConnectionId
}

impl Envelope {
    pub fn new(data: Vec<u8>, connection_id: ConnectionId) -> Self {
        Self {
            data,
            connection_id
        }
    }

    pub fn connection_id(&self) -> &ConnectionId {
        &self.connection_id
    }
}

pub(crate) enum ActorMessage {
    HostConnection(Connection),      // Set up a server host for client to connect to.
    AcceptConnection {stream: TcpStream, host: ConnectionId, new_client: ConnectionParameters, },   // Accept a client trying to connect as a server. Pass back server ConnectionId. Bool indicates if channel ID should be reused.
    FinalizeConnection {stream: TcpStream, connection: Connection, }, // The host accepted your connection. Set up Tx/Rx.
    ConnectTo(Connection),           // Request a connection as a client to a server
    SendMessage(Envelope),       // Message To Send, and connection Id
    DropConnection(ConnectionId),              // Drop connection with id
    Shutdown,                                  // Shutdown the networking actor
}


#[derive(Debug)]
pub enum EgressMessage {
    NewConnection(Connection, ConnectionId), // New connection descriptor and its host connection's Id.
    RxMessage(Envelope), // Data accepted over the wire and the connection Id.
}

impl EgressMessage {
    pub fn new_connection(connection: Connection, id: ConnectionId) -> Self {
        Self::NewConnection(connection, id)
    }

    pub fn new_rx_message(data: Vec<u8>, connection_id: ConnectionId) -> Self {
        Self::RxMessage(Envelope::new(data, connection_id))
    }
}

pub struct NewConnectionMessage {
    connection: Connection,
    parent_connection_id: Option<ConnectionId>,
}

impl NewConnectionMessage {
    pub fn new_subconnection(connection: Connection, parent_connection_id: ConnectionId) -> Self {
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
    pub fn unpack(self) -> (Option<ConnectionId>, Connection) {
        (self.parent_connection_id, self.connection)
    }
}