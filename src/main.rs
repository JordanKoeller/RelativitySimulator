extern crate glfw;

// use self::glfw::Context;
// use self::glfw::{Action, Key};
use std::cell::{RefCell, RefMut};
// use std::sync::mpsc::Receiver;
use std::ffi::{CStr, CString};

extern crate cgmath;
extern crate gl;
extern crate image;
extern crate imgui;
extern crate imgui_glfw_rs;
extern crate imgui_opengl_renderer;
extern crate regex;
extern crate tobj;

mod macros;

#[macro_use]
extern crate lazy_static;

mod common;
mod events;
mod imgui_api;
mod physics;
mod renderer;
mod scene;
mod utils;
mod game_loop;

mod app;

use events::{EventDispatcher, WindowEventManager};
use renderer::Camera;
use utils::{MutRef, Ref, Vec3F};

use imgui::*;

// settings
pub const SCR_WIDTH: u32 = 1600;
pub const SCR_HEIGHT: u32 = 1200;

pub fn main() {
  // glfw: initialize and configure
  // ------------------------------

  // Initialize high-level "singleton" structures
  // --------------------------------------------
  let listener: MutRef<WindowEventManager> = Ref::new(RefCell::new(WindowEventManager::default()));
  let window = renderer::Window::new(SCR_WIDTH, SCR_HEIGHT, "Special Relativity");
  let mut render = renderer::Renderer::new(utils::Vec2F::new(SCR_WIDTH as f32, SCR_HEIGHT as f32));

  // Initialize the player/camera and the respective event handling
  // --------------------------------------------------------------
  let player = app::Player::default(Ref::clone(&listener) as MutRef<dyn EventDispatcher>);

  // Initialize assets/shaders
  // -------------------------
  // let cube = app::ColoredCube::new(Vec3F::new(2.0, 0.0, -10.0), Color::new(0.5, 0.2, 0.8));
  // let cube: Ref<dyn renderer::Drawable> = Ref::new(cube);
  let shader = renderer::Shader::from_file("default", "shaders/simple_shader.glsl");
  render.submit_shader(shader);
  let shader = renderer::Shader::from_file("default_texture", "shaders/simple_textured.glsl");
  render.submit_shader(shader);

  let resources = [
    "resources/textures/awesomeface.png",
    "resources/textures/brickwall.jpg",
    "resources/textures/container.jpg",
    "resources/textures/marble.jpg",
    "resources/textures/wood.png",
    "resources/textures/checkerboard.png",
  ];
  let cubes: Vec<Ref<dyn renderer::Drawable>> = (0..36)
    .map(|x| {
      let tex_id = x % 6;
      let angle = (x as f32 / 36.0) * 2f32 * std::f32::consts::PI;
      let pos = Vec3F::new(angle.cos() * 5f32, 0.0 + (x as f32 / 100f32), angle.sin() * 5f32);
      let tex_cube = app::TexturedCube::new(pos, resources[tex_id]);
      let ret: Ref<dyn renderer::Drawable> = Ref::new(tex_cube);
      ret
    })
    .collect();

    let mut game_loop = game_loop::GameLoop::new(
      window,
      listener,
      player,
      render,
      cubes
    );

    game_loop.run();
}
