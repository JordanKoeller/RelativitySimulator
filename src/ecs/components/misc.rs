use specs::prelude::*;
use specs::prelude::{Component, HashMapStorage};

pub struct EntityTargetComponent {
  pub entity: Option<Entity>,
}

impl Default for EntityTargetComponent {
  fn default() -> Self {
    Self {
      entity: None
    }
  }
}

impl Component for EntityTargetComponent {
  type Storage = HashMapStorage<Self>;
}