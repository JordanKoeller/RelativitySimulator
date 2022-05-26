use std::time::{Duration, Instant};

use cgmath::prelude::*;
use specs::prelude::SystemData;
use specs::prelude::*;

use crate::debug::DebugMetrics;
use crate::ecs::components::{Camera, Player};
use crate::events::{
    Event, EventChannel, KeyCode, ReceiverID, StatelessEventChannel, WindowEvent, WindowEventDispatcher,
};
use crate::graphics::{AssetLibrary, MaterialComponent, MeshComponent};
use crate::gui::*;
use crate::physics::TransformComponent;
use crate::platform::Window;
use crate::renderer::render_pipeline::*;
use crate::renderer::{DrawCall, RenderCommand, RenderQueue, Renderer};
use crate::utils::{CompoundStopwatch, Counter, Mat4F, MutRef, RunningEnum, RunningState, StopwatchLike, Timestep};

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
        Write<'a, DebugMetrics>,
    );

    fn run(
        &mut self,
        (mut renderer, mut events, mut window_events, mut timestep, mut running, camera_storage, mut debugger): Self::SystemData,
    ) {
        let mut window = self.window.borrow_mut();
        timestep.click_frame(Duration::from_secs_f64(window.glfw_token.get_time()));
        window.poll_events();
        debugger.frame_time.start();
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
    type SystemData = (Write<'a, Renderer>, Write<'a, DebugMetrics>);

    fn run(&mut self, (mut renderer, mut debugger): Self::SystemData) {
        let mut window = self.window.borrow_mut();
        renderer.end_frame(&mut window);
        debugger.frame_time.stop();
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
    render_queue: Write<'a, RenderQueue>,
    assets: Write<'a, AssetLibrary>,
    debug_metrics: Write<'a, DebugMetrics>,
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
        );
        // self.init_frame(&mut system_data.renderer);
        system_data.debug_metrics.render_time.start();
        self.render(&mut system_data);
        system_data.debug_metrics.render_time.stop();
    }
}

impl RenderPipelineSystem {
    fn prepare_queue<'a>(
        &self,
        render_queue: &mut RenderQueue,
        entities: &Entities<'a>,
        drawables: &ReadStorage<'a, MeshComponent>,
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

    fn render<'a>(&mut self, system_data: &mut <Self as System<'a>>::SystemData) {
        system_data.renderer.render_scene(
            system_data.render_queue.consume(),
            &system_data.material_s,
            &system_data.transform_s,
            &mut system_data.assets,
            &system_data.debug_metrics,
        );
    }
}
