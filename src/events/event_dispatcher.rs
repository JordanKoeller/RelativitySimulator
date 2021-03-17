use std::collections::{HashMap, HashSet};

pub type ReceiverID = usize;

type SubCount = u32;

pub struct EventChannelWithPayload<Event, Payload>
where
  Event: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
  Payload: Sized,
{
  registration_id: ReceiverID,
  subscriptions: HashMap<Event, SubCount>,
  receiver_inboxes: Vec<HashSet<Event>>,
  payloads: HashMap<Event, Payload>,
  active_events: HashSet<Event>,
}

impl<Event, Payload> EventChannelWithPayload<Event, Payload>
where
  Event: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
  Payload: Sized,
{
  pub fn register_reader(&mut self) -> ReceiverID {
    let ret = self.registration_id.clone();
    self.registration_id += 1;
    self.receiver_inboxes.push(HashSet::new());
    ret
  }

  #[allow(dead_code)]
  pub fn deregister_reader(&mut self, receiver: &ReceiverID) {
    if let Some(inbox) = self.receiver_inboxes.get(*receiver) {
      for evt in inbox.iter() {
        let mut flag = false;
        let sub = self.subscriptions.get_mut(evt);
        if let Some(count) = sub {
          *count -= 1;
          if count == &0 {
            flag = true;
          }
        }
        if flag {
          self.subscriptions.remove(evt);
        }
      }
    }
    self.receiver_inboxes.remove(*receiver);
  }

  pub fn subscribe_to(&mut self, receiver: &ReceiverID, event: Event) {
    if self.subscriptions.contains_key(&event) {
      if let Some(sub_cnt) = self.subscriptions.get_mut(&event) {
        *sub_cnt += 1;
      }
    // self.subscriptions.get_mut(&event) += 1;
    } else {
      self.subscriptions.insert(event.clone(), 1);
    }
    self.receiver_inboxes[*receiver].insert(event);
  }

  pub fn register_with_subs(&mut self, events: &[Event]) -> ReceiverID {
    let receiver = self.register_reader();
    for evt in events {
      self.subscribe_to(&receiver, evt.clone());
    }
    receiver
  }

  #[allow(dead_code)]
  pub fn unsubscribe(&mut self, receiver: &ReceiverID, event: &Event) {
    // Decrement count of subs on the subs list.
    let mut flag = false;
    let sub = self.subscriptions.get_mut(event);
    if let Some(count) = sub {
      *count -= 1;
      if count == &0 {
        flag = true;
      }
    }
    if flag {
      self.subscriptions.remove(event);
    }
    // Remove the sub from that particular receiver's inbox
    if let Some(inbox) = self.receiver_inboxes.get_mut(*receiver) {
      inbox.remove(event);
    }
  }

  pub fn read(&self, receiver: &ReceiverID) -> impl Iterator<Item = (&Event, Option<&Payload>)> + '_ {
    self.receiver_inboxes[*receiver].iter().filter_map(move |v| {
      let ret = self.active_events.get(v);
      match ret {
        Some(evt) => {
          let payload = self.payloads.get(evt);
          // Some((evt, payload))
          match payload {
            Some(pld) => Some((evt, Some(pld))),
            None => Some((evt, None)),
          }
        }
        None => None,
      }
    })
  }

  pub fn publish(&mut self, event: Event) {
    if self.subscriptions.contains_key(&event) {
      self.active_events.insert(event.clone());
    }
  }

  pub fn publish_with_payload(&mut self, event: Event, payload: Payload) {
    if self.subscriptions.contains_key(&event) {
      self.active_events.insert(event.clone());
      self.payloads.insert(event, payload);
    }
  }

  pub fn clear_events(&mut self) {
    self.active_events.clear();
    self.payloads.clear();
  }
}

impl<Event, Payload> Default for EventChannelWithPayload<Event, Payload>
where
  Event: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
  Payload: Sized,
{
  fn default() -> Self {
    Self {
      registration_id: 0,
      subscriptions: HashMap::new(),
      receiver_inboxes: Vec::with_capacity(100),
      active_events: HashSet::new(),
      payloads: HashMap::new(),
    }
  }
}

pub type EventChannel<Event>
where
  Event: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
= EventChannelWithPayload<Event, ()>;

// pub struct EventChannel<Event: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static> {
//   registration_id: ReceiverID,
//   subscriptions: HashMap<Event, SubCount>,
//   receiver_inboxes: Vec<HashSet<Event>>,
//   active_events: HashSet<Event>,
// }

// impl<Event> EventChannel<Event>
// where
//   Event: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
// {
//   pub fn register_reader(&mut self) -> ReceiverID {
//     let ret = self.registration_id.clone();
//     self.registration_id += 1;
//     self.receiver_inboxes.push(HashSet::new());
//     ret
//   }

//   #[allow(dead_code)]
//   pub fn deregister_reader(&mut self, receiver: &ReceiverID) {
//     if let Some(inbox) = self.receiver_inboxes.get(*receiver) {
//       for evt in inbox.iter() {
//         let mut flag = false;
//         let sub = self.subscriptions.get_mut(evt);
//         if let Some(count) = sub {
//           *count -= 1;
//           if count == &0 {
//             flag = true;
//           }
//         }
//         if flag {
//           self.subscriptions.remove(evt);
//         }
//       }
//     }
//     self.receiver_inboxes.remove(*receiver);
//   }

//   pub fn subscribe_to(&mut self, receiver: &ReceiverID, event: Event) {
//     if self.subscriptions.contains_key(&event) {
//       if let Some(sub_cnt) = self.subscriptions.get_mut(&event) {
//         *sub_cnt += 1;
//       }
//     // self.subscriptions.get_mut(&event) += 1;
//     } else {
//       self.subscriptions.insert(event.clone(), 1);
//     }
//     self.receiver_inboxes[*receiver].insert(event);
//   }

//   pub fn register_with_subs(&mut self, events: &[Event]) -> ReceiverID {
//     let receiver = self.register_reader();
//     for evt in events {
//       self.subscribe_to(&receiver, evt.clone());
//     }
//     receiver
//   }

//   #[allow(dead_code)]
//   pub fn unsubscribe(&mut self, receiver: &ReceiverID, event: &Event) {
//     // Decrement count of subs on the subs list.
//     let mut flag = false;
//     let sub = self.subscriptions.get_mut(event);
//     if let Some(count) = sub {
//       *count -= 1;
//       if count == &0 {
//         flag = true;
//       }
//     }
//     if flag {
//       self.subscriptions.remove(event);
//     }
//     // Remove the sub from that particular receiver's inbox
//     if let Some(inbox) = self.receiver_inboxes.get_mut(*receiver) {
//       inbox.remove(event);
//     }
//   }

//   pub fn read(&self, receiver: &ReceiverID) -> impl Iterator<Item = &Event> + '_ {
//     self.receiver_inboxes[*receiver].iter().filter_map(move |v| {
//       let ret = self.active_events.get(v);
//       ret
//     })
//   }

//   pub fn publish(&mut self, event: Event) {
//     if self.subscriptions.contains_key(&event) {
//       self.active_events.insert(event);
//     }
//   }

//   pub fn clear_events(&mut self) {
//     self.active_events.clear();
//   }
// }

// impl<Event> Default for EventChannel<Event>
// where
//   Event: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
// {
//   fn default() -> Self {
//     Self {
//       registration_id: 0,
//       subscriptions: HashMap::new(),
//       receiver_inboxes: Vec::with_capacity(100),
//       active_events: HashSet::new(),
//     }
//   }
// }

// pub trait WithEventChannel {

// }
