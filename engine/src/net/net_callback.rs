use specs::prelude::*;

use crate::{
  ecs::{MonoBehavior, SystemUtilities},
  events::{EventChannel, StatefulEventChannel},
};

use super::{Connection, ConnectionId, ConnectionStatus, Envelope, NetEvent};

// Methods to be implemented
trait NetCallback<Args> {
  /**
   * Callback ran when a connection is made.
   */
  fn on_connect(&self, _args: &Args) {}

  /**
   * Callback that runs when a new message is found
   */
  fn on_message(&self, _message: &Envelope, _args: &Args) {}

  /**
   * Callback ran when a connection is dropped
   */
  fn on_disconnect(&self, _args: &Args) {}

  fn id(&self) -> Option<ConnectionId>;

  fn execute<'a>(&self, events: &Read<'a, StatefulEventChannel<ConnectionId, NetEvent>>, args: &Args) {
    if let Some(id) = self.id() {
      for (_, net_event) in events.read(&id.receiver()).into_iter() {
        match net_event {
          NetEvent::Connected => self.on_connect(args),
          NetEvent::Message(message) => self.on_message(message, args),
          NetEvent::Disconnected => self.on_disconnect(args),
        }
      }
    }
  }
}

#[derive(Default)]
pub struct ConnectionCallback<'a, M>
where
  M: MonoBehavior<'a>,
{
  handle_connect: Option<Box<dyn Fn(&(&SystemUtilities<'a>, &<M as MonoBehavior<'a>>::SystemData)) -> ()>>,
  handle_message:
    Option<Box<dyn Fn(&Envelope, &(&SystemUtilities<'a>, &<M as MonoBehavior<'a>>::SystemData)) -> ()>>,
  handle_disconnect: Option<Box<dyn Fn(&(&SystemUtilities<'a>, &<M as MonoBehavior<'a>>::SystemData)) -> ()>>,
  connection_id: Option<ConnectionId>,
  phantom: std::marker::PhantomData<&'a M>,
}

impl<'a, M> NetCallback<(&SystemUtilities<'a>, &<M as MonoBehavior<'a>>::SystemData)>
  for ConnectionCallback<'a, M>
where
  M: MonoBehavior<'a>,
{
  fn on_connect(&self, args: &(&SystemUtilities<'a>, &<M as MonoBehavior<'a>>::SystemData)) {
    if let Some(handler) = &self.handle_connect {
      handler(args);
    }
  }

  fn on_message(
    &self,
    message: &Envelope,
    args: &(&SystemUtilities<'a>, &<M as MonoBehavior<'a>>::SystemData),
  ) {
    if let Some(handler) = &self.handle_message {
      handler(message, args);
    }
  }

  fn on_disconnect(&self, args: &(&SystemUtilities<'a>, &<M as MonoBehavior<'a>>::SystemData)) {
    if let Some(handler) = &self.handle_disconnect {
      handler(args);
    }
  }

  fn id(&self) -> Option<ConnectionId> {
    self.connection_id
  }
}

impl<'a, M> ConnectionCallback<'a, M>
where
  M: MonoBehavior<'a>,
{
  pub fn new(connection_id: ConnectionId) -> Self {
    Self {
      handle_connect: None,
      handle_disconnect: None,
      handle_message: None,
      phantom: std::marker::PhantomData::default(),
      connection_id: Some(connection_id),
    }
  }

  pub fn with_connect_handler<
    F1: Fn(&(&SystemUtilities<'a>, &<M as MonoBehavior<'a>>::SystemData)) -> () + 'static,
  >(
    &mut self,
    handle_connect: F1,
  ) {
      self.handle_connect = Some(Box::from(handle_connect));
  }

  pub fn with_message_handler<
    F1: Fn(&Envelope, &(&SystemUtilities<'a>, &<M as MonoBehavior<'a>>::SystemData)) -> () + 'static,
  >(
    &mut self,
    handle_message: F1,
  )  {
      self.handle_message = Some(Box::from(handle_message));
  }

  pub fn with_disconnect_handler<
    F1: Fn(&(&SystemUtilities<'a>, &<M as MonoBehavior<'a>>::SystemData)) -> () + 'static,
  >(
    &mut self,
    handle_disconnect: F1,
  ) {
    self.handle_disconnect = Some(Box::from(handle_disconnect));
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use specs::prelude::*;
  use crate::ecs::WorldProxy;

  #[derive(Default)]
  struct TestingSystem<'a> {
    handler: ConnectionCallback<'a, Self>,
    data: Vec<usize>,
  }

  impl<'a> MonoBehavior<'a> for TestingSystem<'a> {
    type SystemData = Read<'a, StatefulEventChannel<ConnectionId, NetEvent>>;

    fn run(&mut self, api: SystemUtilities<'a>, data: Self::SystemData) {
      self.handler.execute(&data, &(&api, &data));
    }

    fn setup(&mut self, _world: WorldProxy) {
      self.handler.with_connect_handler(|(_api, _data)| {
      });
    }
  }

  #[test]
  fn test_callback() {
    let _system = TestingSystem::default();
  }
}
