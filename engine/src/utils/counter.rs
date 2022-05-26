use std::sync::RwLock;

use crate::utils::Mut;

pub struct Counter {
    value: RwLock<u32>,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            value: RwLock::from(0u32),
        }
    }
}

impl Counter {
    pub fn increment(&self) {
        *self.value.write().unwrap() += 1u32;
    }

    pub fn reset(&self) {
        *self.value.write().unwrap() = 0u32;
    }

    pub fn get(&self) -> u32 {
        self.value.read().unwrap().clone()
    }
}
