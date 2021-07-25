use debug::macros::*;
use std::ffi::{CStr, CString};

use renderer::{Texture, TextureLike, Shader};

#[derive(Debug)]
enum BoundState {
  Unbound,
  Bound(u32),
}

pub struct TextureBinder {
  slots: Vec<BoundState>,
  top: usize, // First UNBOUND index.
  reserved: usize,
}

impl TextureBinder {
  pub fn new(reserved: usize) -> Self {
    let sz = 32;
    Self {
      slots: (0 .. sz).into_iter().map(|_| BoundState::Unbound).collect(),
      top: reserved + 1,
      reserved,
    }
  }

  pub fn refresh(&mut self) {
    for i in self.reserved..self.slots.len() {
    self.slots[i] = BoundState::Unbound;
    }
  }
  
  pub fn get_slot(&mut self, texture_id: u32) -> (u32, bool) {
    // First check if it's in an already bound slot.
      for i in (self.reserved + 1..self.top).rev() {
        if let BoundState::Bound(tid) = self.slots[i] {
          if tid == texture_id {
            return (i as u32, false);
          }
        }
      }
      // Could not find. Need to put it into top and increment top.
      if self.top < self.slots.len() {
        self.slots[self.top] = BoundState::Bound(texture_id);
        self.top += 1;
        ((self.top - 1) as u32, true)
      } else {
        panic!("All texture slots are full!");
      }
  }

  pub fn free_slot(&mut self, texture_id: u32) {
    for i in (self.reserved + 1..self.top).rev() {
      if let BoundState::Bound(tid) = self.slots[i] {
        if tid == texture_id {
          self.slots[i] = BoundState::Unbound;
          return;
        }
      }
    }
  }

  pub fn bind<T: TextureLike>(&mut self, shader: &Shader, uniform_name: &str,texture: &T) -> u32 {
    let (slot, new) = self.get_slot(texture.id());
    if new {
      // texture.bind(slot);
      let formatted = format!("{}[{}]", uniform_name, slot);
      shader.set_texture(slot, &CString::new(formatted).unwrap(), texture);
    }
    slot
  }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_init_texture_binder() {
        TextureBinder::new(0);
    }

    #[test]
    fn test_can_reserve_slots() {
          TextureBinder::new(2);
    }

    #[test]
    fn test_can_bind_slot() {
      let mut t = TextureBinder::new(0);
      let ids: [u32; 7] = [8u32, 6u32, 7u32, 5u32, 3u32, 0u32, 9u32];
      let expected_slots: [u32; 7] = [1, 2, 3, 4, 5, 6, 7];
      ids.iter().zip(expected_slots.iter()).for_each(|(&id, &slt)| {
        let (res, new) = t.get_slot(id);
        assert_eq!(res, slt);
        assert_eq!(new, true);
      });
    }

    #[test]
    fn test_reserves_slots() {
      let mut t = TextureBinder::new(3);
      let ids: [u32; 7] = [8u32, 6u32, 7u32, 5u32, 3u32, 0u32, 9u32];
      let expected_slots: [u32; 7] = [4, 5, 6, 7, 8, 9, 10];
      ids.iter().zip(expected_slots.iter()).for_each(|(&id, &slt)| {
        let (res, new) = t.get_slot(id);
        assert_eq!(res, slt);
        assert_eq!(new, true);
      });
    }

    #[test]
    fn test_dupliates_are_not_pushed_again() {
      let mut t = TextureBinder::new(3);
      let ids: [u32; 7] =            [8, 6, 7, 5, 3, 0, 9];
      let expected_slots: [u32; 7] = [4, 5, 6, 7, 8, 9, 10];
      ids.iter().zip(expected_slots.iter()).for_each(|(&id, &slt)| {
        let (res, new) = t.get_slot(id);
        assert_eq!(res, slt);
        assert_eq!(new, true);
      });
      let ids: [u32; 7] =            [8, 3, 7, 5, 3, 10, 8];
      let expected_slots: [u32; 7] = [4, 8, 6, 7, 8, 11, 4];
      ids.iter().zip(expected_slots.iter()).for_each(|(&id, &slt)| {
        let (res, new) = t.get_slot(id);
        assert_eq!(res, slt, "Expected {} but found {}", res, slt);
        if slt < 11 {
          assert_eq!(new, false);
        } else {
          assert_eq!(new, true);
        }
      });
    }
}