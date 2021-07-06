
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
  
  pub fn get_slot(&mut self, texture_id: u32) -> u32 {
    // First check if it's in an already bound slot.
      for i in (self.reserved + 1..self.top).rev() {
        if let BoundState::Bound(tid) = self.slots[i] {
          if tid == texture_id {
            return i as u32;
          }
        }
      }
      // Could not find. Need to put it into top and increment top.
      if self.top < self.slots.len() {
        self.slots[self.top] = BoundState::Bound(texture_id);
        self.top += 1;
        (self.top - 1) as u32
      } else {
        panic!("All texture slots are full!");
      }
  }
}

