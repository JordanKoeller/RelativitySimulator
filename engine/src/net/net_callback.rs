use serde::{de::DeserializeOwned, Deserialize, Serialize};
use specs::prelude::*;

use crate::{
  ecs::{MonoBehavior, SystemUtilities},
  events::{EventChannel, StatefulEventChannel},
};

use super::{
  Connection, ConnectionId, ConnectionStatus, DuplexConnectionId, Envelope, GenericConnectionId, HostConnectionId,
  NetActorHandle, NetEvent,
};

#[derive(Default)]
pub struct HostContext {
  id: Option<HostConnectionId>,
}

impl HostContext {

  pub fn new(id: HostConnectionId) -> Self {
    Self {
      id: Some(id)
    }
  }

  pub fn set_id(&mut self, id: HostConnectionId) {
    self.id = Some(id);
  }

  pub fn on_connect<F>(&self, net_opt: &Option<NetActorHandle>, mut func: F)
  where F: FnMut(DuplexConnectionId) -> () {
    if let Some(id) = self.id {
      if let Some(net) = net_opt {
        for cx in net.get_connections(&id).into_iter() {
          func(cx);
        }
      }
    }
  }
}


#[derive(Default)]
pub struct DuplexContext {
  id: Option<DuplexConnectionId>,
}

impl DuplexContext {

  pub fn new(id: DuplexConnectionId) -> Self {
    Self {
      id: Some(id)
    }
  }

  pub fn set_id(&mut self, id: DuplexConnectionId) {
    self.id = Some(id);
  }

  pub fn on_connect<F>(&self, net_opt: &Option<NetActorHandle>, mut func: F)
  where F: FnMut(DuplexConnectionId) -> () {
    if let Some(id) = self.id {
      if let Some(net) = net_opt {
        for msg in net.get_events(&id).into_iter() {
          match msg {
            NetEvent::Connected { .. } => func(id),
            _ => {}
          }
        }
      }
    }
  }

  pub fn on_message<M, F>(&self, net_opt: &Option<NetActorHandle>, mut func: F)
  where
    M: Serialize + DeserializeOwned,
    F: FnMut(M) -> (),
  {
    if let Some(id) = self.id {
      if let Some(net) = net_opt {
        for msg in net.get_messages(&id).into_iter() {
          func(msg);
        }
      }
    }
  }

  pub fn send<M: Serialize + DeserializeOwned>(&self, net_opt: &Option<NetActorHandle>, payload: M) {
    if let Some(id) = self.id {
      if let Some(net) = net_opt {
        net.send(&id, payload);
      }
    }
  }
}