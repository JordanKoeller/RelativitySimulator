use either::Either;
use lazy_static::lazy::Lazy;
use specs::hibitset::{BitSet, BitSetLike};
use specs::prelude::*;
use specs::world::Index;
use specs::world::LazyBuilder;
use std::ops::Deref;
use std::rc::Rc;

use super::EntityTree;
use crate::datastructures::{NTreeNode, ReducableTree, NTree};
use crate::utils::{GetMutRef, MutRef, Ref, Swap};

pub struct EntityTreeBuilder<'a, 'b: 'a> {
  entities: &'b Entities<'a>,
  lazy_update: &'b Read<'a, LazyUpdate>,
  builder: Swap<LazyBuilder<'a>>,
}

impl<'a, 'b: 'a> EntityTreeBuilder<'a, 'b> {
  pub fn new(entities: &'a Entities<'a>, lazy_update: &'b Read<'a, LazyUpdate>) -> Self {
    Self {
      entities,
      lazy_update,
      builder: Swap::new(lazy_update.create_entity(&entities))
    }
  }

  pub fn with<C: Component + Send + Sync>(&mut self, component: C) -> &mut Self {
    self.builder.swap_with(|builder| builder.with(component));
    self
  }
}

impl<'a, 'b: 'a> NTreeNode for EntityTreeBuilder<'a, 'b> {
  fn spawn_child(&self) -> Self {
      Self::new(&self.entities, &self.lazy_update)
  }
}

impl<'a, 'b: 'a> ReducableTree for EntityTreeBuilder<'a, 'b> {
  type Output = Entity;

  fn reduce<I: IntoIterator<Item = Self::Output>>(mut self, children: I) -> Self::Output {
      let mut accumulator = BitSet::new();
      for child in children {
        accumulator.add(child.id());
      }
      if !accumulator.is_empty() {
        self.with(EntityTree::new(accumulator));
      }
      self.builder.unwrap().build()
  }
}

pub type EntityBuilder<'a, 'b> = NTree<EntityTreeBuilder<'a, 'b>>;