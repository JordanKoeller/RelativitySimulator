use cgmath;
// use cgmath::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
// use std::time::SystemTime;

// #[allow(dead_code)]
// pub fn elapsed(start_time: &SystemTime) -> String {
//   let elapsed = start_time.elapsed().unwrap();
//   format!(
//     "{}s {:.*}ms",
//     elapsed.as_secs(),
//     1,
//     elapsed.subsec_nanos() as f64 / 1_000_000.0
//   )
// }

pub type Vec2F = cgmath::Vector2<f32>;
#[allow(dead_code)]
pub type Vec2I = cgmath::Vector2<i32>;
pub type Vec3F = cgmath::Vector3<f32>;
pub type Vec4F = cgmath::Vector4<f32>;
pub type Mat4F = cgmath::Matrix4<f32>;
pub type Mat3F = cgmath::Matrix3<f32>;
#[allow(dead_code)]
pub type Mat2F = cgmath::Matrix2<f32>;
#[allow(dead_code)]
pub type Color = Vec3F;

#[allow(dead_code)]
pub type Ref<T> = Rc<T>;
pub type MutRef<T> = Rc<RefCell<T>>;
#[allow(dead_code)]
pub type Mut<T> = RefCell<T>;

#[derive(Default)]
pub struct Timestep(pub f32);

#[allow(dead_code, non_snake_case)]
pub fn GetMutRef<T>(v: T) -> MutRef<T> {
  Rc::new(RefCell::new(v))
}

pub fn translate(pos: Vec3F) -> Mat4F {
  Mat4F::from_translation(pos)
}

#[allow(dead_code)]
pub fn scale(factor: f32) -> Mat4F {
  Mat4F::from_scale(factor)
}

#[allow(dead_code)]
pub fn nonunif_scale(factor: Vec3F) -> Mat4F {
  Mat4F::from_nonuniform_scale(factor.x, factor.y, factor.z)
}

// MultiMap

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
