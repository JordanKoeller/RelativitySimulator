use renderer::{DrawableId, Material};
use utils::Mat4F;

#[derive(Clone, Debug)]
pub struct DrawCommand {
  pub id: DrawableId,
  pub transform: Mat4F
}

#[derive(Clone, Debug)]
pub enum RenderCommand {
  Draw {id: DrawableId },
  SetTransform {id: DrawableId, transform: Mat4F},
  
}