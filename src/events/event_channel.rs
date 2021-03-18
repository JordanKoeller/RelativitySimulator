pub type ReceiverID = usize;

pub trait EventChannel<E, P, T>
where
  E: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
  P: Sized,
  T: EventTuple<E,P>
{
  fn register_reader(&mut self) -> ReceiverID;
  fn deregister_reader(&mut self, r: &ReceiverID);
  fn subscribe_to(&mut self, receiver: &ReceiverID, event: E);
  fn unsubscribe(&mut self, receiver: &ReceiverID, event: &E);

  fn read(&self, receiver: &ReceiverID) -> Vec<&T>;

  fn for_each<F: FnMut(&T) -> ()>(&self, receiver: &ReceiverID, func: F);

  fn publish(&mut self, event: T);

  fn clear_events(&mut self);

  fn register_with_subs(&mut self, evts: &[E]) -> ReceiverID {
    let id = self.register_reader();
    evts.iter().for_each(move |e| self.subscribe_to(&id, e.clone()));
    id
  }
}

pub trait EventTuple<E, P>
where
  E: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
  P: Sized,
{
  fn deconstruct(self) -> (E, P);
}

impl<E, P> EventTuple<E, P> for (E, P)
where
  E: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
  P: Sized,
{
  fn deconstruct(self) -> (E, P) {
    self
  }
}

impl<E> EventTuple<E, ()> for E
where
  E: Sync + Send + std::hash::Hash + Eq + Clone + std::fmt::Debug + 'static,
{
  fn deconstruct(self) -> (E, ()) {
    (self, ())
  }
}
