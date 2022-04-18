#![allow(dead_code)]
#![allow(unused_imports)]

extern crate glfw;

extern crate cgmath;
extern crate either;
extern crate gl;
extern crate image;
extern crate imgui;
extern crate imgui_glfw_rs;
extern crate imgui_opengl_renderer;
extern crate rand;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate specs;
extern crate tobj;
extern crate crossbeam_queue;

pub mod macros;

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod debug;
mod game_builder;
pub mod common;
pub mod ecs;
pub mod events;
pub mod game_loop;
pub mod gui;
pub mod physics;
pub mod renderer;
pub mod shapes;
pub mod utils;
pub mod testing;
mod datastructures;
mod graphics;
mod platform;
// mod app;

use crate::game_builder::GameBuilder;
use crate::events::{Event, EventChannel, KeyCode, StatelessEventChannel, WindowEvent};
use crate::game_loop::GameLoop;
use crate::utils::Vec3F;
use specs::{World, WorldExt};

pub use crate::ecs::prefab::*;

// settings
pub const SCR_WIDTH: u32 = 1600;
pub const SCR_HEIGHT: u32 = 1200;

pub fn get_game_builder<'a, 'b>() -> GameBuilder<'a, 'b> {
    let window = platform::Window::new(SCR_WIDTH, SCR_HEIGHT, "Special Relativity");
    GameBuilder::new(window)
}

pub fn main(builder: GameBuilder) {
    // glfw: initialize and configure
    // ------------------------------

    // Initialize high-level "singleton" structures
    // --------------------------------------------

    // Initialize the player/camera and the respective event handling
    // --------------------------------------------------------------

    // Initialize assets/shaders
    // -------------------------
    // let cube = app::ColoredCube::new(Vec3F::new(2.0, 0.0, -10.0), Color::new(0.5, 0.2, 0.8));
    // let cube: Ref<dyn renderer::Drawable> = Ref::new(cube);
    // let shader = renderer::Shader::from_file("debug", "shaders/debug_tessellation.glsl");
    // render.submit_shader(shader);
    // let shader = renderer::Shader::from_file("default", "shaders/simple_shader.glsl");
    // render.submit_shader(shader);

    // let shader = renderer::Shader::from_file("lorentz", "shaders/lorentz.glsl");
    // render.submit_shader(shader);
    // let shader = renderer::Shader::from_file("face_cube", "shaders/face_cube.glsl");
    // render.submit_shader(shader);

    // app::build_city(&mut world);

    // app::flappy_bird::setup_world(&mut world);
    // app::minecraft::setup_world(&mut world);
    // let mut runtime = GameLoop::new(window, world, world_id);
    // runtime.with_systems(app::minecraft::get_system_registration());
    // runtime.with_systems(app::flappy_bird::get_system_registration());
    // runtime.run();
    let mut runtime = builder.build();
    runtime.run();
}
