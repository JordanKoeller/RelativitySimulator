use std::collections::HashMap;
use std::cell::{RefCell, RefMut};

use utils::*;
use super::*;

pub type ReceiverID = usize;

pub type EventSubscription<T> = fn(&mut T, (Event, Option<EventPayload>));

pub struct EventReceiverState<T> {
  dispatcher: MutRef<dyn EventDispatcher>,
  receiver_id: ReceiverID,
  callback_set: HashMap<Event, EventSubscription<T>>,
}

impl<T> EventReceiverState<T>
 where T: Sized {
  pub fn new(dispatcher: MutRef<dyn EventDispatcher>, id: ReceiverID) -> EventReceiverState<T> {
    EventReceiverState {
      dispatcher,
      receiver_id: id,
      callback_set: HashMap::default()
    }
  }
}

pub trait EventReceiver<EventAcceptor> {
  // Attribute access
  fn dispatcher_mut(&mut self) -> RefMut<dyn EventDispatcher>;
  fn receiver_id(&self) -> ReceiverID;
  fn callbacks(&self) -> &HashMap<Event, EventSubscription<Self>>;
  fn callbacks_mut(&mut self) -> &mut HashMap<Event, EventSubscription<Self>>;


  // Event Receiver Functionality
  fn subscribe_to(&mut self, evt: Event, callback: EventSubscription<Self>) {
    // let dispatcher = self.dispatcher_mut();
    let id = self.receiver_id();
    self.dispatcher_mut().register_subscription(id, evt.clone());
    self.callbacks_mut().insert(evt, callback);
  }

  fn unsubscribe(&mut self, evt: Event) {
    let id = self.receiver_id();
    self.dispatcher_mut().unsubscribe(id, evt);
  }

  fn process_all_events(&mut self) {
    let id = self.receiver_id();
    let event_queue = self.dispatcher_mut().consume_inbox(id);
    for (evt, payload) in event_queue {
      if let Some(callback) = self.callbacks().get(&evt) {
        callback(self, (evt, payload));
      }
    }
  }

  // fn process_only_events(&mut self, evts: Vec<Event>);

  fn deregister(&mut self) {
    let id = self.receiver_id();
    self.dispatcher_mut().remove_reciever(id)
  }
}

pub trait WithEventReceiver where Self: Sized {
  fn state(&self) -> &EventReceiverState<Self>;
  fn state_mut(&mut self) -> &mut EventReceiverState<Self>;
}

// impl<V> EventReceiver<V> for WithEventReceiver<V> {
// }

impl <V: WithEventReceiver> EventReceiver<V> for V {
  fn dispatcher_mut(&mut self) -> RefMut<dyn EventDispatcher> {
    self.state_mut().dispatcher.borrow_mut()
  }
  fn receiver_id(&self) -> ReceiverID {
    self.state().receiver_id
  }
  fn callbacks(&self) -> &HashMap<Event, EventSubscription<Self>> {
    &self.state().callback_set
  }
  fn callbacks_mut(&mut self) -> &mut HashMap<Event, EventSubscription<Self>> {
    &mut self.state_mut().callback_set
  }
  
}