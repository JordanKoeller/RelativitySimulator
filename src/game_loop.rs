use utils::*;
use renderer::{Window, Drawable, Renderer, Camera, RenderCommand, OverlayLine};
use app::Player;
use events::{WindowEventManager, EventDispatcher};
use scene::Scene;
use std::rc::Rc;
use std::cell::RefCell;

pub struct GameLoop {
  window: Window,
  listener: MutRef<WindowEventManager>,
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

      self.listener.borrow_mut().process_events(&mut self.window);
      self.scene.update(delta_time);
      self.window.frame_start();
      self.renderer.start_scene(self.scene.player() as &dyn Camera);

      // Debug UI ///////////////////////
      self.debug_panel(delta_time);
      // //////////////////////////////
      let buff = self.scene.draw();
      for asset in buff.iter() {
        self.renderer.submit(asset.clone());
      }
      self.renderer.draw_scene(&mut self.window);
      self.window.frame_end();
    }
  }


  fn debug_panel(&mut self, dt: f32) {
    let mut debug_ui = self.renderer.ui_box("Debug");
    debug_ui.push(OverlayLine::LabelText(
      "Frame Time".to_string(),
      format!("{0:.2} ms", dt * 1000f32)
    ));

    let listener = self.listener.borrow();
    for (evt, _) in listener.global_subscribed_events().iter() {
      let evt_box = listener.global_event_inbox().get(evt);
      if let Some(payload) = evt_box {
          debug_ui.push(OverlayLine::LabelText(
            format!("{:?}", evt),
            match payload {
              Some(payload) => format!("{:?}", payload),
              None => "ACTIVE".to_string()
            }
          ));
      } else {
        debug_ui.push(OverlayLine::LabelText(
          format!("{:?}", evt),
          "UNACTIVE".to_string()
        ));
      }
    }

    self.renderer.submit_2d(debug_ui);
  }
}


// Constructors
impl GameLoop {
  pub fn new(
    window: Window,
    listener: MutRef<WindowEventManager>,
    renderer: Renderer,
    scene: Scene
  ) -> GameLoop {
    GameLoop {
      window,
      listener,
      renderer,
      scene
    }
  }

}