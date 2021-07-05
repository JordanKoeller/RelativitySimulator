

pub struct UnsafeMut<T> {
  value: T,
  borrowed: bool,
  borrowed_mut: bool,
}

impl<T> UnsafeMut<T> {
  pub fn new(v: T) -> Self {
    Self {
      value: v,
      borrowed: false,
      borrowed_mut: false
    }
  }

  pub fn borrow(&self) -> &T {
    if !self.borrowed_mut {
      &self.value
    } else {
      panic!("Value is already borrowed mutably. Cannot borrow again.")
    }
  }

  // pub fn brrow_mut(&self) -> &mut T {
  //   if !self.borrowed_mut && !self.borrowed {
  //     let mut_ptr = &self.value as *mut T;
  //   }
  // }
}