use specs::prelude::*;

use crate::ecs::{SystemUtilities, WorldProxy};
use crate::events::ReceiverID;

pub trait MonoBehavior<'a> {
  type SystemData: specs::SystemData<'a>;

  #[allow(unused_variables)]
  fn run(&mut self, api: SystemUtilities<'a>, resources: Self::SystemData) {}

  #[allow(unused_variables)]
  fn setup(&mut self, world: WorldProxy) {}

  #[allow(unused_variables)]
  fn destroy(&mut self, api: SystemUtilities<'a>, resources: Self::SystemData) {}
}

pub struct Sys<M>
where
  for<'a> M: MonoBehavior<'a>,
{
  mono_behavior: M,
  receiver_id: Option<ReceiverID>,
}

impl<'a, M> System<'a> for Sys<M>
where
  for<'b> M: MonoBehavior<'b>,
{
  type SystemData = (<M as MonoBehavior<'a>>::SystemData, SystemUtilities<'a>);

  fn run(&mut self, (delegate_resources, utilities): Self::SystemData) {
    self.mono_behavior.run(utilities, delegate_resources);
  }

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    let wp = WorldProxy::new(world);
    self.mono_behavior.setup(wp);
  }
}

impl<M> Default for Sys<M>
where
  for<'a> M: MonoBehavior<'a> + Default,
{
  fn default() -> Self {
    Self {
      mono_behavior: M::default(),
      receiver_id: None,
    }
  }
}

impl<M> Sys<M>
where
  for<'a> M: MonoBehavior<'a>,
{
  pub fn new(mono_behavior: M) -> Self {
    Self {
      mono_behavior,
      receiver_id: None,
    }
  }
}
