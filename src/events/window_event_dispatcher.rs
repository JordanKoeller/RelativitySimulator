extern crate glfw;
use std::sync::mpsc::Receiver;

use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

use gl;

use events::*;
use renderer::Window;
use utils::*;
// impl Default for EventListener {
//   fn default() -> Self {
//     EventListener {
//       mouse_movement: Vec2F::new(0.0, 0.0),
//       last_mouse_pos: None,
//       time_pressed: [None; 512],
//       subscribed_events: HashMap::new(),
//     }
//   }
// }
// impl EventListener {
//   pub fn publish_mouse_moved(&mut self, delta: Vec2F) {
//     self.mouse_movement = delta;
//   }

//   fn publish_key_event(&mut self, key: Key, action: Action) {
//     let time = SystemTime::now();
//     match action {
//       Action::Press => {
//         self.time_pressed[key as usize] = Some(time);
//       }
//       Action::Release => {
//         self.time_pressed[key as usize] = None;
//       }
//       _ => {}
//     };
//   }

//   fn hash_evt(&self, evt: &KeyDown) -> u32 {
//     if evt.duration.is_some() {
//       evt.duration.unwrap().to_bits() ^ evt.key as u32
//     } else {
//       evt.key as u32
//     }
//   }

//   pub fn subscribe_event(&mut self, evt: KeyDown) -> u32 {
//     let id = self.hash_evt(&evt);
//     self.subscribed_events.insert(id, evt);
//     id
//   }

//   pub fn remove_event(&mut self, evt: u32) {
//     self.subscribed_events.remove(&evt);
//   }

//   pub fn consume_events(&self) -> Vec<u32> {
//     // Loops over each subscribed event
//     // Returns the ids of the events that are currently triggered in a vetor
//     // TODO: Optimize to use iterators instead of vectors
//     self
//       .subscribed_events
//       .iter()
//       .filter_map(|x| {
//         if let Some(duration) = x.1.duration {
//           if let Some(time_pressed) = self.time_pressed[x.1.key as usize] {
//             if (time_pressed.elapsed().unwrap().as_millis() as f32) < duration {
//               let ret = *x.0;
//               Some(ret)
//             } else {
//               None
//             }
//           } else {
//             None
//           }
//         } else {
//           if let Some(_) = self.time_pressed[x.1.key as usize] {
//             let ret = *x.0;
//             Some(ret)
//           } else {
//             None
//           }
//         }
//       })
//       .collect()
//   }

//   pub fn consume_mouse_moved(&mut self) -> Vec2F {
//     let ret = self.mouse_movement;
//     self.mouse_movement = Vec2F::new(0.0, 0.0);
//     ret
//   }

// pub struct KeyDown {
//   key: Key,
//   duration: Option<f32>,
// }

// impl PartialEq for KeyDown {
//   fn eq(&self, other: &KeyDown) -> bool {
//     self.key == other.key
//       && ((self.duration.is_none() && other.duration.is_none())
//         || (self.duration.is_some() && other.duration.is_some() && self.duration.unwrap() == other.duration.unwrap()))
//   }
// }

// impl Eq for KeyDown {}

// impl KeyDown {
//   pub fn key_down(k: Key) -> KeyDown {
//     KeyDown { key: k, duration: None }
//   }
// }

// pub struct EventListener {
//   mouse_movement: Vec2F,
//   last_mouse_pos: Option<Vec2F>,
//   time_pressed: [Option<SystemTime>; 512],
//   subscribed_events: HashMap<u32, KeyDown>,
// }

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
    self.receive_event(Event::MouseMoved, Some(EventPayload::MouseMove(Vec2F::new(0f32, 0f32))));
    for (_, event) in glfw::flush_messages(&window.events) {
      // Process Application Events
      match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
          // make sure the viewport matches the new window dimensions; note that width and
          // height will be significantly larger than specified on retina displays.
          unsafe { gl::Viewport(0, 0, width, height) }
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
            glfw::Action::Press => self.receive_event(Event::KeyPressed(my_key), None),
            glfw::Action::Release => self.receive_event(Event::KeyReleased(my_key), None),
            glfw::Action::Repeat => self.receive_event(Event::KeyPressed(my_key), None),
          }
        }
        glfw::WindowEvent::MouseButton(button, action, _) => {
          let my_button = MouseButton::from(button);
          match action {
            glfw::Action::Press => self.receive_event(Event::MousePressed(my_button), None),
            glfw::Action::Repeat => self.receive_event(Event::MousePressed(my_button), None),
            glfw::Action::Release => self.receive_event(Event::MouseReleased(my_button), None),
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
