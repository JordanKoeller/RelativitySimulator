#![allow(dead_code)]
#![allow(unused_imports)]

extern crate glfw;

extern crate cgmath;
extern crate gl;
extern crate image;
extern crate imgui;
extern crate imgui_glfw_rs;
extern crate imgui_opengl_renderer;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate specs;
extern crate tobj;
extern crate rand;

mod macros;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod debug;
mod common;
mod ecs;
mod physics;
mod events;
mod game_loop;
mod renderer;
mod utils;
mod gui;
mod shapes;

mod app;

use events::{Event, EventChannel, KeyCode, WindowEvent, StatelessEventChannel};
use utils::{Vec3F};

use specs::{World, WorldExt};
use game_loop::GameLoop;


// settings
pub const SCR_WIDTH: u32 = 1600;
pub const SCR_HEIGHT: u32 = 1200;

pub fn main() {
  // glfw: initialize and configure
  // ------------------------------

  // Initialize high-level "singleton" structures
  // --------------------------------------------
  let mut window_event_channel = StatelessEventChannel::<WindowEvent>::default();
  let window = renderer::Window::new(SCR_WIDTH, SCR_HEIGHT, "Special Relativity");
  let mut render = renderer::Renderer::new(
    utils::Vec2F::new(SCR_WIDTH as f32, SCR_HEIGHT as f32),
    &mut window_event_channel,
  );
  // Initialize the player/camera and the respective event handling
  // --------------------------------------------------------------

  // Initialize assets/shaders
  // -------------------------
  // let cube = app::ColoredCube::new(Vec3F::new(2.0, 0.0, -10.0), Color::new(0.5, 0.2, 0.8));
  // let cube: Ref<dyn renderer::Drawable> = Ref::new(cube);
  let shader = renderer::Shader::from_file("debug", "shaders/debug_tessellation.glsl");
  render.submit_shader(shader);
  let shader = renderer::Shader::from_file("default", "shaders/simple_shader.glsl");
  render.submit_shader(shader);
  let shader = renderer::Shader::from_file("default_texture", "shaders/simple_textured.glsl");
  render.submit_shader(shader);
  let shader = renderer::Shader::from_file("instanced", "shaders/simple_instanced.glsl");
  render.submit_shader(shader);
  let shader = renderer::Shader::from_file_skybox("skybox", "shaders/skybox.glsl");
  render.submit_shader(shader);
  let shader = renderer::Shader::from_file("lorentz", "shaders/lorentz.glsl");
  render.submit_shader(shader);
  let shader = renderer::Shader::from_file("face_cube", "shaders/face_cube.glsl");
  render.submit_shader(shader);


  let mut world = World::new();

  let world_id = window_event_channel.register_with_subs(&[
    WindowEvent::new(Event::KeyPressed(KeyCode::Control)),
    WindowEvent::new(Event::KeyPressed(KeyCode::Esc)),
  ]);


  world.insert(window_event_channel);
  world.insert(utils::Timestep(0.016, 0.01));
  world.insert(utils::Running(true));
  world.insert(render);
  world.insert(world_id);
  app::flappy_bird::setup_world(&mut world);
  // app::build_city(&mut world);

  let mut runtime = GameLoop::new(window, world, world_id);
  runtime.with_systems(app::flappy_bird::get_system_registration());
  runtime.run();
}
