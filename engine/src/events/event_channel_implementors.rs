use super::event_channel::*;
use std::collections::{HashMap, HashSet};

type SubCount = u32;

pub struct StatelessEventChannel<E>
where
  E: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
{
  curr_id: ReceiverId,
  subbed_events: HashMap<E, SubCount>,
  inboxes: Vec<HashSet<E>>,
  active_events: HashSet<E>,
}

impl<E> Default for StatelessEventChannel<E>
where
  E: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
{
  fn default() -> Self {
    Self {
      curr_id: 0,
      subbed_events: HashMap::default(),
      inboxes: Vec::default(),
      active_events: HashSet::default(),
    }
  }
}

impl<E> EventChannel<E, (), E> for StatelessEventChannel<E>
where
  E: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
{
  fn register_reader(&mut self) -> ReceiverId {
    let ret = self.curr_id.clone();
    self.curr_id += 1;
    self.inboxes.push(HashSet::new());
    ret
  }
  fn deregister_reader(&mut self, r: &ReceiverId) {
    if let Some(inbox) = self.inboxes.get(*r) {
      for evt in inbox.iter() {
        let mut flag = false;
        let sub = self.subbed_events.get_mut(evt);
        if let Some(count) = sub {
          *count -= 1;
          if count == &0 {
            flag = true;
          }
        }
        if flag {
          self.subbed_events.remove(evt);
        }
      }
    }
    self.inboxes.remove(*r);
  }

  fn subscribe_to(&mut self, receiver: &ReceiverId, event: E) {
    if self.subbed_events.contains_key(&event) {
      if let Some(sub_cnt) = self.subbed_events.get_mut(&event) {
        *sub_cnt += 1;
      }
    // self.subscriptions.get_mut(&event) += 1;
    } else {
      self.subbed_events.insert(event.clone(), 1);
    }
    self.inboxes[*receiver].replace(event);
  }
  fn unsubscribe(&mut self, receiver: &ReceiverId, event: &E) {
    // Decrement count of subs on the subs list.
    let mut flag = false;
    let sub = self.subbed_events.get_mut(event);
    if let Some(count) = sub {
      *count -= 1;
      if count == &0 {
        flag = true;
      }
    }
    if flag {
      self.subbed_events.remove(event);
    }
    // Remove the sub from that particular receiver's inbox
    if let Some(inbox) = self.inboxes.get_mut(*receiver) {
      inbox.remove(event);
    }
  }

  fn read(&self, receiver: &ReceiverId) -> Vec<&E> {
    self.inboxes[*receiver]
      .iter()
      .filter_map(move |v| self.active_events.get(v))
      .collect()
  }

  fn for_each<F: FnMut(&E) -> ()>(&self, receiver: &ReceiverId, func: F) {
    self.inboxes[*receiver]
      .iter()
      .filter_map(|e| self.active_events.get(e))
      .for_each(func);
  }

  fn publish(&mut self, event: E) {
    if self.subbed_events.contains_key(&event) {
      self.active_events.replace(event);
    }
  }

  fn clear_events(&mut self) {
    self.active_events.clear();
  }
}

pub struct StatefulEventChannel<E, P>
where
  E: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
  P: Sized,
{
  curr_id: ReceiverId,
  subbed_events: HashMap<E, SubCount>,
  inboxes: Vec<HashSet<E>>,
  active_events: HashMap<E, Vec<(E, P)>>,
}

impl<E, P> Default for StatefulEventChannel<E, P>
where
  E: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
  P: Sized,
{
  fn default() -> Self {
    Self {
      curr_id: 0,
      subbed_events: HashMap::default(),
      inboxes: Vec::default(),
      active_events: HashMap::default(),
    }
  }
}

impl<E, P> EventChannel<E, P, (E, P)> for StatefulEventChannel<E, P>
where
  E: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
  P: Sized,
{
  fn register_reader(&mut self) -> ReceiverId {
    let ret = self.curr_id.clone();
    self.curr_id += 1;
    self.inboxes.push(HashSet::new());
    ret
  }

  fn deregister_reader(&mut self, r: &ReceiverId) {
    if let Some(inbox) = self.inboxes.get(*r) {
      for evt in inbox.iter() {
        let mut flag = false;
        let sub = self.subbed_events.get_mut(evt);
        if let Some(count) = sub {
          *count -= 1;
          if count == &0 {
            flag = true;
          }
        }
        if flag {
          self.subbed_events.remove(evt);
        }
      }
    }
    self.inboxes.remove(*r);
  }

  fn subscribe_to(&mut self, receiver: &ReceiverId, event: E) {
    if self.subbed_events.contains_key(&event) {
      if let Some(sub_cnt) = self.subbed_events.get_mut(&event) {
        *sub_cnt += 1;
      }
    // self.subscriptions.get_mut(&event) += 1;
    } else {
      self.subbed_events.insert(event.clone(), 1);
    }
    self.inboxes[*receiver].replace(event);
  }
  fn unsubscribe(&mut self, receiver: &ReceiverId, event: &E) {
    // Decrement count of subs on the subs list.
    let mut flag = false;
    let sub = self.subbed_events.get_mut(event);
    if let Some(count) = sub {
      *count -= 1;
      if count == &0 {
        flag = true;
      }
    }
    if flag {
      self.subbed_events.remove(event);
    }
    // Remove the sub from that particular receiver's inbox
    if let Some(inbox) = self.inboxes.get_mut(*receiver) {
      inbox.remove(event);
    }
  }

  fn read(&self, receiver: &ReceiverId) -> Vec<&(E, P)> {
    if self.inboxes.len() <= *receiver {
      Vec::new()
    } else {
      self.inboxes[*receiver]
        .iter()
        .filter_map(move |subbed_evt| self.active_events.get(subbed_evt))
        .flatten()
        .collect()
    }
  }

  fn for_each<F: FnMut(&(E, P)) -> ()>(&self, receiver: &ReceiverId, func: F) {
    self.inboxes[*receiver]
      .iter()
      .filter_map(move |subbed_evt| self.active_events.get(subbed_evt))
      .flatten()
      .for_each(func);
  }

  fn publish(&mut self, event: (E, P)) {
    let (evt, payload) = event.deconstruct();
    if self.subbed_events.contains_key(&evt) {
      match self.active_events.get_mut(&evt) {
        Some(evts) => evts.push((evt, payload)),
        None => {
          self.active_events.insert(evt.clone(), vec![(evt, payload)]);
        }
      }
    }
  }

  fn clear_events(&mut self) {
    self.active_events.clear();
  }
}
