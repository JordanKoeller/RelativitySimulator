
extern crate glfw;
use cgmath::prelude::*;

use self::glfw::{Action, Key};

use super::{EventListener, KeyDown};
use renderer::Camera;
use stateful::Scene;
use mechanics::Updatable;
use utils::*;

pub const LIGHT_SPEED: f32 = 36.0;

pub struct MechanicsEngine {
    pub listener: EventListener,
    pub subscribed_events: std::collections::HashMap<u32, fn(&mut Scene)>,
}

impl MechanicsEngine {
    pub fn new(scene: &Scene, listener: EventListener) -> MechanicsEngine {
        let subscribed_events = std::collections::HashMap::new();
        let mut ret = MechanicsEngine {
            listener,
            subscribed_events,
        };
        ret.register_player_events();
        ret
    }

    fn register_player_events(&mut self) {
        self.register_event(KeyDown::key_down(Key::W), |s| {
            let player = s.player_mut();
            let facing = player.front();
            player.motion.apply_user_acceleration(facing.normalize_to(1.0));
        });
        self.register_event(KeyDown::key_down(Key::A), |s| {
            let player = s.player_mut();
            let facing = player.right();
            player.motion.apply_user_acceleration(facing.normalize_to(-1.0));
        });
        self.register_event(KeyDown::key_down(Key::S), |s| {
            let player = s.player_mut();
            let facing = player.front();
            player.motion.apply_user_acceleration(facing.normalize_to(-1.0));
        });
        self.register_event(KeyDown::key_down(Key::D), |s| {
            let player = s.player_mut();
            let facing = player.right();
            player.motion.apply_user_acceleration(facing.normalize_to(1.0));
        });
        self.register_event(KeyDown::key_down(Key::Space), |s| {
            let player = s.player_mut();
            player.apply_user_shift(Vec3F::unit_y().normalize_to(0.05));
        });
        self.register_event(KeyDown::key_down(Key::LeftShift), |s| {
            let player = s.player_mut();
            player.apply_user_shift(-Vec3F::unit_y().normalize_to(0.05));
        });
        self.register_event(KeyDown::key_down(Key::E), |s| {
            s.player_mut().motion.increment_max_speed();
        });
        self.register_event(KeyDown::key_down(Key::Q), |s| {
            s.player_mut().motion.decrement_max_speed();
        });
        self.register_event(KeyDown::key_down(Key::F), |s| {
            let player = s.player_mut();
            player.motion.apply_brakes();
        });
    }

    fn register_event(&mut self, evt: KeyDown, callback: fn(&mut Scene)) {
        let evt_id = self.listener.subscribe_event(evt);
        self.subscribed_events.insert(evt_id, callback);
    }
    pub fn update(&mut self, scene: &mut Scene) {
        for evt in self.listener.consume_events().iter() {
            self.subscribed_events.get(evt).expect("Could not find event key")(scene);
        }
        let delta = 0.1 * self.listener.consume_mouse_moved();
        scene.player_mut().rotate(delta.x, delta.y);
        scene.player_mut().update(0.100);
    }
}
