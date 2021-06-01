use ecs::components::{Kinetics, Player, Position, Rotation};
use gui::*;
use renderer::Camera;
use specs::prelude::*;
use utils::Timestep;

pub struct DiagnosticsPanel;

impl<'a> System<'a> for DiagnosticsPanel {
  type SystemData = (
    WriteStorage<'a, Player>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Kinetics>,
    ReadStorage<'a, Rotation>,
    WriteStorage<'a, GuiInputPanel>,
    Read<'a, Timestep>,
  );

  fn run(&mut self, (mut s_player, s_position, s_kinetics, s_rotation, mut s_panel, timestep): Self::SystemData) {
    for (player, position, kinetics, rotation, panel) in
      (&mut s_player, &s_position, &s_kinetics, &s_rotation, &mut s_panel).join()
    {
      let cam = Camera::new(&position.0, &kinetics.velocity, &rotation);
      if panel.empty() {
        panel.push(Box::from(LineBreak));
        panel.push(Box::from(LabeledText::new(&to_string!(cam.position), "Position")));
        panel.push(Box::from(LabeledText::new(&format!("{0:.3}", cam.beta()), "Beta")));
        panel.push(Box::from(LabeledText::new(
          &format!("{0:.3}", timestep.0 * 1000f32),
          "Frame Time",
        )));
      } else {
        panel.lines[1] = Box::from(LabeledText::new(&to_string!(cam.position), "Position"));
        panel.lines[2] = Box::from(LabeledText::new(&format!("{0:.3}", cam.beta()), "Beta"));
        panel.lines[3] = Box::from(LabeledText::new(&format!("{0:.3}", timestep.0 * 1000f32), "Frame Time"));
      }
    }
  }
}
