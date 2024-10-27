use cgmath::InnerSpace;
use specs::{prelude::*, Component, HashMapStorage};

use engine::{
  ecs::{MonoBehavior, Player, SystemUtilities, WorldProxy},
  gui::{widgets::*, ControlPanelBuilder, SystemDebugger},
  physics::RigidBody,
  utils::Vec3F,
};

#[derive(Debug, Component)]
#[storage(HashMapStorage)]
pub struct RelativityParameters {
  beta: f32,
  gamma: f32,
  c: f32,
}

impl RelativityParameters {
  pub fn with_c(c: f32) -> Self {
    Self {
      beta: 0f32,
      gamma: 1f32,
      c,
    }
  }
}

#[derive(SystemData)]
pub(crate) struct RelativitySystemData<'a> {
  player: ReadStorage<'a, Player>,
  motion: ReadStorage<'a, RigidBody>,
  relativity_params: WriteStorage<'a, RelativityParameters>,
}

#[derive(Default)]
pub(crate) struct Relativity;

impl<'a> MonoBehavior<'a> for Relativity {
  type SystemData = RelativitySystemData<'a>;

  fn run(&mut self, api: SystemUtilities<'a>, mut s: Self::SystemData) {
    for (_p, motion, relativity) in (&s.player, &s.motion, &mut s.relativity_params).join() {
      // Compute the relativity parameters
      relativity.beta = motion.velocity.dot(motion.velocity).sqrt() / relativity.c;
      relativity.gamma = 1f32 / (1f32 - relativity.beta * relativity.beta).sqrt();

      let mut panel = self.get_write_panel(&api);
      panel.set_str("Beta", format!("{:3}", relativity.beta));
    }
  }

  fn setup(&mut self, world: WorldProxy) {
    self.register_debugger(&world);
  }
}

impl<'a> SystemDebugger<'a> for Relativity {
  fn create_panel(&self) -> engine::gui::ControlPanelBuilder {
    ControlPanelBuilder::default()
      .with_title("Relativity")
      .push_line("Beta", LabeledText::new("0.0", "Beta"))
  }
}

impl Relativity {
  pub fn three_acceleration(force: &Vec3F, relativity: &RelativityParameters, rigid: &RigidBody) -> Vec3F {
    let mass = 1f32;

    (force - force.dot(rigid.velocity) * rigid.velocity / relativity.c / relativity.c) / mass / relativity.gamma
  }
}
