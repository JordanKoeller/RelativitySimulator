use crate::utils::{Mut};

pub struct Counter {
    value: Mut<u32>,
}

impl Counter {
    pub fn increment(&self) {
        *self.value.borrow_mut() += 1u32;
    }

    pub fn reset(&self) {
        self.value.replace(0u32);
    }

    pub fn get(&self) -> u32 {
        self.value.borrow().clone()
    }
}