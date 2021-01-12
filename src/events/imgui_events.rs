use events::{EventChannel, ReceiverID};
use utils::*;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum ImguiUiEvent {
  FloatInput(ReceiverID, f32, String),
  IntInput(ReceiverID, i32, String),
  BoolInput(ReceiverID, bool, String),
  Vec2Input(ReceiverID, Vec2F, String),
  Vec3Input(ReceiverID, Vec3F, String),
  Vec2IntInput(ReceiverID, Vec2F, String),
}

impl ImguiUiEvent {
  pub fn index(&self) -> usize {
    let space = usize::MAX / 6;
    match self {
      ImguiUiEvent::FloatInput(id, _, _) => 0 + id,
      ImguiUiEvent::IntInput(id, _, _) => space + id,
      ImguiUiEvent::BoolInput(id, _, _) => 2 * space + id,
      ImguiUiEvent::Vec2Input(id, _, _) => 3 * space + id,
      ImguiUiEvent::Vec3Input(id, _, _) => 4 * space + id,
      ImguiUiEvent::Vec2IntInput(id, _, _) => 5 * space + id,
    }
  }
}

impl std::hash::Hash for ImguiUiEvent {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
     self.index().hash(state)
  }
}

impl PartialEq for ImguiUiEvent {
  fn eq(&self, other: &ImguiUiEvent) -> bool {
    self.index() == other.index()
  }
}

impl Eq for ImguiUiEvent {}