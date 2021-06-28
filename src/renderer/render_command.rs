use renderer::DrawableId;
use physics::components::TransformComponent;
use utils::Mat4F;

#[derive(Clone, Debug)]
pub struct DrawCommand {
  pub id: DrawableId,
  pub transform: Mat4F
}