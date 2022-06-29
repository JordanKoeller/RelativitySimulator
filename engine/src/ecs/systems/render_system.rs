use std::sync::RwLockWriteGuard;
use std::time::{Duration, Instant};

use cgmath::prelude::*;
use specs::prelude::SystemData;
use specs::prelude::*;

use crate::debug::DebugMetrics;
use crate::ecs::components::{Camera, Player};
use crate::ecs::{MonoBehavior, SystemUtilities, WorldProxy};
use crate::events::{
    Event, EventChannel, KeyCode, ReceiverID, StatelessEventChannel, WindowEvent, WindowEventDispatcher,
};
use crate::graphics::{
    AssetLibrary, Assets, MaterialComponent, MeshComponent, Uniform, VertexArray, VertexArrayBuilder,
};
use crate::gui::{widgets::*, ControlPanel, ControlPanelBuilder, SystemDebugger};
use crate::physics::TransformComponent;
use crate::platform::Window;
use crate::renderer::render_pipeline::*;
use crate::renderer::{DrawCall, RenderCommand, RenderQueue, Renderer};
use crate::utils::{CompoundStopwatch, Counter, Mat4F, MutRef, RunningEnum, RunningState, StopwatchLike};

pub struct StartFrameSystem {
    pub window: MutRef<Window>,
    pub receiver_id: ReceiverID,
}

impl<'a> MonoBehavior<'a> for StartFrameSystem {
    type SystemData = (
        Write<'a, Renderer>,
        Write<'a, StatelessEventChannel<WindowEvent>>,
        Write<'a, WindowEventDispatcher>,
        Write<'a, RunningState>,
        ReadStorage<'a, Camera>,
    );

    fn run(
        &mut self,
        api: SystemUtilities<'a>,
        (mut renderer, mut events, mut window_events, mut running, camera_storage): Self::SystemData,
    ) {
        let mut window = self.window.borrow_mut();
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
        renderer.process_events(&mut events);
        renderer.init_frame(&mut window);
        for camera in (&camera_storage).join() {
            renderer.start_scene(&camera);
        }
        let panel = self.get_panel(&api);
        renderer.submit_env_uniform("ambient_strength", Uniform::Float(panel.get_float("ambient_strength")));
        renderer.submit_env_uniform("diffuse_strength", Uniform::Float(panel.get_float("diffuse_strength")));
        renderer.submit_env_uniform(
            "specular_strength",
            Uniform::Float(panel.get_float("specular_strength")),
        );
        renderer.submit_env_uniform("specular_power", Uniform::Float(panel.get_float("specular_power")));
        renderer.submit_env_uniform("gamma", Uniform::Float(panel.get_float("gamma")));
    }

    fn setup(&mut self, world: WorldProxy) {
        self.register_debugger(&world);
    }
}

impl<'a> SystemDebugger<'a> for StartFrameSystem {
    fn create_panel(&self) -> ControlPanelBuilder {
        ControlPanelBuilder::default()
            .with_title("Render Parameters")
            .push_line("ambient_strength", InputFloat::new("Ambient Strength", 0.2))
            .push_line("diffuse_strength", InputFloat::new("Diffuse Strength", 0.4))
            .push_line("specular_strength", InputFloat::new("Specular Strength", 0.8))
            .push_line(
                "specular_power",
                InputFloat::new_with_limits("Specular Power", 32f32, 4f32, 64f32),
            )
            .push_line("gamma", InputFloat::new_with_limits("Gamma", 1.8f32, 0.5f32, 3.0f32))
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

pub struct RegisterDrawableSystem {
    receiver_id: Option<ReaderId<ComponentEvent>>,
}

impl Default for RegisterDrawableSystem {
    fn default() -> Self {
        Self { receiver_id: None }
    }
}

impl<'a> System<'a> for RegisterDrawableSystem {
    type SystemData = (Write<'a, AssetLibrary>, WriteStorage<'a, MeshComponent>);

    fn run(&mut self, (mut assets, mut mesh_storage): Self::SystemData) {
        assets.flush_all();
        let mut modified = specs::hibitset::BitSet::new();
        for evt in mesh_storage.channel().read(self.receiver_id.as_mut().unwrap()) {
            match evt {
                ComponentEvent::Modified(id) => {
                    modified.add(*id);
                }
                _ => {}
            }
        }
        for (mesh, _) in (&mut mesh_storage, &modified).join() {
            self.update_buffer_for(mesh, &mut assets);
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<MeshComponent>();
        self.receiver_id = Some(world.system_data::<WriteStorage<'_, MeshComponent>>().register_reader());
    }
}

impl RegisterDrawableSystem {
    fn update_buffer_for<'a>(&self, mesh: &mut MeshComponent, assets: &mut Write<'a, AssetLibrary>) {
        let asset = <AssetLibrary as Assets<VertexArrayBuilder>>::get_asset_mut(assets, &mut mesh.vertex_array_id);
        if let Some(mut vao_mut) = asset {
            vao_mut.update_dynamic_buffers()
        }
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
        if render_queue.len() == 0 {
            for (entity, drawable) in (entities, drawables).join() {
                let cmd = DrawCall {
                    mesh_component: drawable.clone(),
                    entity,
                    cmd: RenderCommand::Draw,
                };
                render_queue.push(cmd);
            }
        }
    }

    fn init_frame(&self, renderer: &mut Renderer) {
        renderer.init_frame(&mut self.window.borrow_mut());
    }

    fn render<'a>(&mut self, system_data: &mut RenderSystemData<'a>) {
        system_data.renderer.render_scene(
            system_data.render_queue.iter(),
            &system_data.material_s,
            &system_data.transform_s,
            &mut system_data.assets,
            &system_data.debug_metrics,
        );
    }
}
