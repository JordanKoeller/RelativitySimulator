use std::collections::HashMap;
use std::hash::Hash;



const DEFAULT_CAPACITY: usize = 10;

#[derive(Clone)]
pub struct MultiMap<K, V>
where
  K: Eq + Hash + Clone,
  V: Clone,
{
  data: HashMap<K, Vec<V>>,
}

impl<K, V> MultiMap<K, V>
where
  K: Eq + Hash + Clone,
  V: Clone,
{
  pub fn new() -> MultiMap<K, V> {
    MultiMap {
      data: HashMap::default(),
    }
  }
  pub fn push(&mut self, k: K, v: V) {
    let get_attempt = self.data.get_mut(&k);
    match get_attempt {
      Some(vec) => vec.push(v),
      None => {
        let mut vec = Vec::with_capacity(DEFAULT_CAPACITY);
        vec.push(v);
        self.data.insert(k, vec);
      }
    }
  }

  #[allow(dead_code)]
  pub fn get(&self, k: &K) -> Option<&Vec<V>> {
    self.data.get(k)
  }

  #[allow(dead_code)]
  pub fn remove(&mut self, k: &K) -> Option<Vec<V>> {
    self.data.remove(k)
  }

  #[allow(dead_code)]
  pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
    self.data.iter().flat_map(|(k, vals)| vals.iter().map(move |v| (k, v)))
  }

  #[allow(dead_code)]
  pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
    self
      .data
      .iter_mut()
      .flat_map(|(k, vals)| vals.iter_mut().map(move |v| (k, v)))
  }

  pub fn clear(&mut self) {
    self.data.clear()
  }
}
