#![macro_use]

use player::Player;
// use common::{process_events, process_input};
use camera::Camera;
use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix4, Vector3};
use drawable::{Drawable, Model, Skybox, TextOverlay};
use physics::Movable;
use shader::Shader;
use shader_manager::ShaderManager;
use std::ffi::CStr;

extern crate glfw;
use self::glfw::{Action, Key};

pub struct Game {
  nanosuit: Model,
  player: Player,
  shader_manager: ShaderManager,
  text_overlay: [TextOverlay; 3],
  skybox: Skybox,
  width: u32,
  height: u32,
  pressed_keys: [bool; 512],
}

impl Game {
  pub fn new(height: u32, width: u32) -> Game {
    let shader = Shader::new("shaders/1.model_loading.vs", "shaders/1.model_loading.fs");
    let text_shader = Shader::new("shaders/text/text.vs", "shaders/text/text.fs");
    let sky_shader = Shader::new("shaders/skybox/skybox.vs", "shaders/skybox/skybox.fs");
    let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0)); // translate it down so it's at the center of the scene
    model = model * Matrix4::from_scale(0.2); // it's a bit too big for our scene, so scale it down
    let suit = Model::new("resources/objects/nanosuit/nanosuit.obj", model, "world".to_string());
    let camera = Player {
      position: Vector3::<f32>::new(0.0, 0.0, 3.0),
      ..Player::default()
    };
    let mut shader_manager = ShaderManager::new();
    shader_manager.add_shader("world".to_string(), shader);
    shader_manager.add_shader("text".to_string(), text_shader);
    shader_manager.add_shader("skybox".to_string(), sky_shader);
    Game {
      nanosuit: suit,
      player: camera,
      shader_manager: shader_manager,
      text_overlay: [TextOverlay::new(), TextOverlay::new(), TextOverlay::new()],
      skybox: Skybox::new([
        "resources/Skybox/right.jpg".to_string(),
        "resources/Skybox/left.jpg".to_string(),
        "resources/Skybox/top.jpg".to_string(),
        "resources/Skybox/bottom.jpg".to_string(),
        "resources/Skybox/front.jpg".to_string(),
        "resources/Skybox/back.jpg".to_string(),
      ]),
      height: height,
      width: width,
      pressed_keys: [false; 512],
    }
  }

  pub fn draw(&mut self) {
    let shader = self.shader_manager.get_shader("world".to_string());
    shader.use_program();
    let projection: Matrix4<f32> = perspective(
      Deg(self.player.zoom()),
      self.width as f32 / self.height as f32,
      0.1,
      100.0,
    );
    let view = self.player.get_view_matrix();
    shader.set_mat4(c_str!("projection"), &projection);
    shader.set_mat4(c_str!("view"), &view);

    // render the loaded model

    self.nanosuit.draw(&self.shader_manager);
    self.skybox.set_matrices(&view, &projection);
    self.skybox.draw(&self.shader_manager);

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

    self.player.integrate(dt);
    self.player.clear_acceleration();
    if self.pressed_keys[Key::LeftShift as usize] {
      if self.player.velocity().magnitude() > 1.0 {
        self.player.set_velocity(self.player.velocity() * 0.99);
      } else {
        self.player.set_velocity(self.player.velocity() / 10.0);
      }
    }
    self.text_overlay[0].set_data(format!("position = {}", to_string!(self.player.pos())), 20.0, 20.0);
    self.text_overlay[1].set_data(format!("velocity = {}", to_string!(self.player.velocity())), 20.0, 60.0);
    self.text_overlay[2].set_data(format!("fps = {:.0}", 1.0 / dt), 20.0, 1160.0);
  }

  pub fn key_action(&mut self, key: Key, action: Action) {
    match action {
      Action::Press => self.pressed_keys[key as usize] = true,
      Action::Release => self.pressed_keys[key as usize] = false,
      _ => println!("Had a repeat action"),
    };
  }

  pub fn mouse_moved(&mut self, dx: f32, dy: f32) {
    let sensitivity = 0.1;
    self.player.rotate(dx * sensitivity, dy * sensitivity);
  }
}
