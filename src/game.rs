#![macro_use]

use player::Player;
// use common::{process_events, process_input};
use camera::Camera;
use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix4, Vector3};
use drawable::{Drawable, Model, Skybox, TextOverlay};
use physics::{Movable, LIGHT_SPEED};
use shader::Shader;
use shader_manager::ShaderManager;
use std::ffi::CStr;
use std::time::SystemTime;

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
  start_time: SystemTime,
}

impl Game {
  pub fn new(height: u32, width: u32) -> Game {
    let shader = Shader::tesselation_pipeline(
      "shaders/tesselation/vs.glsl",
      "shaders/tesselation/fs.glsl",
      "shaders/tesselation/cs.glsl",
      "shaders/tesselation/es.glsl",
    );
    // let shader = Shader::new("shaders/1.model_loading.vs", "shaders/1.model_loading.fs");
    let text_shader = Shader::new("shaders/text/text.vs", "shaders/text/text.fs");
    let sky_shader = Shader::new("shaders/skybox/skybox.vs", "shaders/skybox/skybox.fs");
    let model = Matrix4::from_angle_x(cgmath::Rad::from(cgmath::Deg(0.0)));
    let model = Matrix4::from_scale(0.02) * model; // it's a bit too big for our scene, so scale it down
    let suit = Model::new(
      "resources/objects/Camellia City/OBJ/Camellia_City.obj",
      model,
      "world".to_string(),
    );
    let camera = Player {
      position: Vector3::<f32>::new(1.0, 5.0, -123.0),
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
      start_time: SystemTime::now(),
    }
  }

  pub fn draw(&mut self) {
    let shader = self.shader_manager.get_shader("world".to_string());
    shader.use_program();
    let projection: Matrix4<f32> = perspective(
      Deg(self.player.zoom()),
      self.width as f32 / self.height as f32,
      0.1,
      10000.0,
    );
    let view = self.player.get_view_matrix();

    shader.set_mat4(c_str!("view"), &view);
    shader.set_mat4(c_str!("projection"), &projection);
    shader.set_vector3(c_str!("cameraPos"), &self.player.pos());
    shader.set_vector3(c_str!("cameraVelocity"), &self.player.beta_vector());
    // shader.set_vector3(c_str!("cameraPosVelBasis"), &self.player.posInVelocityBasis());
    shader.set_mat3(c_str!("changeOfBasis"), &self.player.velocity_basis_matrix());
    shader.set_mat3(
      c_str!("changeOfBasisInverse"),
      &self.player.velocity_inverse_basis_matrix(),
    );
    shader.set_float(c_str!("gamma"), self.player.gamma());
    shader.set_float(c_str!("beta"), self.player.beta());
    shader.set_float(c_str!("t"), 0.0);
    shader.set_float(c_str!("c"), LIGHT_SPEED);
    // shader.set_float(c_str!("displacementFactor"), (self.start_time.elapsed().expect("Could not grab elapsed").as_millis() as f32 / 2000.0).sin() / 5.0);

    // render the loaded model

    self.nanosuit.draw(&self.shader_manager);
    self.skybox.set_matrices(&view, &projection);
    let shader = self.shader_manager.get_shader("skybox".to_string());
    shader.use_program();
    // shader.set_mat4(c_str!("view"), &view);
    // shader.set_mat4(c_str!("projection"), &projection);
    shader.set_float(c_str!("gamma"), self.player.gamma());
    shader.set_float(c_str!("beta"), self.player.beta());
    shader.set_float(c_str!("t"), 0.0);
    shader.set_float(c_str!("c"), LIGHT_SPEED);
    shader.set_mat3(c_str!("changeOfBasis"), &self.player.velocity_basis_matrix());
    shader.set_mat3(
      c_str!("changeOfBasisInverse"),
      &self.player.velocity_inverse_basis_matrix(),
    );
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
        self.player.set_velocity(self.player.velocity() * 0.8);
      } else {
        self.player.set_velocity(self.player.velocity() / 10.0);
      }
    }
    self.text_overlay[0].set_data(format!("Position = {}", to_string!(self.player.pos())), 20.0, 20.0);
    self.text_overlay[1].set_data(format!("Gamma = {}", self.player.gamma()), 20.0, 60.0);
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
