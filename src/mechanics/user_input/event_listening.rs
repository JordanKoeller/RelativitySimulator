extern crate glfw;
use std::sync::mpsc::Receiver;

use std::collections::HashMap;
use std::time::SystemTime;

use self::glfw::{Action, Key};
use gl;

use utils::Vec2F;

pub struct KeyDown {
    key: Key,
    duration: Option<f32>,
}

impl PartialEq for KeyDown {
    fn eq(&self, other: &KeyDown) -> bool {
        self.key == other.key
            && ((self.duration.is_none() && other.duration.is_none())
                || (self.duration.is_some()
                    && other.duration.is_some()
                    && self.duration.unwrap() == other.duration.unwrap()))
    }
}

impl Eq for KeyDown {}

impl KeyDown {
    pub fn key_down(k: Key) -> KeyDown {
        KeyDown { key: k, duration: None }
    }
}

pub struct EventListener {
    mouse_movement: Vec2F,
    last_mouse_pos: Option<Vec2F>,
    time_pressed: [Option<SystemTime>; 512],
    subscribed_events: HashMap<u32, KeyDown>,
}

impl Default for EventListener {
    fn default() -> Self {
        EventListener {
            mouse_movement: Vec2F::new(0.0, 0.0),
            last_mouse_pos: None,
            time_pressed: [None; 512],
            subscribed_events: HashMap::new(),
        }
    }
}
impl EventListener {
    pub fn publish_mouse_moved(&mut self, delta: Vec2F) {
        self.mouse_movement = delta;
    }

    fn publish_key_event(&mut self, key: Key, action: Action) {
        let time = SystemTime::now();
        match action {
            Action::Press => {
                self.time_pressed[key as usize] = Some(time);
            }
            Action::Release => {
                self.time_pressed[key as usize] = None;
            }
            _ => {}
        };
    }

    fn hash_evt(&self, evt: &KeyDown) -> u32 {
        if evt.duration.is_some() {
            evt.duration.unwrap().to_bits() ^ evt.key as u32
        } else {
            evt.key as u32
        }
    }

    pub fn subscribe_event(&mut self, evt: KeyDown) -> u32 {
        let id = self.hash_evt(&evt);
        self.subscribed_events.insert(id, evt);
        id
    }

    pub fn remove_event(&mut self, evt: u32) {
        self.subscribed_events.remove(&evt);
    }

    pub fn consume_events(&self) -> Vec<u32> {
        // Loops over each subscribed event
        // Returns the ids of the events that are currently triggered in a vetor
        // TODO: Optimize to use iterators instead of vectors
        self.subscribed_events
            .iter()
            .filter_map(|x| {
                if let Some(duration) = x.1.duration {
                    if let Some(time_pressed) = self.time_pressed[x.1.key as usize] {
                        if (time_pressed.elapsed().unwrap().as_millis() as f32) < duration {
                            let ret = *x.0;
                            Some(ret)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    if let Some(_) = self.time_pressed[x.1.key as usize] {
                        let ret = *x.0;
                        Some(ret)
                    } else {
                        None
                    }
                }
            })
            .collect()
    }

    pub fn consume_mouse_moved(&mut self) -> Vec2F {
        let ret = self.mouse_movement;
        self.mouse_movement = Vec2F::new(0.0, 0.0);
        ret
    }

    pub fn process_events(&mut self, events: &Receiver<(f64, glfw::WindowEvent)>, window: &mut glfw::Window) {
        for (_, event) in glfw::flush_messages(events) {
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
                        self.publish_mouse_moved(offset);
                    }
                    self.last_mouse_pos = Some(new_pos);
                }
                glfw::WindowEvent::Key(key_code, _, key_action, _) => {
                    if key_code == Key::Escape && key_action == Action::Press {
                        window.set_should_close(true);
                    }
                    self.publish_key_event(key_code, key_action);
                }
                _ => {}
            }
        }
    }
}
