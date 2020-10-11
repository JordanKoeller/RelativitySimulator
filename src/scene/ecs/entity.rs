use super::super::Scene;

pub trait Entity {

  fn register(self: Box<Self>, scene: &mut Scene);
}
