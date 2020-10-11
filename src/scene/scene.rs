use cgmath::prelude::*;
use std::os::raw::c_void;
use utils::*;

use renderer::{Camera, RenderCommand};
use app::Player;

use super::{Entity, Renderable};

pub struct Scene {
  player: Player,
  renderables: Vec<MutRef<Box<dyn Renderable>>>,
  scriptables: Vec<MutRef<Box<dyn Renderable>>>,
  collidables: Vec<MutRef<Box<dyn Renderable>>>,
  kinematics: Vec<MutRef<Box<dyn Renderable>>>,
}

impl Scene {
  pub fn update(&mut self, dt: Timestep) {
    self.player.update(dt);
    // TODO
    // self.process_events();
    // self.run_scripts();
    // self.run_physics();
  }

  pub fn draw(&self) -> Vec<RenderCommand> {
    let mut render_buff = Vec::new();
    self.renderables.iter().for_each(|ent| {
      render_buff.push(ent.borrow().draw());
    });
    render_buff
  }


  pub fn register_renderable(&mut self, entity: MutRef<Box<dyn Renderable>>) {
    self.renderables.push(entity);
  }

  // pub fn camera(&self) -> Ref<dyn Camera> {
  //   Ref::clone(&self.camera)
  // }

  pub fn player(&self) -> &Player {
    &self.player
  }

  pub fn player_mut(&mut self) -> &mut Player {
    &mut self.player
  }

  pub fn new(ents: Vec<Box<dyn Entity>>, player: Player) -> Scene {
    let mut ret = Scene::new_helper(player);
    for e in ents {
      e.register(&mut ret);
    }
    ret
  }

  fn new_helper(player: Player) -> Scene {
    Scene {
      player,
      renderables: Vec::default(),
      scriptables: Vec::default(),
      collidables: Vec::default(),
      kinematics:  Vec::default(),
    }
  }
}
