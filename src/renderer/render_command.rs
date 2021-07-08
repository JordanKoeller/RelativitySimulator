use std::cmp::{Ord, Ordering};
use specs::Entity;

use ecs::{DrawableId};
use utils::Mat4F;
use ecs::Material;


#[derive(Eq, PartialEq, Debug)]
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