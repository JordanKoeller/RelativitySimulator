use crate::events::ReceiverID;
use crate::utils::*;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum ImguiUiEvent {
    FloatInput(usize, f64),
    IntInput(usize, i32),
    BoolInput(usize, bool),
    Vec2Input(usize, Vec2F),
    Vec3Input(usize, Vec3F),
    Vec2IntInput(usize, Vec2F),
}

impl ImguiUiEvent {
    pub fn index(&self) -> usize {
        match self {
            ImguiUiEvent::FloatInput(e, _) => *e,
            ImguiUiEvent::IntInput(e, _) => *e,
            ImguiUiEvent::BoolInput(e, _) => *e,
            ImguiUiEvent::Vec2Input(e, _) => *e,
            ImguiUiEvent::Vec3Input(e, _) => *e,
            ImguiUiEvent::Vec2IntInput(e, _) => *e,
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
