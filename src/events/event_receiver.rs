// use std::collections::HashMap;
// use std::cell::{RefMut};

// use utils::*;
// use super::*;

// pub type ReceiverID = usize;

// pub type EventSubscription<T> = fn(&mut T, (Event, Option<EventPayload>));

// pub struct EventReceiver<T> {
//   pub receiver_id: ReceiverID,
//   pub callbacks: HashMap<Event, EventSubscription<T>>,
// }

// impl<T> EventReceiver<T> {
//   // Attribute access

//   // Event Receiver Functionality
//   pub fn subscribe_to(&mut self, evt: Event, callback: EventSubscription<T>, dispatcher: &mut dyn EventDispatcher) {
//     // let dispatcher = self.dispatcher_mut();
//     let id = self.receiver_id;
//     dispatcher.register_subscription(id, evt.clone());
//     self.callbacks.insert(evt, callback);
//   }

//   fn unsubscribe(&mut self, evt: Event, dispatcher: &mut dyn EventDispatcher) {
//     let id = self.receiver_id;
//     dispatcher.unsubscribe(id, evt);
//   }

//   pub fn process_all_events(&mut self, receiver: &mut T, dispatcher: &mut dyn EventDispatcher) {
//     let id = self.receiver_id;
//     let event_queue = dispatcher.consume_inbox(id);
//     for (evt, payload) in event_queue {
//       if let Some(callback) = self.callbacks.get(&evt) {
//         callback(receiver, (evt, payload));
//       }
//     }
//   }

//   fn deregister(&mut self, dispatcher: &mut dyn EventDispatcher) {
//     let id = self.receiver_id;
//     dispatcher.remove_reciever(id);
//   }
// }

// impl<T> Default for EventReceiver<T> {
//   fn default() -> Self {
//     EventReceiver {
//       receiver_id: 0,
//       callbacks: HashMap::new()
//     }
//   }
// }