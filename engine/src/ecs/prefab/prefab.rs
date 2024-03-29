use std::any::{Any, TypeId};
use std::collections::HashMap;

use std::os::raw::c_void;

use specs::prelude::*;
use specs::storage::AntiStorage;
use specs::world::LazyBuilder;

use crate::ecs::SystemUtilities;

trait ComponentWithStorage<'a>: Component<Storage = AntiStorage<'a>> + Sized {}

pub trait PrefabBuilder {
  type PrefabState;
  fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) -> Entity;
}

pub struct ComponentCache {
  cache: HashMap<TypeId, Box<dyn Any>>,
}

impl Default for ComponentCache {
  fn default() -> Self {
    Self { cache: HashMap::new() }
  }
}

impl ComponentCache {
  pub fn cache<C: Component + Clone>(&mut self, component: C) {
    let id = TypeId::of::<C>();
    self.cache.insert(id, Box::new(component));
  }

  pub fn get_clone<C: Component + Clone>(&self) -> Option<C> {
    let id = TypeId::of::<C>();
    if let Some(component_ref) = self.cache.get(&id) {
      unsafe { Some(cast_clone_ref::<C>(component_ref)) }
    } else {
      None
    }
  }

  pub fn get_or<C: Component + Clone, F: Fn() -> C>(&mut self, component: F) -> C {
    if !self.has_component::<C>() {
      self.cache(component());
    }
    self.get_clone::<C>().unwrap()
  }

  pub fn has_component<C: Component + Clone>(&self) -> bool {
    let id = TypeId::of::<C>();
    self.cache.contains_key(&id)
  }
}

unsafe fn cast_clone_ref<C: Component + Clone>(value: &Box<dyn Any>) -> C {
  let a_ptr = value as *const Box<dyn Any>;
  let c_ptr = a_ptr as *const c_void;
  let component_ptr = c_ptr as *const Box<C>;
  let c_value = &*component_ptr;
  *(c_value.clone())
}

#[cfg(test)]
mod test {

  use specs::prelude::*;
  use specs::{Component, NullStorage};

  use super::*;

  #[derive(Clone, Default, Component)]
  #[storage(NullStorage)]
  struct SomeComponent {
    pub value: u32,
  }

  #[derive(Clone, Default, Component)]
  #[storage(NullStorage)]
  struct SomeOtherComponent {
    pub value1: u32,
    pub value2: f32,
  }

  #[test]
  fn test_component_cache() {
    let mut cache = ComponentCache::default();
    {
      let comp_1 = SomeComponent { value: 3u32 };
      let comp_2 = SomeOtherComponent {
        value1: 4u32,
        value2: 32f32,
      };
      cache.cache(comp_1);
      cache.cache(comp_2);
    }
    let value_1 = cache.get_clone::<SomeComponent>();
    let value_2 = cache.get_clone::<SomeOtherComponent>();
    assert_eq!(value_1.is_some(), true);
    assert_eq!(value_2.is_some(), true);
    let value_1 = value_1.unwrap();
    let value_2 = value_2.unwrap();
    assert_eq!(value_1.value, 3u32);
    assert_eq!(value_2.value1, 4u32);
    assert_eq!(value_2.value2, 32f32);
  }
}
