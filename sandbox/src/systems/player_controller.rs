use cgmath::prelude::*;
use specs::prelude::*;

use engine::ecs::components::{Camera, EventReceiver, Player};
use engine::ecs::{MonoBehavior, SystemUtilities, WorldProxy};
use engine::events::{Event, EventChannel, EventPayload, KeyCode, StatelessEventChannel, WindowEvent};
use engine::gui::{ControlPanelBuilder, InputFloat, LabeledText, SystemDebugger};
use engine::physics::{RigidBody, TransformComponent};
use engine::utils::{Mat4F, QuatF, Vec3F};

#[derive(SystemData)]
pub struct PlayerControllerSystemData<'a> {
    player: ReadStorage<'a, Player>,
    camera: WriteStorage<'a, Camera>,
    transform: WriteStorage<'a, TransformComponent>,
    event_receiver: ReadStorage<'a, EventReceiver>,
    event_channel: Write<'a, StatelessEventChannel<WindowEvent>>,
}

#[derive(Default)]
pub struct PlayerController {
    sensitivity_scalar: f32,
}

impl<'a> MonoBehavior<'a> for PlayerController {
    type SystemData = PlayerControllerSystemData<'a>;

    fn run(&mut self, api: SystemUtilities<'a>, mut s: Self::SystemData) {
        {
            let panel = self.get_write_panel(&api);
            self.sensitivity_scalar = panel.get_float("Mouse Sensitivity");
        }
        for (_p, camera, transform, events) in (&s.player, &mut s.camera, &mut s.transform, &s.event_receiver).join() {
            let init_rotation = transform.clone();
            s.event_channel.for_each(&events.0, |evt| match evt.code {
                Event::KeyDown(KeyCode::W) => transform.translation += init_rotation.front().normalize_to(0.04f32),
                Event::KeyDown(KeyCode::A) => transform.translation -= init_rotation.right().normalize_to(0.04f32),
                Event::KeyDown(KeyCode::S) => transform.translation -= init_rotation.front().normalize_to(0.04f32),
                Event::KeyDown(KeyCode::D) => transform.translation += init_rotation.right().normalize_to(0.04f32),
                Event::KeyDown(KeyCode::LeftShift) => {
                    transform.translation -= init_rotation.world_up().normalize_to(0.04f32)
                }
                Event::KeyDown(KeyCode::Space) => {
                    transform.translation += init_rotation.world_up().normalize_to(0.04f32)
                }
                Event::MouseMoved => {
                    if let Some(payload) = &evt.payload {
                        match payload {
                            EventPayload::MouseMove(vec) => {
                                // Rotate in frame of camera
                                let right = init_rotation.right();
                                let up = init_rotation.up();
                                let dx = cgmath::Rad(-vec.x * self.sensitivity_scalar);
                                let dy = cgmath::Rad(vec.y * self.sensitivity_scalar);

                                let delta = QuatF::from_axis_angle(up, dx) * QuatF::from_axis_angle(right, dy);

                                // Rotate around Euler angles
                                let euler_angles = cgmath::Euler::new(-dy, dx, cgmath::Rad(0f32));
                                let mut delta = QuatF::from(euler_angles);

                                // Rotate around world coordinates
                                let delta = QuatF::from_angle_y(dx) * QuatF::from_angle_x(-dy);
                                transform.push_rotation(&delta.normalize());
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
            self.refresh_camera(transform, camera);
            let mut panel = self.get_write_panel(&api);
            let quat = &transform.rotation;
            panel.set_str("Player Position", to_string!(transform.translation));
            panel.set_str("Player Facing", to_string!(transform.front()));
            panel.set_str(
                "Player Quaternion",
                format!("<{:.3}, {:.3}, {:.3}> {:.3}", quat.v.x, quat.v.y, quat.v.z, quat.s),
            );
        }
    }

    fn setup(&mut self, mut world: WorldProxy) {
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
        let pos = Vec3F::new(4f32, 4f32, 2f32);
        let tc = TransformComponent::new(pos, Vec3F::new(1f32, 1f32, 1f32), QuatF::one());
        world.register::<Player>();
        world.register::<RigidBody>();
        world.register::<TransformComponent>();
        world.register::<EventReceiver>();
        world.register::<Camera>();
        world
            .create_entity()
            .with(Player)
            .with(tc)
            .with(Camera::default())
            .with(RigidBody::new_stationary())
            .with(receiver)
            .build();
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

impl PlayerController {
    fn refresh_camera(&self, t: &TransformComponent, cam: &mut Camera) {
        let location = cgmath::Point3::<f32>::new(t.translation.x, t.translation.y, t.translation.z);
        let pov = t.translation + t.front();
        let center = cgmath::Point3::<f32>::new(pov.x, pov.y, pov.z);
        let up = Vec3F::unit_y();
        let matrix = Mat4F::look_at(location, center, up);
        cam.set_matrix(matrix);
        cam.set_position(t.translation);
    }
}
