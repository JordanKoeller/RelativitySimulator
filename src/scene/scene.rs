use cgmath::prelude::*;
use utils::*;

use renderer::{Camera, Drawable};


pub struct Scene {
  camera: Ref<dyn Camera>,
  drawables: Vec<MutRef<dyn Drawable>>,
  updatables: Vec<MutRef<dyn super::Updatable>>,
}

impl Scene {
  fn update(&mut self, dt: Timestep) {
    // TODO
  }

  fn render(&self) {

  }

}
