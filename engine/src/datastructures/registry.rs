use crossbeam_queue::SegQueue;
use specs::Builder;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait RegistryItem {
    type K: Eq + PartialEq + Hash + Clone + Sync + Send;
    type V: Sized;

    fn build(self) -> Self::V;
    fn key(&self) -> Self::K;
    fn is_buildable(&self) -> bool;
}

pub trait Registry<KVB>
where
    KVB: RegistryItem,
{
    fn create() -> Self;
    fn get_registry_id(&self, lookup_name: &str) -> Option<KVB::K>;
    fn fetch(&self, registry_id: &KVB::K) -> Option<RwLockReadGuard<'_, KVB::V>>;

    fn fetch_mut(&self, registry_id: &KVB::K) -> Option<RwLockWriteGuard<'_, KVB::V>>;

    // Schedules for building and insertion later, but gives back an ID now.
    // If the same lookup_name is enqueued twice before the lazy queue has been drained
    // This must still give back a key and skip enqueuing twice.
    fn enqueue_builder<B: Into<KVB>>(&self, lookup_name: &str, builder: B) -> KVB::K;

    // Drains the queue, saving everything into the registry
    fn flush(&mut self);
}

#[derive(Default)]
pub struct GenericRegistry<KVB>
where
    KVB: RegistryItem,
{
    name_lookup: RwLock<HashMap<String, KVB::K>>,
    value_lookup: HashMap<KVB::K, RwLock<KVB::V>>,
    inbox: SegQueue<KVB>,
}

impl<KVB> Registry<KVB> for GenericRegistry<KVB>
where
    KVB: RegistryItem,
{
    fn create() -> Self {
        Self {
            name_lookup: RwLock::new(HashMap::new()),
            value_lookup: HashMap::new(),
            inbox: SegQueue::new(),
        }
    }

    fn get_registry_id(&self, lookup_name: &str) -> Option<KVB::K> {
        self.name_lookup
            .read()
            .ok()
            .and_then(|names| names.get(lookup_name).map(|k_ref| k_ref.clone()))
    }

    fn fetch(&self, registry_id: &KVB::K) -> Option<RwLockReadGuard<'_, KVB::V>> {
        self.value_lookup
            .get(registry_id)
            .map(|entry| entry.read().ok().unwrap())
    }

    fn fetch_mut(&self, registry_id: &KVB::K) -> Option<RwLockWriteGuard<'_, KVB::V>> {
        self.value_lookup
            .get(registry_id)
            .map(|entry| entry.write().ok().unwrap())
    }

    fn enqueue_builder<B: Into<KVB>>(&self, lookup_name: &str, builder: B) -> KVB::K {
        let builder = builder.into();
        if let Some(k) = self.get_registry_id(lookup_name) {
            k.clone()
        } else {
            let k = builder.key();
            self.name_lookup
                .write()
                .unwrap()
                .insert(lookup_name.to_string(), k.clone());
            self.inbox.push(builder);
            k
        }
    }

    fn flush(&mut self) {
        while !self.inbox.is_empty() {
            let builder = self.inbox.pop().unwrap();
            self.value_lookup.insert(builder.key(), RwLock::from(builder.build()));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static mut KEY_NUM: u32 = 0u32;

    struct TestKVB {
        k: u32,
    }

    impl Default for TestKVB {
        fn default() -> Self {
            unsafe {
                KEY_NUM += 1;
            }
            Self { k: unsafe { KEY_NUM } }
        }
    }

    impl RegistryItem for TestKVB {
        type K = u32;
        type V = u32;
        #[allow(unused_mut)]
        fn build(mut self) -> Self::V {
            self.k
        }
        fn key(&self) -> Self::K {
            self.k
        }
        fn is_buildable(&self) -> bool {
            true
        }
    }

    #[test]
    fn test_kvb_increments_each_key() {
        unsafe {
            KEY_NUM = 0;
        }
        let k1 = TestKVB::default();
        let k2 = TestKVB::default();
        let k3 = TestKVB::default();
        assert_eq!(k1.k, 1u32);
        assert_eq!(k2.k, 2u32);
        assert_eq!(k3.k, 3u32);
    }

    #[test]
    fn same_readable_name_gives_same_key() {
        unsafe {
            KEY_NUM = 0;
        }
        let registry = GenericRegistry::<TestKVB>::default();
        let k1 = registry.enqueue_builder("build1", TestKVB::default());
        let k2 = registry.enqueue_builder("build1", TestKVB::default());
        let k3 = registry.enqueue_builder("build2", TestKVB::default());
        let k4 = registry.enqueue_builder("build2", TestKVB::default());
        assert_eq!(k1, k2);
        assert_ne!(k2, k3);
        assert_eq!(k3, k4);
    }

    #[test]
    fn values_are_built_when_flushed() {
        unsafe {
            KEY_NUM = 0;
        }
        let mut registry = GenericRegistry::<TestKVB>::default();
        let k1 = registry.enqueue_builder("build1", TestKVB::default());
        let k3 = registry.enqueue_builder("build2", TestKVB::default());
        assert_eq!(registry.fetch(&k1).is_none(), true);
        assert_eq!(registry.fetch(&k3).is_none(), true);
        registry.flush();
        assert_ne!(registry.fetch(&k1).is_none(), true);
        assert_ne!(registry.fetch(&k3).is_none(), true);
    }
}
