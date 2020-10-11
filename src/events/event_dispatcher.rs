use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use super::{Event, EventPayload, ReceiverID};

type SubCount = u32;

pub trait EventDispatcher {
  // Some methods for letterboxes, etc.
  fn global_event_inbox_mut(&mut self) -> &mut HashMap<Event, Option<EventPayload>>;
  fn global_subscribed_events_mut(&mut self) -> &mut HashMap<Event, SubCount>;
  fn receiver_inboxes_mut(&mut self) -> &mut HashMap<ReceiverID, HashSet<Event>>;

  fn global_event_inbox(&self) -> &HashMap<Event, Option<EventPayload>>;
  fn global_subscribed_events(&self) -> &HashMap<Event, SubCount>;
  fn receiver_inboxes(&self) -> &HashMap<ReceiverID, HashSet<Event>>;

  // "Implementation" inner workings

  // Add a subscription of a receiver to an event.
  // This adds that event to the receiver's inbox. If the receiver does not
  // already have an inbox, an inbox is created containing the event.
  fn register_subscription(&mut self, receiver: ReceiverID, event: Event) {
    // First add the subscription to the global list of subscribed events
    if let Some(count) = self.global_subscribed_events_mut().get_mut(&event) {
      *count += 1;
    } else {
      self.global_subscribed_events_mut().insert(event.clone(), 1);
    }
    if let Some(inbox) = self.receiver_inboxes_mut().get_mut(&receiver) {
      inbox.insert(event);
    } else {
      self
        .receiver_inboxes_mut()
        .insert(receiver, HashSet::from_iter(vec![event]));
    }
  }

  // "take in" an event/payload and make a memo of it locally if this dispatcher has a reciever
  // subscribed to the event in question.
  fn receive_event(&mut self, event: Event, payload: Option<EventPayload>) {
    if self.global_subscribed_events_mut().contains_key(&event) {
      self.global_event_inbox_mut().insert(event.clone(), payload);
    }
    match &event {
      Event::KeyReleased(k) => {
        self.global_event_inbox_mut().remove(&Event::KeyDown(k.clone()));
      },
      Event::MouseReleased(b) => {
        self.global_event_inbox_mut().remove(&Event::MouseDown(b.clone()));
      }
      _ => {}
    }
  }

  fn refresh(&mut self) {
    self.global_event_inbox_mut().retain(|evt, _| {
      match evt {
        Event::KeyDown(_) => true,
        Event::MouseDown(_) => true,
        _ => false
      }
    });
  }

  // Unsubscribe a receiver from an event
  fn unsubscribe(&mut self, receiver: ReceiverID, event: Event) {
    if let Some(inbox) = self.receiver_inboxes_mut().get_mut(&receiver) {
      inbox.remove(&event);
      if inbox.is_empty() {
        self.receiver_inboxes_mut().remove(&receiver);
      }
      let sub_count = self.global_subscribed_events_mut().get_mut(&event).unwrap();
      *sub_count -= 1;
      if sub_count == &0 {
        self.global_event_inbox_mut().remove(&event);
      }
    } else {
      println!(
        "Tried to remove event {:?} from receiver {}, but receiver was not subscribed to event",
        event, receiver
      );
    }
  }

  fn remove_reciever(&mut self, receiver: ReceiverID) {
    if let Some(inbox) = self.receiver_inboxes_mut().get(&receiver) {
      let subbed_events = inbox.clone();
      for evt in subbed_events.iter() {
        self.unsubscribe(receiver, evt.clone());
      }
    }
  }

  fn consume_inbox(&self, receiver: ReceiverID) -> Vec<(Event, Option<EventPayload>)> {
    if let Some(subbed_events) = self.receiver_inboxes().get(&receiver) {
      subbed_events
        .iter()
        .filter_map(|evt| match self.global_event_inbox().get(evt).as_ref() {
          Some(&payload) => Some((evt.clone(), payload.clone())),
          None => None,
        })
        .collect()
    } else {
      Vec::new()
    }
  }

  fn clear(&mut self) {
    self.global_event_inbox_mut().clear();
  }
  // HELPER FUNCTIONS
}
