use std::collections::HashMap;

use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Guid(Uuid);

impl Guid {
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Component for Guid {
  type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

pub struct  GuidMap(HashMap<Guid, Entity>);

impl GuidMap {
  pub fn get(&self, guid: &Guid) -> Option<&Entity> {
    self.0.get(guid)
  }

  pub fn set(&mut self, guid: Guid, entity: Entity) {
    self.0.insert(guid, entity);
  }

  pub fn remove(&mut self, guid: &Guid) {
    self.0.remove(guid);
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }
}

impl Default for GuidMap {
  fn default() -> Self {
      Self(HashMap::new())
  }
}

#[derive(Default)]
pub struct GuidRegistrySystem {
  receiver_id: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for GuidRegistrySystem {
  type SystemData = (
    Write<'a, GuidMap>,
    ReadStorage<'a, Guid>,
    Entities<'a>,
  );

  fn run(&mut self, (mut guid_map, s_guid, s_ent): Self::SystemData) {
    let mut deleted = specs::hibitset::BitSet::new();
    let mut added = specs::hibitset::BitSet::new();
    for evt in s_guid.channel().read(self.receiver_id.as_mut().unwrap()) {
      match evt {
        ComponentEvent::Inserted(id) => {
          added.add(*id);
        },
        ComponentEvent::Removed(id) => {
          deleted.add(*id);
        },
        ComponentEvent::Modified(_) => {},
      }
    }
    for (guid, entity, _added) in (&s_guid, &s_ent, added).join() {
      guid_map.set(*guid, entity);
    }
    for (guid, _deleted) in (&s_guid, deleted).join() {
      guid_map.remove(guid);
    }
  }

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.receiver_id = Some(world.system_data::<WriteStorage<'_, Guid>>().register_reader());
  }
}

