use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::{ConnectionStatus, DuplexConnectionId, HostConnectionId};
// TODO: Convert this to use a LRU cache so that connection IDs can be reused

pub struct ConnectionManager {
  counter: AtomicUsize,
  channel_statuses: RwLock<Vec<ConnectionStatus>>,
  host_clients: RwLock<HashMap<HostConnectionId, Vec<DuplexConnectionId>>>,
}

impl ConnectionManager {
  pub fn new() -> Self {
    Self {
      counter: AtomicUsize::new(0usize),
      channel_statuses: RwLock::default(),
      host_clients: RwLock::default(),
    }
  }

  pub fn set_channel_status(&self, id: usize, status: ConnectionStatus) {
    self
      .channel_statuses
      .write()
      .map(|mut statuses| {
        statuses[id] = status;
      })
      .ok();
  }

  pub fn get_channel_status(&self, id: usize) -> ConnectionStatus {
    match self.channel_statuses.read().unwrap().get(id) {
      Some(status) => *status,
      None => ConnectionStatus::Uninitialized,
    }
  }

  pub fn get_clients_of(&self, host: &HostConnectionId) -> Vec<DuplexConnectionId> {
    let cx_opts = self
      .host_clients
      .read()
      .map(|hosts| hosts.get(host).map(|v| v.clone()))
      .ok();
    if let Some(cx) = cx_opts.flatten() {
      cx.clone()
    } else {
      Vec::new()
    }
  }

  pub fn set_client(&self, host: &HostConnectionId, client: DuplexConnectionId) -> usize {
    self
      .host_clients
      .write()
      .map(|mut hosts| {
        if hosts.contains_key(host) {
          hosts.get_mut(host).unwrap().push(client);
          hosts.get(host).unwrap().len()
        } else {
          hosts.insert(*host, vec![client]);
          1
        }
      })
      .ok()
      .unwrap_or(0)
  }

  pub fn get_channel_id(&self) -> usize {
    self
      .channel_statuses
      .write()
      .map(|mut statuses| statuses.push(ConnectionStatus::Initialized))
      .ok();
    self.counter.fetch_add(1usize, Ordering::SeqCst)
  }

  pub fn drop_channel(&self, channel: usize) {
    // TODO: once I have a LRU cache I can intelligently drop channels.
    self
      .channel_statuses
      .write()
      .map(|mut statuses| {
        statuses[channel] = ConnectionStatus::Dropped;
      })
      .ok();
  }
}
