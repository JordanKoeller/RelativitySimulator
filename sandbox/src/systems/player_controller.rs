use cgmath::prelude::*;
use specs::prelude::*;

use engine::ecs::components::{Camera, EventReceiver, Player};
use engine::ecs::{MonoBehavior, SystemUtilities, WorldProxy};
use engine::events::{Event, EventChannel, EventPayload, KeyCode, StatelessEventChannel, WindowEvent};
use engine::gui::{widgets::*, ControlPanelBuilder, SystemDebugger};
use engine::utils::Vec3F;

#[derive(SystemData)]
pub struct PlayerControllerSystemData<'a> {
    player: ReadStorage<'a, Player>,
    camera: WriteStorage<'a, Camera>,
    event_receiver: ReadStorage<'a, EventReceiver>,
    event_channel: Write<'a, StatelessEventChannel<WindowEvent>>,
}

pub struct PlayerController {
    sensitivity_scalar: f64,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            sensitivity_scalar: 0.001f64,
        }
    }
}

impl<'a> MonoBehavior<'a> for PlayerController {
    type SystemData = PlayerControllerSystemData<'a>;

    fn run(&mut self, api: SystemUtilities<'a>, mut s: Self::SystemData) {
        {
            let panel = self.get_write_panel(&api);
            self.sensitivity_scalar = panel.get_float("Mouse Sensitivity");
        }
        for (_p, camera, events) in (&s.player, &mut s.camera, &s.event_receiver).join() {
            let mut delta = Vec3F::zero();
            s.event_channel.for_each(&events.0, |evt| match evt.code {
                Event::KeyDown(KeyCode::W) => delta += camera.front().normalize_to(0.04f64),
                Event::KeyDown(KeyCode::A) => delta -= camera.right().normalize_to(0.04f64),
                Event::KeyDown(KeyCode::S) => delta -= camera.front().normalize_to(0.04f64),
                Event::KeyDown(KeyCode::D) => delta += camera.right().normalize_to(0.04f64),
                Event::KeyDown(KeyCode::LeftShift) => delta -= Vec3F::unit_y().normalize_to(0.04f64),
                Event::KeyDown(KeyCode::Space) => delta += Vec3F::unit_y().normalize_to(0.04f64),
                Event::MouseMoved => {
                    if let Some(payload) = &evt.payload {
                        match payload {
                            EventPayload::MouseMove(vec) => {
                                let dx = -cgmath::Rad(-vec.x * self.sensitivity_scalar);
                                let dy = cgmath::Rad(vec.y * self.sensitivity_scalar);
                                let euler_angles = cgmath::Euler::new(dy, dx, cgmath::Rad(0f64));
                                camera.push_rotation(euler_angles);
                            }
                            _ => panic!("Received a payload of {:?} on MouseMoved event!", payload),
                        }
                    }
                }
                _ => panic!(
                    "Received an event that the player controller does not listen for! {:?}",
                    evt
                ),
            });
            camera.push_translation(delta);
            let mut panel = self.get_write_panel(&api);
            panel.set_str("Player Position", to_string!(camera.position()));
            panel.set_str("Player Facing", to_string!(camera.front()));
        }
    }

    fn setup(&mut self, mut world: WorldProxy) {
        Self::SystemData::setup(&mut world);
        self.register_debugger(&world);
        let receiver = {
            let mut listener = world.write_resource::<StatelessEventChannel<WindowEvent>>();
            EventReceiver(listener.register_with_subs(&[
                WindowEvent::new(Event::KeyDown(KeyCode::W)),
                WindowEvent::new(Event::KeyDown(KeyCode::A)),
                WindowEvent::new(Event::KeyDown(KeyCode::S)),
                WindowEvent::new(Event::KeyDown(KeyCode::D)),
                WindowEvent::new(Event::KeyDown(KeyCode::LeftShift)),
                WindowEvent::new(Event::KeyDown(KeyCode::Space)),
                WindowEvent::new(Event::MouseMoved),
            ]))
        };
        let camera = Camera::new(Vec3F::new(4f64, 4f64, 2f64), Vec3F::new(0f64, 0f64, 1f64));
        world.create_entity().with(Player).with(camera).with(receiver).build();
    }
}

impl<'a> SystemDebugger<'a> for PlayerController {
    fn create_panel(&self) -> ControlPanelBuilder {
        ControlPanelBuilder::default()
            .with_title("Player Controller")
            .push_line(
                "Player Position",
                LabeledText::new("<0.0, 0.0, 0.0>", "Player Position"),
            )
            .push_line("Player Facing", LabeledText::new("<0.0, 0.0, 0.0>", "Player Facing"))
            .push_line(
                "Player Quaternion",
                LabeledText::new("<0.0, 0.0, 0.0> 0.0", "Player Quaternion"),
            )
            .push_line(
                "Mouse Sensitivity",
                InputFloat::new_with_limits("Mouse Sensitivity", 0.001, 0.001, 0.01),
            )
    }
}
