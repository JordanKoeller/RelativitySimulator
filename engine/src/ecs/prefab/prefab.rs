use specs::prelude::*;
use specs::world::LazyBuilder;

pub trait PrefabBuilder {
    type PrefabState;
    fn build<'a>(&self, entity_builder: LazyBuilder<'a>, state: Self::PrefabState) -> LazyBuilder<'a>;
}