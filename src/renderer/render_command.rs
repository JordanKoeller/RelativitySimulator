use specs::Entity;

use ecs::{DrawableId};
use utils::Mat4F;
use ecs::Material;

#[derive(Clone, Debug)]
pub struct DrawCommand {
  pub id: DrawableId,
  pub transform: Mat4F
}

#[derive(Clone, Debug)]
pub enum RenderCommand {
  Draw {id: DrawableId, transform: Mat4F, material: Material },
  // SetTransform {id: DrawableId, transform: Mat4F},
  
}