
#[macro_use]
extern crate engine;
extern crate specs;
extern crate cgmath;


mod prefabs;
mod systems;
mod components;
mod dispatcher;
mod skybox;

fn main() {
    engine::main();
}
