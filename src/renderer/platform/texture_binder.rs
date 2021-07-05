
enum BoundState {
  Unbound,
  Bound(u32),
}

pub struct TextureBinder {
  slots: Vec<BoundState>,
  top: usize,
  reserved: usize,
}

impl TextureBinder {
  pub fn new(reserved: usize) -> Self {
    let sz = 32;
    Self {
      slots: (reserved+1 .. sz).into_iter().map(|_| BoundState::Unbound).collect(),
      top: reserved,
      reserved,
    }
  }

  pub fn refresh(&mut self) {
    for i in self.reserved..self.slots.len() {
      self.slots[i] = BoundState::Unbound;
    }
  }
  
  pub fn get_slot(&mut self, texture_id: u32) -> Option<u32> {
    if self.top == self.reserved {
      self.slots[self.reserved + 1] = BoundState::Bound(texture_id);
      self.top = self.reserved + 1;
      Some(self.top as u32)
    } else {
      for i in (self.reserved + 1..self.top + 1).rev() {
        if let BoundState::Bound(tid) = self.slots[i] {
          if tid == texture_id {
            return Some(i as u32);
          }
        }
      }
      self.top += 1;
      if self.top >= self.slots.len() {
        None
      } else {
        self.slots[self.top] = BoundState::Bound(texture_id);
        Some(self.top as u32)
      }
    }
  }
}

