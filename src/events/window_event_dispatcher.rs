extern crate glfw;

use std::collections::{HashMap, HashSet};

use gl;

use events::*;
use renderer::Window;
use utils::*;

pub struct WindowEventManager {
  event_inbox: HashMap<Event, Option<EventPayload>>,
  subscribed_events: HashMap<Event, u32>,
  inboxes: HashMap<ReceiverID, HashSet<Event>>,
  last_mouse_pos: Option<Vec2F>,
}

impl EventDispatcher for WindowEventManager {
  fn global_event_inbox_mut(&mut self) -> &mut HashMap<Event, Option<EventPayload>> {
    &mut self.event_inbox
  }
  fn global_subscribed_events_mut(&mut self) -> &mut HashMap<Event, u32> {
    &mut self.subscribed_events
  }
  fn receiver_inboxes_mut(&mut self) -> &mut HashMap<ReceiverID, HashSet<Event>> {
    &mut self.inboxes
  }

  fn global_event_inbox(&self) -> &HashMap<Event, Option<EventPayload>> {
    &self.event_inbox
  }
  fn global_subscribed_events(&self) -> &HashMap<Event, u32> {
    &self.subscribed_events
  }
  fn receiver_inboxes(&self) -> &HashMap<ReceiverID, HashSet<Event>> {
    &self.inboxes
  }
}

impl WindowEventManager {
  pub fn process_events(&mut self, window: &mut Window) {
    self.refresh();
    self.receive_event(Event::MouseMoved, Some(EventPayload::MouseMove(Vec2F::new(0f32, 0f32))));
    for (_, event) in glfw::flush_messages(&window.events) {
      // Process Application Events
      match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
          // make sure the viewport matches the new window dimensions; note that width and
          // height will be significantly larger than specified on retina displays.
          println!("Setting frame buffer {} {}", width, height);
          unsafe { gl::Viewport(0, 0, width, height) }
          self.receive_event(Event::WindowResized, Some(EventPayload::WindowSize(Vec2F::new(width as f32, height as f32))))
        }
        glfw::WindowEvent::CursorPos(xpos, ypos) => {
          let new_pos = Vec2F::new(xpos as f32, ypos as f32);
          if let Some(last_pos) = self.last_mouse_pos {
            let offset = Vec2F::new(new_pos.x - last_pos.x, last_pos.y - new_pos.y);
            self.receive_event(Event::MouseMoved, Some(EventPayload::MouseMove(offset)));
          }
          self.last_mouse_pos = Some(new_pos);
        }
        glfw::WindowEvent::Key(key_code, _, key_action, _) => {
          let my_key = KeyCode::from(key_code);
          match key_action {
            glfw::Action::Press => {
              self.receive_event(Event::KeyDown(my_key.clone()), None);
              self.receive_event(Event::KeyPressed(my_key), None);
            }
            glfw::Action::Release => self.receive_event(Event::KeyReleased(my_key), None),
            _ => {} // glfw::Action::Repeat => self.receive_event(Event::KeyDown(my_key), None),
          }
        }
        glfw::WindowEvent::MouseButton(button, action, _) => {
          let my_button = MouseButton::from(button);
          match action {
            glfw::Action::Press => {
              self.receive_event(Event::MousePressed(my_button.clone()), None);
              self.receive_event(Event::MouseDown(my_button), None);
            }
            // glfw::Action::Repeat => self.receive_event(Event::MousePressed(my_button), None),
            glfw::Action::Release => self.receive_event(Event::MouseReleased(my_button), None),
            _ => {}
          }
        }
        _ => {}
      }

      // Process IMGUI
      window.imgui_glfw.handle_event(&mut window.im_context, &event);
    }
  }
}

impl Default for WindowEventManager {
  fn default() -> WindowEventManager {
    WindowEventManager {
      event_inbox: HashMap::new(),
      subscribed_events: HashMap::new(),
      inboxes: HashMap::new(),
      last_mouse_pos: None,
    }
  }
}
