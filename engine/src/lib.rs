#![allow(dead_code)]
#![allow(unused_imports)]

extern crate glfw;

extern crate cgmath;
extern crate crossbeam_queue;
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
#[macro_use]
extern crate log;
#[macro_use]
extern crate env_logger;
// #[macro_use]
// extern crate tokio;

pub mod macros;

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod debug;
pub mod common;
mod datastructures;
pub mod ecs;
pub mod events;
mod game_builder;
pub mod game_loop;
pub mod graphics;
pub mod gui;
pub mod physics;
mod platform;
pub mod renderer;
pub mod testing;
pub mod utils;
pub mod net;
// mod app;

use crate::events::{Event, EventChannel, KeyCode, StatelessEventChannel, WindowEvent};
use crate::game_builder::GameBuilder;
use crate::game_loop::GameLoop;
use crate::utils::Vec3F;

use std::thread;
use specs::{World, WorldExt};

pub use crate::ecs::prefab;

pub use log::info;

// settings
pub const SCR_WIDTH: u32 = 1600;
pub const SCR_HEIGHT: u32 = 900;

pub fn get_game_builder<'a, 'b>() -> GameBuilder<'a, 'b> {
    env_logger::init();
    let window = platform::Window::new(SCR_WIDTH, SCR_HEIGHT, "Special Relativity");
    GameBuilder::new(window)
}

pub fn main(builder: GameBuilder) {
    let mut runtime = builder.build();
    runtime.run();

}
