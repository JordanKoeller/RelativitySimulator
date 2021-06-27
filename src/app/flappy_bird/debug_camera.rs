use cgmath::prelude::*;
use specs::prelude::*;

use utils::*;
use ecs::components::*;
use ecs::components::Rotation;
use gui::{GuiInputPanel, LabeledText, LineBreak};
use events::*;

pub struct CameraDebugger;

impl<'a> System<'a> for CameraDebugger {
  type SystemData = (
    WriteStorage<'a, Position>,
    WriteStorage<'a, Rotation>,
    WriteStorage<'a, GuiInputPanel>,
    WriteStorage<'a, Camera>,
    ReadStorage<'a, EventReceiver>,
    Write<'a, StatelessEventChannel<WindowEvent>>,
    Read<'a, Timestep>
  );

  fn run(&mut self, (mut s_pos, mut s_rot, mut s_panel, mut s_cam, s_evt_id, events_channel, dt): Self::SystemData) {
    for (position, rotation, panel, mut camera, event_id) in (&mut s_pos, &mut s_rot, &mut s_panel, &mut s_cam, &s_evt_id).join() {
      // First I process any events
      let init_rotation = rotation.clone();
      events_channel.for_each(&event_id.0, |evt| {
        match evt.code {
          Event::KeyDown(KeyCode::W) => position.0 += init_rotation.front().normalize_to(0.04f32),
          Event::KeyDown(KeyCode::A) => position.0 -= init_rotation.right().normalize_to(0.04f32),
          Event::KeyDown(KeyCode::S) => position.0 -= init_rotation.front().normalize_to(0.04f32),
          Event::KeyDown(KeyCode::D) => position.0 += init_rotation.right().normalize_to(0.04f32),
          Event::KeyDown(KeyCode::Q) => position.0 -= init_rotation.up().normalize_to   (0.04f32),
          Event::KeyDown(KeyCode::E) => position.0 += init_rotation.up().normalize_to   (0.04f32),
          Event::MouseMoved => {
            if let Some(payload) = &evt.payload {
              match payload {
                EventPayload::MouseMove(vec) => {
                  rotation.rotate(vec.x * 0.05, vec.y * 0.05)
                },
                _ => panic!(format!("Received a payload of {:?} on MouseMoved event!", payload))
              }
            }
          },
          _ => panic!("Encountered unexpected key code in camera debugger")
        }
      });
      self.refresh_camera(&position, &rotation, &mut camera);

      // Then I work on updating the panel
      if panel.empty() {
        panel.push(Box::from(LineBreak));
        panel.push(Box::from(LabeledText::new(&to_string!(position.0), "Position")));
        panel.push(Box::from(LabeledText::new(&to_string!(rotation.front()), "Forward")));
        panel.push(Box::from(LabeledText::new(
          &format!("{0:.3}", dt.0 * 1000f32),
          "Frame Time",
        )));
      } else {
        panel.lines[1] = Box::from(LabeledText::new(&to_string!(position.0), "Position"));
        panel.lines[2] = Box::from(LabeledText::new(&to_string!(rotation.front()), "Forward"));
        panel.lines[3] = Box::from(LabeledText::new(&format!("{0:.3}", dt.0 * 1000f32), "Frame Time"));
      }
    }
  }

  fn setup(&mut self, world: &mut World) {
    let receiver = {
      let mut listener = world.write_resource::<StatelessEventChannel<WindowEvent>>();
      EventReceiver(listener.register_with_subs(&[
        // WindowEvent::new(Event::KeyDown(KeyCode::W)),
        // WindowEvent::new(Event::KeyDown(KeyCode::A)),
        // WindowEvent::new(Event::KeyDown(KeyCode::S)),
        // WindowEvent::new(Event::KeyDown(KeyCode::D)),
        // WindowEvent::new(Event::KeyDown(KeyCode::Q)),
        // WindowEvent::new(Event::KeyDown(KeyCode::E)),
        // WindowEvent::new(Event::MouseMoved),
        ]))
    };
    world.register::<GuiInputPanel>();
    world.register::<Camera>();
    world.create_entity()
      .with(Camera::default())
      .with(Rotation(Vec2F::new(0f32, 90f32)))
      .with(GuiInputPanel::new("Camera Info"))
      .with(Position(Vec3F::new(0f32, 0f32, -20f32)))
      .with(receiver)
      .build();
  }
}

impl CameraDebugger {

  fn refresh_camera(&self, pos: &Position, rot: &Rotation, cam: &mut Camera) {
    let location = cgmath::Point3::<f32>::new(pos.0.x, pos.0.y, pos.0.z);
    let pov = pos.0 + rot.front();
    let center = cgmath::Point3::<f32>::new(pov.x, pov.y, pov.z);
    let up = Vec3F::unit_y();
    let matrix = Mat4F::look_at(location, center, up);
    cam.set_matrix(matrix);
  }
}