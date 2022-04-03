use specs::prelude::*;
use crate::gui::*;
use crate::ecs::components::Player;
use crate::utils::Timestep;

use crate::physics::{RigidBody, TransformComponent};

pub struct DiagnosticsPanel;

impl<'a> System<'a> for DiagnosticsPanel {
    type SystemData = (
        WriteStorage<'a, Player>,
        ReadStorage<'a, TransformComponent>,
        ReadStorage<'a, RigidBody>,
        WriteStorage<'a, DebugPanel>,
        Read<'a, Timestep>,
    );

    fn run(&mut self, (mut s_player, s_transform, s_rigid, mut s_panel, timestep): Self::SystemData) {
        for (_player, transform, _rigid_body, panel) in (&mut s_player, &s_transform, &s_rigid, &mut s_panel).join() {
            if panel.panel.empty() {
                panel.panel.push(Box::from(LineBreak));
                panel.panel.push(Box::from(LabeledText::new(
                    &to_string!(transform.translation),
                    "Position",
                )));
                panel.panel.push(Box::from(LabeledText::new(
                    &format!("{0:.3}", timestep.dt().as_millis()),
                    "Frame Time",
                )));
            } else {
                panel.panel.lines[1] = Box::from(LabeledText::new(&to_string!(transform.translation), "Position"));
                panel.panel.lines[2] = Box::from(LabeledText::new(
                    &format!("{0:.3}", timestep.dt().as_millis()),
                    "Frame Time",
                ));
            }
        }
    }
}
