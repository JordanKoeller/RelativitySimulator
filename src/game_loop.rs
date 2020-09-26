use std::cell::{RefCell, RefMut};
use utils::*;
use renderer::{Window, Drawable, Renderer, Camera, RenderCommand};
use app::Player;
use events::WindowEventManager;

pub type Scene = Vec<Ref<dyn Drawable>>;

pub struct GameLoop {
  window: Window,
  listener: MutRef<WindowEventManager>,
  player: Player,
  renderer: Renderer,
  scene: Scene
}

impl GameLoop {
  pub fn run(&mut self) {
    // let mut delta_time = 0f32;
    let mut last_time = 0f32;

    while self.window.is_open() {
      let curr_time = self.window.glfw_token.get_time() as f32;
      let delta_time = curr_time - last_time;
      last_time = curr_time;

      self.window.frame_start();
      self.listener.borrow_mut().process_events(&mut self.window);

      self.player.update(delta_time);

      self.renderer.start_scene(&self.player as &dyn Camera);

      for asset in self.scene.iter() {
        self.renderer.submit(RenderCommand::from(Ref::clone(&asset)));
      }

      self.renderer.draw_scene(&mut self.window);

      self.window.frame_end();
    }
  }
}


// Constructors
impl GameLoop {
  pub fn new(
    window: Window,
    listener: MutRef<WindowEventManager>,
    player: Player,
    renderer: Renderer,
    scene: Scene
  ) -> GameLoop {
    GameLoop {
      window,
      listener,
      player,
      renderer,
      scene
    }
  }

}