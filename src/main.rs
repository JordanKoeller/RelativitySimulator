extern crate freetype;
extern crate glfw;

use self::glfw::Context;
use self::glfw::{Action, Key};
use std::sync::mpsc::Receiver;
// use std::ffi::CStr;

extern crate cgmath;
extern crate gl;
extern crate image;
extern crate regex;
extern crate tobj;

mod macros;

#[macro_use]
extern crate lazy_static;

mod initializers;
mod mechanics;
mod renderer;
mod scenes;
mod stateful;

mod common;
mod utils;
// mod debug;

use mechanics::user_input::EventListener;
use renderer::{Camera, IShader};
use utils::{Color, Vec3F};

// settings
pub const SCR_WIDTH: u32 = 1600;
pub const SCR_HEIGHT: u32 = 1200;

pub fn main() {
  // glfw: initialize and configure
  // ------------------------------
  let (mut window, events, mut glfw) = initializers::window_init(SCR_WIDTH, SCR_HEIGHT, "Relativity Simulation");
  // let mut game = game::Game::new(SCR_HEIGHT, SCR_WIDTH);
  let mut asset_manager = initializers::AssetManager::default();

  // Initialize the scene
  let mut scene = scenes::CubeScene::get_scene(&mut asset_manager);

  // Bind the scene to the renderer and the mechanics engine
  let renderer = renderer::Renderer::new();
  let listener = mechanics::EventListener::default();
  let mut engine = mechanics::MechanicsEngine::new(&scene, listener);
  // Render wireframe
  // unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) };
  // render loop
  // -----------
  while !window.should_close() {
    // per-frame time logic
    // --------------------
    // events
    // -----
    engine.listener.process_events(&events, &mut window);
    engine.update(&mut scene);
    // render
    // ------
    unsafe {
      gl::ClearColor(0.1, 0.2, 0.3, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
    renderer.render(&scene);
    // don't forget to enable shader before setting uniforms

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    // -------------------------------------------------------------------------------
    window.swap_buffers();
    glfw.poll_events();
  }
}
