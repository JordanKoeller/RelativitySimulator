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

mod macros;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod debug;
mod common;
mod ecs;
mod events;
mod game_loop;
mod renderer;
mod utils;

mod app;

use events::{Event, EventChannel, KeyCode, WindowEvent, WindowEventChannel};
use renderer::Drawable;
use utils::{Vec2F, Vec3F};

use specs::{Builder, DispatcherBuilder, World, WorldExt};

use ecs::components::*;
use ecs::systems::player_events::PlayerEvents;

// settings
pub const SCR_WIDTH: u32 = 1600;
pub const SCR_HEIGHT: u32 = 1200;

pub fn main() {
  // glfw: initialize and configure
  // ------------------------------

  // Initialize high-level "singleton" structures
  // --------------------------------------------
  let mut window_event_receiver = WindowEventChannel::default();
  let mut window_event_channel = EventChannel::<WindowEvent>::default();
  let mut window = renderer::Window::new(SCR_WIDTH, SCR_HEIGHT, "Special Relativity");
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
  let shader = renderer::SimpleShader::from_file("default", "shaders/simple_shader.glsl");
  render.submit_shader(Box::from(shader));
  let shader = renderer::SimpleShader::from_file("default_texture", "shaders/simple_textured.glsl");
  render.submit_shader(Box::from(shader));
  let shader = renderer::SkyboxShader::from_file("skybox", "shaders/skybox.glsl");
  render.submit_shader(Box::from(shader));
  let shader = renderer::SimpleShader::from_file("lorentz", "shaders/lorentz.glsl");
  render.submit_shader(Box::from(shader));

  let mut g_loop = game_loop::GameLoop::new(window_event_channel.register_with_subs(&[
    WindowEvent::new(Event::KeyPressed(KeyCode::Control)),
    WindowEvent::new(Event::KeyPressed(KeyCode::Esc)),
  ]));

  let mut world = World::new();

  let mut dispatcher = app::setup_dispatcher();

  dispatcher.setup(&mut world);

  // let resources = [
  //   "resources/textures/awesomeface.png",
  //   "resources/textures/brickwall.jpg",
  //   "resources/textures/container.jpg",
  //   "resources/textures/marble.jpg",
  //   "resources/textures/wood.png",
  //   "resources/textures/checkerboard.png",
  // ];
  // (0..36).for_each(|x| {
  //   let tex_id = x % 6;
  //   let angle = (x as f32 / 36.0) * 2f32 * std::f32::consts::PI;
  //   let pos = Vec3F::new(angle.cos() * 5f32, 0.0 + (x as f32 / 100f32), angle.sin() * 5f32);
  //   let tex_cube = app::TexturedCube::new(resources[tex_id]);
  //   let transform = utils::translate(pos);
  //   world
  //     .create_entity()
  //     .with(tex_cube.renderable())
  //     .with(Transform(transform))
  //     .build();
  // });
  // world
  //   .create_entity()
  //   .with(app::Skybox::new("resources/skybox").renderable())
  //   .build();

  world.insert(window_event_channel);
  world.insert(utils::Timestep(0.016));
  world.insert(render);
  app::scenes::build_grid_scene(Vec3F::new(5f32, 0f32, 0f32), &mut world);
  g_loop.run(&mut dispatcher, &mut world, &mut window, &mut window_event_receiver);
}
