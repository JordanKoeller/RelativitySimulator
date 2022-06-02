use cgmath::prelude::*;
use cgmath::{Deg, Rad};
use specs::prelude::*;

use ecs::components::*;
use events::*;
use gui::{DebugPanel, GuiInputPanel, LabeledText, LineBreak};
use utils::*;

use physics::{RigidBody, TransformComponent};

pub struct CameraDebugger;

impl<'a> System<'a> for CameraDebugger {
    type SystemData = (
        WriteStorage<'a, TransformComponent>,
        WriteStorage<'a, DebugPanel>,
        WriteStorage<'a, Camera>,
        ReadStorage<'a, EventReceiver>,
        Write<'a, StatelessEventChannel<WindowEvent>>,
        Read<'a, Timestep>,
    );

    fn run(&mut self, (mut s_transform, mut s_panel, mut s_cam, s_evt_id, events_channel, dt): Self::SystemData) {
        for (transform, panel, mut camera, event_id) in (&mut s_transform, &mut s_panel, &mut s_cam, &s_evt_id).join() {
            // First I process any events
            let init_rotation = transform.clone();
            events_channel.for_each(&event_id.0, |evt| {
                match evt.code {
                    Event::KeyDown(KeyCode::W) => transform.translation += init_rotation.front().normalize_to(0.04f32),
                    Event::KeyDown(KeyCode::A) => transform.translation -= init_rotation.right().normalize_to(0.04f32),
                    Event::KeyDown(KeyCode::S) => transform.translation -= init_rotation.front().normalize_to(0.04f32),
                    Event::KeyDown(KeyCode::D) => transform.translation += init_rotation.right().normalize_to(0.04f32),
                    Event::KeyDown(KeyCode::Q) => transform.translation -= init_rotation.up().normalize_to(0.04f32),
                    Event::KeyDown(KeyCode::E) => transform.translation += init_rotation.up().normalize_to(0.04f32),
                    Event::MouseMoved => {
                        if let Some(payload) = &evt.payload {
                            match payload {
                                EventPayload::MouseMove(vec) => {
                                    // transform.push_rotation(Vec3F::unit_x(), vec.x * 0.05);
                                    // transform.push_rotation(Vec3F::unit_y(), vec.y * 0.05);
                                    transform.rotate(vec.x * 0.05, vec.y * 0.05)
                                }
                                _ => panic!("Received a payload of {:?} on MouseMoved event!", payload),
                            }
                        }
                    }
                    _ => panic!("Encountered unexpected key code in camera debugger"),
                }
            });
            self.refresh_camera(&transform, &mut camera);

            // Then I work on updating the panel
            if panel.panel.empty() {
                panel.panel.push(Box::from(LineBreak));
                panel.panel.push(Box::from(LabeledText::new(
                    &to_string!(transform.translation),
                    "Position",
                )));
                panel
                    .panel
                    .push(Box::from(LabeledText::new(&to_string!(transform.front()), "Forward")));
                panel.panel.push(Box::from(LabeledText::new(
                    &format!("{0:.3}", dt.dt().as_millis()),
                    "Frame Time",
                )));
                panel.panel.push(Box::from(LabeledText::new(
                    &format!("{0:.3}", dt.render_dt().as_millis()),
                    "Render Time",
                )));
            } else {
                panel.panel.lines[1] = Box::from(LabeledText::new(&to_string!(transform.translation), "Position"));
                panel.panel.lines[2] = Box::from(LabeledText::new(&to_string!(transform.front()), "Forward"));
                panel.panel.lines[3] =
                    Box::from(LabeledText::new(&format!("{0:.3}", dt.dt().as_millis()), "Frame Time"));
                panel.panel.lines[4] = Box::from(LabeledText::new(
                    &format!("{0:.3}", dt.render_dt().as_millis()),
                    "Render Time",
                ));
            }
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
                WindowEvent::new(Event::KeyDown(KeyCode::Q)),
                WindowEvent::new(Event::KeyDown(KeyCode::E)),
                // WindowEvent::new(Event::MouseMoved),
            ]))
        };
        world.register::<DebugPanel>();
        world.register::<Camera>();
        let mut tc = TransformComponent::new(
            Vec3F::unit_z() * -20f32,
            Vec3F::new(1f32, 1f32, 1f32),
            QuatF::from_angle_x(Rad::from(Deg(90f32))),
        );
        tc.rotation = Vec3F::unit_y() * 90f32;
        world
            .create_entity()
            .with(Camera::default())
            .with(tc)
            .with(DebugPanel::new("Camera Info"))
            .with(receiver)
            .build();
    }
}

impl CameraDebugger {
    fn refresh_camera(&self, t: &TransformComponent, cam: &mut Camera) {
        let location = cgmath::Point3::<f32>::new(t.translation.x, t.translation.y, t.translation.z);
        let pov = t.translation + t.front();
        let center = cgmath::Point3::<f32>::new(pov.x, pov.y, pov.z);
        let up = Vec3F::unit_y();
        let matrix = Mat4F::look_at(location, center, up);
        cam.set_matrix(matrix);
        cam.set_position(location);
    }
}
