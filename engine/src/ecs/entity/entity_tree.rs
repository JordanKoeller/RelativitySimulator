use lazy_static::lazy::Lazy;
use specs::hibitset::{BitSet, BitSetLike};
use specs::prelude::*;
use specs::world::Index;
use specs::world::LazyBuilder;
use std::ops::Deref;
use std::rc::Rc;

use crate::utils::{GetMutRef, MutRef};

#[derive(Clone, Debug, Default)]
pub struct EntityTree {
  entities: BitSet,
}

impl EntityTree {
  pub(crate) fn new(entities: BitSet) -> Self {
    Self { entities }
  }

  pub fn add(&mut self, entity: Entity) {
    self.entities.add(entity.id());
  }
}

impl Component for EntityTree {
  type Storage = VecStorage<Self>;
}

impl Join for EntityTree {
  type Type = <BitSet as Join>::Type;
  type Value = <BitSet as Join>::Value;
  type Mask = BitSet;

  unsafe fn open(self) -> (Self::Mask, Self::Value) {
    self.entities.open()
  }

  unsafe fn get(value: &mut Self::Value, id: Index) -> Self::Type {
    BitSet::get(value, id)
  }
}


pub(crate) struct EntityTreeRootLabel;

impl Component for EntityTreeRootLabel {
  type Storage = VecStorage<Self>;
}