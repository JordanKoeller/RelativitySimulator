use utils::*;

use glfw::Key as GLKey;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
#[allow(dead_code)]
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

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum EventPayload {
  MouseScroll(i32),
  MouseMove(Vec2F),
  Duration(f32),
  WindowSize(Vec2F)
}

#[repr(usize)]
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum MouseButton {
  LeftButton,
  RightButton,
  MiddleButton,
  MouseButtonLength,
}

impl From<glfw::MouseButton> for  MouseButton {
  fn from(button: glfw::MouseButton) -> MouseButton {
    match button {
      glfw::MouseButton::Button1 => MouseButton::LeftButton,
      glfw::MouseButton::Button2 => MouseButton::RightButton,
      glfw::MouseButton::Button3 => MouseButton::MiddleButton,
      _ => panic!("Could not match mouse button {:?}", button),
    }
  }
}

impl From<usize> for MouseButton {
  fn from(u: usize) -> MouseButton {
    match u {
      0 => MouseButton::LeftButton,
      1 => MouseButton::RightButton,
      2 => MouseButton::MiddleButton,
      3 => MouseButton::MouseButtonLength,
      _ => panic!("Could not match mouse button {}", u),
    }
  }
}

#[repr(usize)]
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
  Backspace,
  KeyCodeLength
}

impl From<GLKey> for KeyCode {
  fn from(code: GLKey) -> Self {
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
      GLKey::Num0 => KeyCode::Zero,
      GLKey::Num1 => KeyCode::One,
      GLKey::Num2 => KeyCode::Two,
      GLKey::Num3 => KeyCode::Three,
      GLKey::Num4 => KeyCode::Four,
      GLKey::Num5 => KeyCode::Five,
      GLKey::Num6 => KeyCode::Six,
      GLKey::Num7 => KeyCode::Seven,
      GLKey::Num8 => KeyCode::Eight,
      GLKey::Num9 => KeyCode::Nine,
      GLKey::Right => KeyCode::ArrowRight,
      GLKey::Left => KeyCode::ArrowLeft,
      GLKey::Up => KeyCode::ArrowUP,
      GLKey::Down => KeyCode::ArrowDown,
      GLKey::LeftShift => KeyCode::LeftShift,
      GLKey::LeftAlt => KeyCode::Alt,
      GLKey::RightAlt => KeyCode::Alt,
      GLKey::Space => KeyCode::Space,
      GLKey::Tab => KeyCode::Tab,
      GLKey::LeftControl => KeyCode::Control,
      GLKey::Escape => KeyCode::Esc,
      GLKey::Backspace => KeyCode::Backspace,
      _ => panic!("Could not parse code {:?}", code),
    }
  }
}

impl From<usize> for KeyCode {
  fn from(u: usize) -> Self {
    use super::KeyCode::*;
    match u {
      u if u == A as usize => A,
      u if u == B as usize => B,
      u if u == C as usize => C,
      u if u == D as usize => D,
      u if u == E as usize => E,
      u if u == F as usize => F,
      u if u == G as usize => G,
      u if u == H as usize => H,
      u if u == I as usize => I,
      u if u == J as usize => J,
      u if u == K as usize => K,
      u if u == L as usize => L,
      u if u == M as usize => M,
      u if u == N as usize => N,
      u if u == O as usize => O,
      u if u == P as usize => P,
      u if u == Q as usize => Q,
      u if u == R as usize => R,
      u if u == S as usize => S,
      u if u == T as usize => T,
      u if u == U as usize => U,
      u if u == V as usize => V,
      u if u == W as usize => W,
      u if u == X as usize => X,
      u if u == Y as usize => Y,
      u if u == Z as usize => Z,
      u if u == Space as usize => Space,
      u if u == One as usize => One,
      u if u == Two as usize => Two,
      u if u == Three as usize => Three,
      u if u == Four as usize => Four,
      u if u == Five as usize => Five,
      u if u == Six as usize => Six,
      u if u == Seven as usize => Seven,
      u if u == Eight as usize => Eight,
      u if u == Nine as usize => Nine,
      u if u == Zero as usize => Zero,
      u if u == LeftShift as usize => LeftShift,
      u if u == RightShift as usize => RightShift,
      u if u == Control as usize => Control,
      u if u == Tab as usize => Tab,
      u if u == Alt as usize => Alt,
      u if u == Esc as usize => Esc,
      u if u == Enter as usize => Enter,
      u if u == ArrowDown as usize => ArrowDown,
      u if u == ArrowUP as usize => ArrowUP,
      u if u == ArrowLeft as usize => ArrowLeft,
      u if u == ArrowRight as usize => ArrowRight,
      u if u == LeftBracket as usize => LeftBracket,
      u if u == RightBracket as usize => RightBracket,
      u if u == Pipe as usize => Pipe,
      u if u == Comma as usize => Comma,
      u if u == Period as usize => Period,
      u if u == Slash as usize => Slash,
      u if u == Tilde as usize => Tilde,
      u if u == Esc as usize => Esc,
      u if u == Backspace as usize => Backspace,
      u if u == KeyCodeLength as usize => KeyCodeLength,
      _ => panic!("Could not convert usize {} to a KeyCode", u)
    }
  }
}
