#![macro_use]

use player::Player;
// use common::{process_events, process_input};
use camera::Camera;
use cgmath::prelude::*;
use drawable::{Drawable, TextOverlay};
use physics::{Movable};
use shader::Shader;
use shader_manager::ShaderManager;

use scene::Scene;
use city_scene::procedure_scene;

extern crate glfw;
use self::glfw::{Action, Key};

pub struct Game {
  player: Player,
  scene: Scene,
  shader_manager: ShaderManager,
  text_overlay: [TextOverlay; 4],
  lorentz_flag: i32,
  pressed_keys: [bool; 512],
}

impl Game {
  pub fn new(height: u32, width: u32) -> Game {
    let text_shader = Shader::new("shaders/text/text.vs", "shaders/text/text.fs");
    let mut shader_manager = ShaderManager::new();
    shader_manager.add_shader("text".to_string(), text_shader);
    // let (scene, camera) = Scene::city_scene(&mut shader_manager, width as f32, height as f32);
    // let (scene, camera) = Scene::grid_scene(&mut shader_manager, width as f32, height as f32);
    // let (scene, camera) = Scene::colorbox_scene(&mut shader_manager, width as f32, height as f32);
    let (scene, camera) = procedure_scene(&mut shader_manager, width as f32, height as f32);
    Game {
      player: camera,
      scene: scene,
      shader_manager: shader_manager,
      text_overlay: [TextOverlay::new(), TextOverlay::new(), TextOverlay::new(), TextOverlay::new()],
      lorentz_flag: 0,
      pressed_keys: [false; 512],
    }
  }

  pub fn draw(&mut self) {

    self.scene.draw(&self.player, &self.shader_manager, self.lorentz_flag);
    for overlay in &self.text_overlay {
      overlay.draw(&self.shader_manager);
    }
  }

  pub fn update(&mut self, dt: f32) {
    if self.pressed_keys[Key::W as usize] {
      self.player.apply_acceleration(self.player.front().normalize_to(4.0));
    }
    if self.pressed_keys[Key::A as usize] {
      self.player.apply_acceleration(-self.player.right().normalize_to(4.0));
    }
    if self.pressed_keys[Key::S as usize] {
      self.player.apply_acceleration(-self.player.front().normalize_to(4.0));
    }
    if self.pressed_keys[Key::D as usize] {
      self.player.apply_acceleration(self.player.right().normalize_to(4.0));
    }
    if self.pressed_keys[Key::E as usize] {
      self.lorentz_flag = (self.lorentz_flag + 1) % 3;
      // println!("Setting flag to {}", self.lorentz_flag);
      self.pressed_keys[Key::E as usize] = false; // Turn off boolean to not trigger over and over again.
    }
    self.player.integrate(dt);
    self.player.clear_acceleration();
    if self.pressed_keys[Key::LeftShift as usize] {
      if self.player.velocity().magnitude() > 1.0 {
        self.player.set_velocity(self.player.velocity() * 0.8);
      } else {
        self.player.set_velocity(self.player.velocity() / 10.0);
      }
    }
    self.text_overlay[0].set_data(format!("Looking = {}", to_string!(self.player.front())), 20.0, 20.0);
    self.text_overlay[1].set_data(format!("Beta = {}", self.player.beta()), 20.0, 60.0);
    self.text_overlay[2].set_data(format!("fps = {:.0}", 1.0 / dt), 20.0, 1160.0);
    self.text_overlay[3].set_data(format!("Flag = {}", self.lorentz_flag), 20.0, 1120.0);
  }

  pub fn key_action(&mut self, key: Key, action: Action) {
    match action {
      Action::Press => self.pressed_keys[key as usize] = true,
      Action::Release => self.pressed_keys[key as usize] = false,
      _ => {},
    };
  }

  pub fn mouse_moved(&mut self, dx: f32, dy: f32) {
    let sensitivity = 0.1;
    self.player.rotate(dx * sensitivity, dy * sensitivity);
  }
}
