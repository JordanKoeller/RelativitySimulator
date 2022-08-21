use specs::{Component, VecStorage};

use crate::events::ReceiverID;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ConnectionId {
    channel: usize, // Which TCP/UDP connection this message is for
    receiver: ReceiverID,
}

impl ConnectionId {
    pub fn zero() -> Self {
        Self {
            channel: 0,
            receiver: 0,
        }
    }

    pub fn new(receiver_id: ReceiverID, channel: usize) -> Self {
        Self {
            receiver: receiver_id,
            channel,
        }
    }

    pub fn channel(&self) -> usize {
        self.channel
    }

    pub fn receiver(&self) -> ReceiverID {
        self.receiver
    }
}

impl Component for ConnectionId {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct Connection {
    uri: String,
    port: u32,
    connection_type: SocketType,
    connection_id: ConnectionId,
}

impl Connection {
    pub fn new(parameters: ConnectionParameters, id: ConnectionId) -> Self {
        Self {
            uri: parameters.uri,
            port: parameters.port,
            connection_type: parameters.connection_type,
            connection_id: id,
        }
    }

    pub fn id(&self) -> ConnectionId {
        self.connection_id.clone()
    }

    pub fn connection_string(&self) -> String {
        format!("{}:{}", self.uri, self.port)
    }

    pub fn connection_type(&self) -> SocketType {
        self.connection_type
    }
}

pub struct ConnectionParameters {
    uri: String,
    port: u32,
    connection_type: SocketType,
}

impl ConnectionParameters {
    pub fn new_tcp_client(uri: &str, port: u32) -> Self {
        Self {
            uri: uri.to_string(),
            port,
            connection_type: SocketType::TCPClient,
        }
    }

    pub fn new_tcp_host(uri: &str, port: u32) -> Self {
        Self {
            uri: uri.to_string(),
            port,
            connection_type: SocketType::TCPServer,
        }
    }

    pub fn new_udp(uri: &str, port: u32) -> Self {
        Self {
            uri: uri.to_string(),
            port,
            connection_type: SocketType::UDP,
        }
    }

    pub fn to_connection(self, connection_id: ConnectionId) -> Connection {
        Connection {
            uri: self.uri,
            port: self.port,
            connection_type: self.connection_type,
            connection_id,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SocketType {
    TCPClient,
    TCPServer,
    UDP,
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
