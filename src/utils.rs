use cgmath;
use cgmath::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::time::SystemTime;

#[allow(dead_code)]
pub fn elapsed(start_time: &SystemTime) -> String {
  let elapsed = start_time.elapsed().unwrap();
  format!(
    "{}s {:.*}ms",
    elapsed.as_secs(),
    1,
    elapsed.subsec_nanos() as f64 / 1_000_000.0
  )
}

pub type Vec2F = cgmath::Vector2<f32>;
pub type Vec2I = cgmath::Vector2<i32>;
pub type Vec3F = cgmath::Vector3<f32>;
pub type Vec4F = cgmath::Vector4<f32>;
pub type Mat4F = cgmath::Matrix4<f32>;
pub type Mat3F = cgmath::Matrix3<f32>;
pub type Mat2F = cgmath::Matrix2<f32>;
pub type Color = Vec3F;


pub type Ref<T> = Rc<T>;
pub type MutRef<T> = Rc<RefCell<T>>;

pub type Timestep = f32;

// pub fn GetMutRef<T>(ref: Ref<T>) -> MutRef<T> {
//   Rc::new(RefCell::new(mutRef))
// }

pub fn translate(pos: Vec3F) -> Mat4F {
  Mat4F::from_translation(pos)
}

pub fn scale(factor: f32) -> Mat4F {
  Mat4F::from_scale(factor)
}

// MultiMap

const DEFAULT_CAPACITY: usize = 10;

pub struct MultiMap<K, V>
where
  K: Eq + Hash,
{
  data: HashMap<K, Vec<V>>,
}

impl<K, V> MultiMap<K, V>
where
  K: Eq + Hash,
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

  pub fn remove(&mut self, k: &K) -> Option<Vec<V>> {
    self.data.remove(k)
  }

  pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
    self.data.iter().flat_map(|(k, vals)| vals.iter().map(move |v| (k, v)))
  }

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
