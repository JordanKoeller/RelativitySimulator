use cgmath::prelude::*;
use specs::prelude::*;

use engine::ecs::components::{Camera, EventReceiver, Player};
use engine::ecs::{MonoBehavior, SystemUtilities, WorldProxy};
use engine::events::{Event, EventChannel, EventPayload, KeyCode, StatelessEventChannel, WindowEvent};
use engine::gui::{ControlPanelBuilder, LabeledText, SystemDebugger};
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
pub struct PlayerController;

impl<'a> MonoBehavior<'a> for PlayerController {
    type SystemData = PlayerControllerSystemData<'a>;

    fn run(&mut self, api: SystemUtilities<'a>, mut s: Self::SystemData) {
        for (_p, camera, transform, events) in (&s.player, &mut s.camera, &mut s.transform, &s.event_receiver).join() {
            let init_rotation = transform.clone();
            s.event_channel.for_each(&events.0, |evt| match evt.code {
                Event::KeyDown(KeyCode::W) => transform.translation += init_rotation.front().normalize_to(0.04f64),
                Event::KeyDown(KeyCode::A) => transform.translation -= init_rotation.right().normalize_to(0.04f64),
                Event::KeyDown(KeyCode::S) => transform.translation -= init_rotation.front().normalize_to(0.04f64),
                Event::KeyDown(KeyCode::D) => transform.translation += init_rotation.right().normalize_to(0.04f64),
                Event::KeyDown(KeyCode::LeftShift) => transform.translation -= init_rotation.up().normalize_to(0.04f64),
                Event::KeyDown(KeyCode::Space) => transform.translation += init_rotation.up().normalize_to(0.04f64),
                Event::MouseMoved => {
                    if let Some(payload) = &evt.payload {
                        match payload {
                            EventPayload::MouseMove(vec) => transform.rotate(vec.x * 0.05, vec.y * 0.05),
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
            panel.set_str("Player Position", to_string!(transform.translation));
            panel.set_str("Player Facing", to_string!(transform.rotation.normalize()));
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
        let pos = Vec3F::new(4f64, 4f64, 2f64);
        let mut tc = TransformComponent::new(pos, Vec3F::new(1f64, 1f64, 1f64), QuatF::zero());
        tc.rotation = Vec3F::unit_y() * 90f64;
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
    }
}

impl PlayerController {
    fn refresh_camera(&self, t: &TransformComponent, cam: &mut Camera) {
        let location = cgmath::Point3::<f64>::new(t.translation.x, t.translation.y, t.translation.z);
        let pov = t.translation + t.front();
        let center = cgmath::Point3::<f64>::new(pov.x, pov.y, pov.z);
        let up = Vec3F::unit_y();
        let matrix = Mat4F::look_at(location, center, up);
        cam.set_matrix(matrix);
    }
}
