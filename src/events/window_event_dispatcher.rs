extern crate glfw;

use gl;

use utils::Vec2F;

use super::{Event, StatelessEventChannel, EventChannel, EventPayload, KeyCode, MouseButton};

use renderer::Window;
use debug::*;

#[derive(Clone, Debug)]
pub struct WindowEvent {
  pub code: Event,
  pub payload: Option<EventPayload>,
}

impl std::hash::Hash for WindowEvent {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.code.hash(state)
  }
}

impl PartialEq for WindowEvent {
  fn eq(&self, other: &Self) -> bool {
    self.code == other.code
  }
}

impl Eq for WindowEvent {}

impl WindowEvent {
  pub fn new(evt: Event) -> Self {
    WindowEvent {
      code: evt,
      payload: None,
    }
  }

  pub fn payload(code: Event, payload: EventPayload) -> Self {
    WindowEvent {
      code,
      payload: Some(payload),
    }
  }
}

pub struct WindowEventDispatcher {
  last_mouse_pos: Option<Vec2F>,
  down_keys: Vec<bool>,
  down_mouse: Vec<bool>,
  count: u32,
}

impl WindowEventDispatcher {
  pub fn process_events(&mut self, channel: &mut StatelessEventChannel<WindowEvent>, window: &mut Window) {
    channel.clear_events();
    for (_, event) in glfw::flush_messages(&window.events) {
      // Process Application Events
      self.count += 1;
      window.imgui_glfw.handle_event(&mut window.im_context, &event);
      match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
          // make sure the viewport matches the new window dimensions; note that width and
          // height will be significantly larger than specified on retina displays.
          unsafe { gl::Viewport(0, 0, width, height) }
        }
        glfw::WindowEvent::CursorPos(xpos, ypos) => {
          if !window.cursor {
            let new_pos = Vec2F::new(xpos as f32, ypos as f32);
            if let Some(last_pos) = self.last_mouse_pos {
              let offset = Vec2F::new(new_pos.x - last_pos.x, last_pos.y - new_pos.y);
              let evt = WindowEvent::payload(Event::MouseMoved, EventPayload::MouseMove(offset));
              channel.publish(evt);
              // self.receive_event(Event::MouseMoved, Some(EventPayload::MouseMove(offset)));
            }
            self.last_mouse_pos = Some(new_pos);
          } else {
            let new_pos = Vec2F::new(xpos as f32, ypos as f32);
            self.last_mouse_pos = Some(new_pos);
          }
        }
        glfw::WindowEvent::Key(key_code, _, key_action, _) => {
          let my_key: KeyCode = KeyCode::from(key_code);
          if !window.cursor || my_key == KeyCode::Control {
            match key_action {
              glfw::Action::Press => {
                self.down_keys[my_key.clone() as usize] = true;
                channel.publish(WindowEvent::new(Event::KeyPressed(my_key)));
              }
              glfw::Action::Release => {
                self.down_keys[my_key.clone() as usize] = false;
                channel.publish(WindowEvent::new(Event::KeyReleased(my_key)));
              }
  
              _ => {} // glfw::Action::Repeat => self.receive_event(Event::KeyDown(my_key), None),
            }
          }
        }
        glfw::WindowEvent::MouseButton(button, action, _) => {
          if !window.cursor {
            let my_button = MouseButton::from(button);
            match action {
              glfw::Action::Press => {
                self.down_mouse[my_button.clone() as usize] = true;
                channel.publish(WindowEvent::new(Event::MousePressed(my_button)));
              }
              // glfw::Action::Repeat => self.receive_event(Event::MousePressed(my_button), None),
              glfw::Action::Release => channel.publish(WindowEvent::new(Event::MouseReleased(my_button))),
              _ => {}
            }
          }
        }
        _ => {}
      }
      // Process IMGUI
    }
    for ind in 0..KeyCode::KeyCodeLength as usize {
      if self.down_keys[ind] {
        channel.publish(WindowEvent::new(Event::KeyDown(KeyCode::from(ind))))
      }
    }
    for ind in 0..MouseButton::MouseButtonLength as usize {
      if self.down_mouse[ind] {
        channel.publish(WindowEvent::new(Event::MouseDown(MouseButton::from(ind))))
      }
    }
  }
}

impl Default for WindowEventDispatcher {
  fn default() -> Self {
    WindowEventDispatcher {
      count: 0,
      last_mouse_pos: None,
      down_keys: (0..KeyCode::KeyCodeLength as usize).map(|_| false).collect(),
      down_mouse: (0..MouseButton::MouseButtonLength as usize).map(|_| false).collect(),
    }
  }
}

