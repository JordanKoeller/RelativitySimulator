use specs::prelude::*;
use specs::world::LazyBuilder;

pub trait PrefabBuilder {
    type PrefabState;
    fn build<B: Builder>(&self, entity_builder: B, state: Self::PrefabState) -> B;
}