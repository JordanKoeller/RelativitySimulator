use std::cmp::{Ord, Ordering};
use specs::Entity;

use ecs::{DrawableId};
use utils::Mat4F;
use ecs::Material;


#[derive(Eq, PartialEq, Debug, Clone)]
pub enum RenderCommand {
  Draw,
  Free
}

impl RenderCommand {
  pub fn priority(&self) -> u32 {
    match self { // smaller numbers will happen FIRST
      RenderCommand::Free => 1,
      RenderCommand::Draw => 0,
    }
  }
}

impl PartialOrd for RenderCommand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.priority().cmp(&other.priority()))
  }
}

impl Ord for RenderCommand {
  fn cmp(&self, other: &Self) -> Ordering {
    self.priority().cmp(&other.priority())
  }
}