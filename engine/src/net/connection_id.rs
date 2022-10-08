use specs::{Component, VecStorage};

use super::SocketType;
use crate::events::ReceiverId;

pub type ChannelId = usize;

pub trait ConnectionId {
  fn receiver(&self) -> &ReceiverId;

  fn channel(&self) -> &ChannelId;

  fn socket_type(&self) -> &SocketType;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct GenericConnectionId {
  channel: usize, // Which TCP/UDP connection this message is for
  receiver: ReceiverId,
  socket_type: SocketType,
}

impl GenericConnectionId {
  pub fn zero() -> Self {
    Self {
      channel: 0,
      receiver: 0,
      socket_type: SocketType::Unknown,
    }
  }

  pub fn new(receiver_id: ReceiverId, channel: usize, socket_type: SocketType) -> Self {
    Self {
      receiver: receiver_id,
      channel,
      socket_type,
    }
  }
}

impl ConnectionId for GenericConnectionId {
  fn channel(&self) -> &ChannelId {
    &self.channel
  }

  fn receiver(&self) -> &ReceiverId {
    &self.receiver
  }

  fn socket_type(&self) -> &SocketType {
    &self.socket_type
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct HostConnectionId {
  channel: ChannelId,
  receiver: ReceiverId,
}

impl HostConnectionId {
  pub fn new(receiver: ReceiverId, channel: ChannelId) -> Self {
    Self { channel, receiver }
  }
}

impl ConnectionId for HostConnectionId {
  fn channel(&self) -> &ChannelId {
    &self.channel
  }

  fn receiver(&self) -> &ReceiverId {
    &self.receiver
  }

  fn socket_type(&self) -> &SocketType {
    &SocketType::TCPHost
  }
}

impl Component for HostConnectionId {
  type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DuplexConnectionId {
  channel: ChannelId,
  receiver: ReceiverId,
}

impl DuplexConnectionId {
  pub fn new(receiver: ReceiverId, channel: ChannelId) -> Self {
    Self { channel, receiver }
  }
}

impl ConnectionId for DuplexConnectionId {
  fn channel(&self) -> &ChannelId {
    &self.channel
  }

  fn receiver(&self) -> &ReceiverId {
    &self.receiver
  }

  fn socket_type(&self) -> &SocketType {
    &SocketType::TCPClient
  }
}

impl Component for DuplexConnectionId {
  type Storage = VecStorage<Self>;
}

impl From<GenericConnectionId> for HostConnectionId {
  fn from(v: GenericConnectionId) -> Self {
    Self {
      channel: *v.channel(),
      receiver: *v.receiver(),
    }
  }
}

impl From<GenericConnectionId> for DuplexConnectionId {
  fn from(v: GenericConnectionId) -> Self {
    Self {
      channel: *v.channel(),
      receiver: *v.receiver(),
    }
  }
}

impl From<HostConnectionId> for DuplexConnectionId {
  fn from(v: HostConnectionId) -> Self {
    Self {
      channel: *v.channel(),
      receiver: *v.receiver(),
    }
  }
}

impl From<HostConnectionId> for GenericConnectionId {
  fn from(v: HostConnectionId) -> Self {
    Self {
      channel: *v.channel(),
      receiver: *v.receiver(),
      socket_type: *v.socket_type(),
    }
  }
}

impl From<DuplexConnectionId> for GenericConnectionId {
  fn from(v: DuplexConnectionId) -> Self {
    Self {
      channel: *v.channel(),
      receiver: *v.receiver(),
      socket_type: *v.socket_type(),
    }
  }
}
