use renderer::DrawableId;
use ecs::components::Transform;

#[derive(Clone)]
pub struct DrawCommand {
  pub id: DrawableId,
  pub transform: Transform
}