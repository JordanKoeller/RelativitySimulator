use renderer::DrawableId;
use ecs::components::Transform;

#[derive(Clone, Debug)]
pub struct DrawCommand {
  pub id: DrawableId,
  pub transform: Transform
}