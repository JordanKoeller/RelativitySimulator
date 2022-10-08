use specs::prelude::*;
use std::ops::{Deref, DerefMut};

use super::SystemUtilities;

pub struct WorldProxy<'a> {
  world: &'a mut World,
}

impl<'a> WorldProxy<'a> {
  pub fn new(world: &'a mut World) -> Self {
    Self { world }
  }

  pub fn utilities(&self) -> SystemUtilities<'_> {
    self.world.system_data()
  }
}

impl<'a> Deref for WorldProxy<'a> {
  type Target = World;

  fn deref(&self) -> &Self::Target {
    self.world
  }
}

impl<'a> DerefMut for WorldProxy<'a> {
  fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
    self.world
  }
}
