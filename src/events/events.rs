use utils::*;

use glfw::Key as GLKey;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Event {
  KeyPressed(KeyCode),
  KeyDown(KeyCode),
  KeyReleased(KeyCode),
  MousePressed(MouseButton),
  MouseReleased(MouseButton),
  MouseDown(MouseButton),
  MouseScrolled,
  MouseMoved,
  MouseDragged,
  WindowResized,
}

impl Event {
  pub fn event_inverse(self) -> Option<Event> {
    match self {
      Event::KeyDown(k) => Some(Event::KeyReleased(k)),
      Event::KeyReleased(k) => Some(Event::KeyDown(k)),
      Event::MousePressed(k) => Some(Event::MouseReleased(k)),
      Event::MouseReleased(k) => Some(Event::MousePressed(k)),
      _ => None,
    }
  }
}

#[derive(Clone, Debug)]
pub enum EventPayload {
  MouseScroll(i32),
  MouseMove(Vec2F),
  Duration(f32),
  WindowSize(Vec2F)
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum MouseButton {
  LeftButton,
  RightButton,
  MiddleButton,
}

impl MouseButton {
  pub fn from(button: glfw::MouseButton) -> MouseButton {
    match button {
      glfw::MouseButton::Button1 => MouseButton::LeftButton,
      glfw::MouseButton::Button2 => MouseButton::RightButton,
      glfw::MouseButton::Button3 => MouseButton::MiddleButton,
      _ => panic!(format!("Could not match mouse button {:?}", button)),
    }
  }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum KeyCode {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  Space,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Zero,
  LeftShift,
  RightShift,
  Control,
  Tab,
  Alt,
  Esc,
  Enter,
  ArrowDown,
  ArrowUP,
  ArrowLeft,
  ArrowRight,
  LeftBracket,
  RightBracket,
  Pipe,
  Comma,
  Period,
  Slash,
  Tilde,
}

impl KeyCode {
  pub fn from(code: glfw::Key) -> KeyCode {
    match code {
      GLKey::A => KeyCode::A,
      GLKey::B => KeyCode::B,
      GLKey::C => KeyCode::C,
      GLKey::D => KeyCode::D,
      GLKey::E => KeyCode::E,
      GLKey::F => KeyCode::F,
      GLKey::G => KeyCode::G,
      GLKey::H => KeyCode::H,
      GLKey::I => KeyCode::I,
      GLKey::J => KeyCode::J,
      GLKey::K => KeyCode::K,
      GLKey::L => KeyCode::L,
      GLKey::M => KeyCode::M,
      GLKey::N => KeyCode::N,
      GLKey::O => KeyCode::O,
      GLKey::P => KeyCode::P,
      GLKey::Q => KeyCode::Q,
      GLKey::R => KeyCode::R,
      GLKey::S => KeyCode::S,
      GLKey::T => KeyCode::T,
      GLKey::U => KeyCode::U,
      GLKey::V => KeyCode::V,
      GLKey::W => KeyCode::W,
      GLKey::X => KeyCode::X,
      GLKey::Y => KeyCode::Y,
      GLKey::Z => KeyCode::Z,
      GLKey::LeftShift => KeyCode::LeftShift,
      GLKey::Space => KeyCode::Space,
      GLKey::Tab => KeyCode::Tab,

      _ => panic!(format!("Could not parse code {:?}", code)),
    }
  }
}
