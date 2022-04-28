use std::time::{Duration, Instant};

use cgmath::prelude::*;
use specs::prelude::SystemData;
use specs::prelude::*;


use crate::ecs::components::{Camera, Player};
use crate::events::{
    Event, EventChannel, KeyCode, ReceiverID, StatelessEventChannel, WindowEvent, WindowEventDispatcher,
};
use crate::gui::*;
use crate::renderer::render_pipeline::*;
use crate::renderer::{DrawCall, RenderCommand, RenderQueue, Renderer,};
use crate::graphics::{AssetLibrary, MeshComponent, MaterialComponent};
use crate::platform::Window;
use crate::utils::{Mat4F, MutRef, RunningEnum, RunningState, Timestep};

use crate::physics::TransformComponent;

pub struct StartFrameSystem {
    pub window: MutRef<Window>,
    pub receiver_id: ReceiverID,
}

impl<'a> System<'a> for StartFrameSystem {
    type SystemData = (
        Write<'a, Renderer>,
        Write<'a, StatelessEventChannel<WindowEvent>>,
        Write<'a, WindowEventDispatcher>,
        Write<'a, Timestep>,
        Write<'a, RunningState>,
        ReadStorage<'a, Camera>,
    );

    fn run(
        &mut self,
        (mut renderer, mut events, mut window_events, mut timestep, mut running, camera_storage): Self::SystemData,
    ) {
        let mut window = self.window.borrow_mut();
        timestep.click_frame(Duration::from_secs_f64(window.glfw_token.get_time()));
        window.poll_events();
        window_events.process_events(&mut events, &mut window);
        events.for_each(&self.receiver_id, |window_evt| match window_evt.code {
            Event::KeyPressed(KeyCode::Control) => {
                window.toggle_cursor();
            }
            Event::KeyPressed(KeyCode::Esc) => {
                running.state = RunningEnum::Stopped;
            }
            Event::KeyPressed(KeyCode::Alt) => {
                match running.state {
                    RunningEnum::Running => running.state = RunningEnum::StepFrameWait,
                    RunningEnum::StepFrameWait => running.state = RunningEnum::Running,
                    _ => {}
                }
                println!("Updated running state to {:?}", running.state);
            }
            Event::KeyPressed(KeyCode::F) => match running.state {
                RunningEnum::StepFrameWait => {
                    running.state = RunningEnum::StepFrame;
                    println!("Stepping!");
                }
                _ => {}
            },
            _ => {}
        });
        renderer.init_frame(&mut window);
        for camera in (&camera_storage).join() {
            renderer.start_scene(&camera, &timestep);
        }
    }
}

pub struct EndFrameSystem {
    pub window: MutRef<Window>,
}

impl<'a> System<'a> for EndFrameSystem {
    type SystemData = Write<'a, Renderer>;

    fn run(&mut self, mut renderer: Self::SystemData) {
        let mut window = self.window.borrow_mut();
        renderer.end_frame(&mut window);
    }
}

pub struct RegisterDrawableSystem;

impl<'a> System<'a> for RegisterDrawableSystem {
    type SystemData = Write<'a, AssetLibrary>;

    fn run(&mut self, mut assets: Self::SystemData) {
        assets.flush_all();
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<MeshComponent>();
    }
}

#[derive(SystemData)]
pub struct RenderSystemData<'a> {
    entities: Entities<'a>,
    drawable_s: ReadStorage<'a, MeshComponent>,
    transform_s: ReadStorage<'a, TransformComponent>,
    material_s: ReadStorage<'a, MaterialComponent>,
    renderer: Write<'a, Renderer>,
    timestep: Write<'a, Timestep>,
    render_queue: Write<'a, RenderQueue>,
    assets: Write<'a, AssetLibrary>,
}

pub struct RenderPipelineSystem {
    window: MutRef<Window>,
    event_receiver_id: ReceiverID,
    draw_call_count: u32,
    render_time: Duration,
}

impl RenderPipelineSystem {
    pub fn new(window: MutRef<Window>, id: ReceiverID) -> Self {
        Self {
            window,
            event_receiver_id: id,
            draw_call_count: 0u32,
            render_time: Duration::new(0u64, 0u32),
        }
    }
}

impl<'a> System<'a> for RenderPipelineSystem {
    type SystemData = RenderSystemData<'a>;

    fn run(&mut self, mut system_data: Self::SystemData) {
        self.prepare_queue(
            &mut system_data.render_queue,
            &system_data.entities,
            &system_data.drawable_s,
            &system_data.assets,
        );
        // self.init_frame(&mut system_data.renderer);
        let start_time = self.window.borrow().glfw_token.get_time();
        let draw_call_count = self.render(&mut system_data);
        let end_time = self.window.borrow().glfw_token.get_time();
        self.render_time = Duration::from_secs_f64(end_time - start_time);
        self.draw_call_count = draw_call_count;
    }

    // fn update_debugger(&mut self, _data: &mut Self::SystemData, gui: &mut DebugPanel) {
    //     gui.panel.lines[1] = Box::from(LabeledText::new("Draw Calls", &self.draw_call_count.to_string()));
    //     gui.panel.lines[2] = Box::from(LabeledText::new(
    //         "GPU Render Time",
    //         &format!("{0:.3}", self.render_time.as_secs_f32() * 1000f32),
    //     ));
    // }

    // fn setup_debug_panel(&mut self, _: &mut World) -> Option<DebugPanel> {
    //     let mut ui = DebugPanel::new("Renderer Debugger");
    //     ui.panel.push(Box::from(LineBreak));
    //     ui.panel.push(Box::from(LabeledText::new("Draw Calls", "")));
    //     ui.panel.push(Box::from(LabeledText::new("GPU Render Time", "")));
    //     Some(ui)
    // }
}

impl RenderPipelineSystem {
    fn prepare_queue<'a>(
        &self,
        render_queue: &mut RenderQueue,
        entities: &Entities<'a>,
        drawables: &ReadStorage<'a, MeshComponent>,
        assets: &Write<'a, AssetLibrary>,
    ) {
        for (entity, drawable) in (entities, drawables).join() {
            let cmd = DrawCall {
                mesh_component: drawable.clone(),
                entity,
                cmd: RenderCommand::Draw,
            };
            render_queue.push(cmd);
        }
    }

    fn init_frame(&self, renderer: &mut Renderer) {
        renderer.init_frame(&mut self.window.borrow_mut());
    }

    fn render<'a>(&mut self, system_data: &mut <Self as System<'a>>::SystemData) -> u32 {
        system_data.renderer.render_scene(
            system_data.render_queue.consume(),
            &system_data.material_s,
            &system_data.transform_s,
            &mut system_data.assets,
        )
    }
}
