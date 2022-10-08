use crate::debug::macros::*;
use std::ffi::{CStr, CString};

use crate::graphics::{Shader, ShaderId, Texture, TextureId};

struct LRUTextureNode {
  pub value: u32,
  pub next_node: usize,
  pub prev_node: usize,
  pub generation: usize,
}

impl LRUTextureNode {
  pub fn new(value: u32, generation: usize) -> Self {
    Self {
      value,
      next_node: std::usize::MAX,
      prev_node: std::usize::MAX,
      generation,
    }
  }
}

pub struct TextureBinder {
  slots: Vec<Option<LRUTextureNode>>,
  oldest_node: usize,
  newest_node: usize,
  generation: usize,
  top: usize,
}

impl TextureBinder {
  pub fn new(capacity: usize) -> Self {
    let mut slots = Vec::new();
    for _ in 0..capacity + 1 {
      slots.push(None);
    }
    Self {
      slots,
      oldest_node: std::usize::MAX,
      newest_node: std::usize::MAX,
      generation: 0usize,
      top: 1usize,
    }
  }

  pub fn get_slot(&mut self, texture_id: u32) -> (u32, bool) {
    for i in 1..self.top {
      // Found this texture already bound somewhere. Return the index as the slot identifier.
      if let Some(node) = self.slots[i].as_mut() {
        if node.value == texture_id {
          node.generation = self.generation;
          if i == self.oldest_node {
            self.oldest_node = node.next_node;
          }
          self.move_to_back(i);
          return (i as u32, false);
        }
      }
    }
    // Not found and there was an empty slot, so fill it
    if self.top < self.slots.len() {
      self.slots[self.top] = Some(LRUTextureNode::new(texture_id, self.generation));
      if self.top == 1 {
        self.newest_node = 1;
        self.oldest_node = 1;
      } else {
        self.move_to_back(self.top);
      }
      self.top += 1;
      return (self.newest_node as u32, true);
    }
    // No empty slots. So look to oldest node and see if I can invalidate it.
    if let Some(oldest_node) = self.slots[self.oldest_node].as_mut() {
      if oldest_node.generation < self.generation {
        oldest_node.generation = self.generation;
        oldest_node.value = texture_id;
        let oldest_slot = self.oldest_node;
        if oldest_node.next_node != std::usize::MAX {
          self.oldest_node = oldest_node.next_node;
          self.move_to_back(oldest_slot);
        }
        return (oldest_slot as u32, true);
      }
    }
    panic!("No available slot!");
  }

  pub fn free_slot(&mut self, texture_id: u32) {
    for i in (1..self.top).rev() {
      if self.slots[i].is_some() {
        if self.slots[i].as_ref().unwrap().value == texture_id {
          self.slots[i] = None;
          return;
        }
      }
    }
  }

  pub fn bind(&mut self, shader: &Shader, uniform_name: &str, texture: &TextureId) -> u32 {
    let (slot, new) = self.get_slot(texture.id());
    if new {
      // texture.bind(slot);
      let formatted = format!("{}[{}]", uniform_name, slot);
      shader.set_texture(slot, &formatted, texture);
    }
    slot
  }

  pub fn refresh(&mut self) {
    for i in 1..self.slots.len() {
      self.slots[i] = None;
    }
    self.top = 1;
    self.newest_node = std::usize::MAX;
    self.oldest_node = std::usize::MAX;
    self.generation = 0;
  }

  pub fn increment_generation(&mut self) {
    self.generation += 1;
  }

  pub fn bound_slots(&self) -> impl Iterator<Item = &u32> {
    self
      .slots
      .iter()
      .filter_map(|node_opt| node_opt.as_ref().map(|node| &node.value))
  }

  fn move_to_back(&mut self, node_to_move: usize) {
    let (moving_prev, moving_id, moving_next) = {
      let moving_node = self.slots[node_to_move].as_ref().unwrap();
      (moving_node.prev_node, node_to_move, moving_node.next_node)
    };
    let last_id = self.newest_node;
    // First fix moving_prev to point to moving_next
    if moving_prev != std::usize::MAX {
      self.slots[moving_prev].as_mut().unwrap().next_node = moving_next;
    }
    if moving_next != std::usize::MAX {
      self.slots[moving_next].as_mut().unwrap().prev_node = moving_prev;
    }
    let moving_node = self.slots[moving_id].as_mut().unwrap();
    moving_node.prev_node = last_id;
    moving_node.next_node = std::usize::MAX;
    let last_node = self.slots[last_id].as_mut().unwrap();
    last_node.next_node = moving_id;
    self.newest_node = moving_id;
  }
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_can_bind_slot() {
    let mut t = TextureBinder::new(32);
    let ids: [u32; 7] = [8u32, 6u32, 7u32, 5u32, 3u32, 0u32, 9u32];
    let expected_slots: [u32; 7] = [1, 2, 3, 4, 5, 6, 7];
    ids.iter().zip(expected_slots.iter()).for_each(|(&id, &slt)| {
      let (res, new) = t.get_slot(id);
      assert_eq!(res, slt);
      assert_eq!(new, true);
    });
  }

  #[test]
  fn test_dupliates_are_not_pushed_again() {
    let mut t = TextureBinder::new(32);
    let ids: [u32; 7] = [8, 6, 7, 5, 3, 0, 9];
    let expected_slots: [u32; 7] = [1, 2, 3, 4, 5, 6, 7];
    ids.iter().zip(expected_slots.iter()).for_each(|(&id, &slt)| {
      let (res, new) = t.get_slot(id);
      assert_eq!(res, slt);
      assert_eq!(new, true);
    });
    let ids: [u32; 7] = [8, 3, 7, 5, 3, 10, 8];
    let expected_slots: [u32; 7] = [1, 5, 3, 4, 5, 8, 1];
    ids.iter().zip(expected_slots.iter()).for_each(|(&id, &slt)| {
      let (res, new) = t.get_slot(id);
      assert_eq!(res, slt, "Expected {} but found {}", res, slt);
      if slt < 8 {
        assert_eq!(new, false);
      } else {
        assert_eq!(new, true);
      }
    });
  }

  #[test]
  fn test_invalidates_textures_from_old_generation() {
    let mut t = TextureBinder::new(5);
    assert_eq!(t.get_slot(1), (1, true));
    assert_eq!(t.get_slot(2), (2, true));
    assert_eq!(t.get_slot(3), (3, true));
    assert_eq!(t.get_slot(4), (4, true));
    assert_eq!(t.get_slot(5), (5, true));
    t.increment_generation();
    assert_eq!(t.get_slot(6), (1, true));
    assert_eq!(t.get_slot(2), (2, false));
    assert_eq!(t.get_slot(3), (3, false));
    assert_eq!(t.get_slot(2), (2, false));
    assert_eq!(t.get_slot(8), (4, true));
    assert_eq!(t.get_slot(9), (5, true));
  }
}
