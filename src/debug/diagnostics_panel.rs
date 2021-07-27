use ecs::components::{Player};
use gui::*;
use specs::prelude::*;
use utils::Timestep;

use physics::{TransformComponent, RigidBody};

pub struct DiagnosticsPanel;

impl<'a> System<'a> for DiagnosticsPanel {
  type SystemData = (
    WriteStorage<'a, Player>,
    ReadStorage<'a, TransformComponent>,
    ReadStorage<'a, RigidBody>,
    WriteStorage<'a, GuiInputPanel>,
    Read<'a, Timestep>,
  );

  fn run(&mut self, (mut s_player, s_transform, s_rigid, mut s_panel, timestep): Self::SystemData) {
    for (_player, transform, _rigid_body, panel) in
      (&mut s_player, &s_transform, &s_rigid, &mut s_panel).join()
    {
      if panel.empty() {
        panel.push(Box::from(LineBreak));
        panel.push(Box::from(LabeledText::new(&to_string!(transform.translation), "Position")));
        panel.push(Box::from(LabeledText::new(
          &format!("{0:.3}", timestep.dt().as_millis()),
          "Frame Time",
        )));
      } else {
        panel.lines[1] = Box::from(LabeledText::new(&to_string!(transform.translation), "Position"));
        // panel.lines[2] = Box::from(LabeledText::new(&format!("{0:.3}", rigid_body.beta()), "Beta"));
        panel.lines[2] = Box::from(LabeledText::new(&format!("{0:.3}", timestep.dt().as_millis()), "Frame Time"));
      }
    }
  }
}
