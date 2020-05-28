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


mod macros;

#[macro_use]
extern crate lazy_static;

mod drawable;
// mod text_overlay;
mod camera;
mod common;
mod game;
// mod mesh;
// mod model;
mod shader_manager;
mod shader;
mod utils;
mod physics;
mod player;
// settings
const SCR_WIDTH: u32 = 1600;
const SCR_HEIGHT: u32 = 1200;

const LISTEN_FOR_KEYS: [Key; 6] = [Key::W, Key::A, Key::S, Key::D, Key::Q, Key::LeftShift];

pub fn main() {
  let mut first_move = true;
  let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
  let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;

  // timing
  let mut delta_time: f32; // time between current frame and last frame
  let mut last_frame: f32 = 0.0;

  // glfw: initialize and configure
  // ------------------------------
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
  #[cfg(target_os = "macos")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  // glfw window creation
  // --------------------
  let (mut window, events) = glfw
    .create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window");

  window.make_current();
  window.set_framebuffer_size_polling(true);
  window.set_cursor_pos_polling(true);
  window.set_scroll_polling(true);

  // tell GLFW to capture our mouse
  window.set_cursor_mode(glfw::CursorMode::Disabled);

  // gl: load all OpenGL function pointers
  // ---------------------------------------
  gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
  unsafe {
    gl::Enable(gl::CULL_FACE);
    gl::Enable(gl::DEPTH_TEST);
    gl::Enable(gl::MULTISAMPLE);
    gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

  }

  let mut game = game::Game::new(SCR_HEIGHT, SCR_WIDTH);

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
    process_window_events(&events, &mut first_move, &mut last_x, &mut last_y, &mut game);
    // process_events(&events, &mut first_move, &mut last_x, &mut last_y, &mut camera);

    // // input
    // // -----
    // process_input(&mut window, delta_time, &mut camera);
    process_key_actions(&mut window, &mut game);

    // render
    // ------
    unsafe {
      gl::ClearColor(0.1, 0.1, 0.1, 1.0);
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

fn process_key_actions(window: &mut glfw::Window, game: &mut game::Game) {
  if window.get_key(Key::Escape) == Action::Press {
    window.set_should_close(true)
  }
  for k in &LISTEN_FOR_KEYS {
    game.key_action(*k, window.get_key(*k));
  }
}

fn process_window_events(
  events: &Receiver<(f64, glfw::WindowEvent)>,
  first_mouse: &mut bool,
  last_x: &mut f32,
  last_y: &mut f32,
  game: &mut game::Game,
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

      _ => {}
    }
  }
}
