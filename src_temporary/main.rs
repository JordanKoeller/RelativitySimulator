extern crate glfw;
extern crate freetype;

use self::glfw::Context;
use self::glfw::{Action, Key};
use std::sync::mpsc::Receiver;
// use std::ffi::CStr;

extern crate cgmath;
extern crate gl;
extern crate image;
extern crate tobj;
extern crate regex;



mod macros;

#[macro_use]
extern crate lazy_static;

mod renderer;
mod initializers;


mod drawable;
// mod text_overlay;
mod camera;
mod common;
mod game;
// mod mesh;
// mod model;
// mod shader;
mod utils;
mod physics;
mod player;
mod scene;
mod city_scene;


// settings
const SCR_WIDTH: u32 = 1600;
const SCR_HEIGHT: u32 = 1200;

pub fn main() {
  let mut first_move = true;
  let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
  let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;

  // timing
  let mut delta_time: f32; // time between current frame and last frame
  let mut last_frame: f32 = 0.0;

  // glfw: initialize and configure
  // ------------------------------
  let (mut window, events, mut glfw) = initializers::window_init(SCR_WIDTH, SCR_HEIGHT, "Relativity Simulation");
  let mut game = game::Game::new(SCR_HEIGHT, SCR_WIDTH);

  // Render wireframe
  // unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) };

  // render loop
  // -----------
  while !window.should_close() {
    // per-frame time logic
    // --------------------
    let current_frame = glfw.get_time() as f32;
    delta_time = current_frame - last_frame;
    last_frame = current_frame;

    // events
    // -----
    process_window_events(&events, &mut window, &mut first_move, &mut last_x, &mut last_y, &mut game);

    // // input
    // // -----
    // render
    // ------
    unsafe {
      gl::ClearColor(0.0, 0.0, 0.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    game.update(delta_time);

    game.draw();
    // don't forget to enable shader before setting uniforms

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    // -------------------------------------------------------------------------------
    window.swap_buffers();
    glfw.poll_events();
  }
}

fn process_window_events(
  events: &Receiver<(f64, glfw::WindowEvent)>,
  window: &mut glfw::Window,
  first_mouse: &mut bool,
  last_x: &mut f32,
  last_y: &mut f32,
) {
  for (_, event) in glfw::flush_messages(events) {
    match event {
      glfw::WindowEvent::FramebufferSize(width, height) => {
        // make sure the viewport matches the new window dimensions; note that width and
        // height will be significantly larger than specified on retina displays.
        unsafe { gl::Viewport(0, 0, width, height) }
      }
      glfw::WindowEvent::CursorPos(xpos, ypos) => {
        let (xpos, ypos) = (xpos as f32, ypos as f32);
        if *first_mouse {
          *last_x = xpos;
          *last_y = ypos;
          *first_mouse = false;
        }

        let xoffset = xpos - *last_x;
        let yoffset = *last_y - ypos; // reversed since y-coordinates go from bottom to top

        *last_x = xpos;
        *last_y = ypos;

        game.mouse_moved(xoffset, yoffset);
      }
      glfw::WindowEvent::Key(key_code, _, key_action, _) => {
        if key_code == Key::Escape && key_action == Action::Press {
          window.set_should_close(true);
        }
        game.key_action(key_code, key_action);
      }
      _ => {}
    }
  }
}
