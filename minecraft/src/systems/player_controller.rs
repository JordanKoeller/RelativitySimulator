use cgmath::prelude::*;
use specs::prelude::*;
use engine::ecs::components::{Camera, EventReceiver, MeshComponent, Player};
use engine::ecs::SystemDelegate;
use engine::events::{Event, EventChannel, EventPayload, KeyCode, StatefulEventChannel, StatelessEventChannel, WindowEvent};
use engine::gui::*;
use engine::physics::{Gravity, RigidBody, TransformComponent};
use engine::utils::{random, Mat4F, QuatF, Timer, TimerLike, Timestep, Vec2F, Vec3F};
const IMPULSE: f32 = 0.2f32;

#[derive(SystemData)]
pub struct PlayerControllerSystemData<'a> {
    player: ReadStorage<'a, Player>,
    camera: WriteStorage<'a, Camera>,
    rigid_body: WriteStorage<'a, RigidBody>,
    transform: WriteStorage<'a, TransformComponent>,
    event_receiver: ReadStorage<'a, EventReceiver>,
    event_channel: Write<'a, StatelessEventChannel<WindowEvent>>,
    timestep: Read<'a, Timestep>,
}

#[derive(Default)]
pub struct PlayerController;

impl<'a> SystemDelegate<'a> for PlayerController {
    type SystemData = PlayerControllerSystemData<'a>;

    fn run(&mut self, mut s: Self::SystemData) {
        for (_p, mut camera, transform, events) in
            (&s.player, &mut s.camera, &mut s.transform, &s.event_receiver).join()
        {
            let init_rotation = transform.clone();
            s.event_channel.for_each(&events.0, |evt| match evt.code {
                Event::KeyDown(KeyCode::W) => transform.translation += init_rotation.front().normalize_to(0.04f32),
                Event::KeyDown(KeyCode::A) => transform.translation -= init_rotation.right().normalize_to(0.04f32),
                Event::KeyDown(KeyCode::S) => transform.translation -= init_rotation.front().normalize_to(0.04f32),
                Event::KeyDown(KeyCode::D) => transform.translation += init_rotation.right().normalize_to(0.04f32),
                Event::KeyDown(KeyCode::LeftShift) => transform.translation -= init_rotation.up().normalize_to(0.04f32),
                Event::KeyDown(KeyCode::Space) => transform.translation += init_rotation.up().normalize_to(0.04f32),
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
            self.refresh_camera(&transform, &mut camera)
        }
    }

    fn update_debugger(&mut self, s: &mut Self::SystemData, debugger: &mut DebugPanel) {
        for (_p, rigid_body, transform) in (&s.player, &mut s.rigid_body, &s.transform).join() {
            debugger.panel.lines[1] = Box::from(LabeledText::new("Position", &to_string!(transform.translation)));
            debugger.panel.lines[2] = Box::from(LabeledText::new("Velocity", &to_string!(rigid_body.velocity)));
            debugger.panel.lines[3] = Box::from(LabeledText::new("Facing", &to_string!(transform.front())));
        }
    }

    fn setup(&mut self, world: &mut World) {
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
        let mut tc = TransformComponent::new(pos, Vec3F::new(1f32, 1f32, 1f32), QuatF::zero());
        tc.rotation = Vec3F::unit_y() * 90f32;
        world.register::<Player>();
        world.register::<RigidBody>();
        world.register::<TransformComponent>();
        world.register::<EventReceiver>();
        world.register::<Camera>();
        world.register::<MeshComponent>();
        world
            .create_entity()
            .with(Player)
            .with(tc)
            .with(Camera::default())
            .with(RigidBody::new_stationary())
            .with(receiver)
            .build();
    }

    fn setup_debug_panel(&mut self, _world: &mut World) -> Option<DebugPanel> {
        let mut gui = DebugPanel::new("Player Controller");
        gui.panel.push(Box::from(LabeledText::new("Pressed Buttons", "")));
        gui.panel.push(Box::from(LabeledText::new("Position", "")));
        gui.panel.push(Box::from(LabeledText::new("Velocity", "")));
        gui.panel.push(Box::from(LabeledText::new("Facing", "")));
        Some(gui)
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
    }
}
