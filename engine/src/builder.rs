use specs::prelude::*;
use std::time::Duration;

use crate::ecs::components::{DrawableId, Material, MeshComponent};
use crate::ecs::systems::*;
use crate::ecs::SystemManager;
use crate::events::{Event, EventChannel, KeyCode, ReceiverID, StatelessEventChannel, WindowEvent};
use crate::game_loop::GameLoop;
use crate::gui::GuiRenderer;
use crate::physics::TransformComponent;
use crate::renderer::{Renderer, Shader, Window};
use crate::utils::{GetMutRef, MutRef, RunningState, Timestep, Vec2F};

struct RendererBuilder {
    pub shaders: Vec<Shader>,
    dims: Vec2F,
    receiver_id: ReceiverID,
}

impl RendererBuilder {
    pub fn empty() -> Self {
        RendererBuilder {
            shaders: Vec::default(),
            dims: Vec2F::new(0f32, 0f32),
            receiver_id: 0,
        }
    }

    pub fn set_dims(mut self, dims: Vec2F) -> Self {
        self.dims = dims;
        self
    }

    pub fn bind_window_events(mut self, channel: &mut StatelessEventChannel<WindowEvent>) -> Self {
        self.receiver_id = channel.register_with_subs(&[
            WindowEvent::new(Event::WindowResized),
            WindowEvent::new(Event::KeyPressed(KeyCode::Tab)),
            WindowEvent::new(Event::KeyPressed(KeyCode::Q)),
        ]);
        self
    }

    pub fn build(self) -> Renderer {
        let mut renderer = Renderer::new(self.dims, self.receiver_id);
        self.shaders
            .into_iter()
            .for_each(|shader| renderer.submit_shader(shader));
        renderer
    }
}

pub struct GameBuilder<'a, 'b> {
    render_builder: RendererBuilder,
    dispatcher_builder: DispatcherBuilder<'a, 'b>,
    world: World,
    window: Window,
}

impl<'a, 'b> GameBuilder<'a, 'b> {
    // COnstructs an empty GameBuilder instance, to be populated later using a builder pattern.
    pub fn new(window: Window) -> Self {
        let world = WorldBuilder::build();

        Self {
            render_builder: RendererBuilder::empty(),
            dispatcher_builder: DispatcherBuilder::new(),
            world,
            window,
        }
    }

    pub fn add_shader(mut self, filename: &str) -> Self {
        let fname_parts: Vec<&str> = filename.split("/").collect();
        let fname: &str = fname_parts[fname_parts.len() - 1].split(".").next().unwrap();
        self.render_builder.shaders.push(Shader::from_file(filename, fname));
        self
    }

    pub fn add_shader_id(mut self, filename: &str, shader_id: &str) -> Self {
        self.render_builder.shaders.push(Shader::from_file(filename, shader_id));
        self
    }

    pub fn insert_resource<R>(mut self, r: R) -> Self
    where
        R: Resource,
    {
        self.world.insert(r);
        self
    }

    pub fn register_component<C: Component>(mut self) -> Self
    where
        C::Storage: Default,
    {
        self.world.register::<C>();
        self
    }

    pub fn with_system<T>(mut self, system: T, name: &str, dep: &[&str]) -> Self
    where
        T: for<'c> System<'c> + Send + 'a,
    {
        self.dispatcher_builder.add(system, name, dep);
        self
    }

    pub fn with_local_system<T>(mut self, system: T) -> Self
    where
        T: for<'c> RunNow<'c> + 'b,
    {
        self.dispatcher_builder.add_thread_local(system);
        self
    }

    pub fn with_entity<F>(mut self, f: F) -> Self
    where
        F: FnOnce(EntityBuilder) -> Entity,
    {
        let entity_builder = self.world.create_entity();
        f(entity_builder);
        self
    }

    pub fn build(mut self) -> GameLoop<'a, 'b> {
        // Create a WindowEvent channel and set up control events.
        let mut window_channel = StatelessEventChannel::<WindowEvent>::default();
        let world_id = window_channel.register_with_subs(&[
            WindowEvent::new(Event::KeyPressed(KeyCode::Control)),
            WindowEvent::new(Event::KeyPressed(KeyCode::Esc)),
            WindowEvent::new(Event::KeyPressed(KeyCode::Alt)),
            WindowEvent::new(Event::KeyPressed(KeyCode::F)),
        ]);

        // Build the Renderer resource, bind it to the world
        let renderer = self
            .render_builder
            .set_dims(self.window.get_dims_f32())
            .bind_window_events(&mut window_channel)
            .build();
        let renderer = Self::add_default_shaders(renderer);
        self.world.insert(renderer);

        // Bind the remaining resources to the world
        let mut time = Timestep::default();
        time.click_frame(Duration::from_secs_f64(self.window.glfw_token.get_time()));
        time.click_frame(Duration::from_secs_f64(self.window.glfw_token.get_time() + 1e-8f64));
        self.world.insert(window_channel);
        self.world.insert(time);
        self.world.insert(RunningState::default());

        // Register some components

        // Set up dispatcher with rendering systems.
        let window_ref = GetMutRef(self.window);
        let dispatcher = self
            .dispatcher_builder
            .with_thread_local(StartFrameSystem {
                window: MutRef::clone(&window_ref),
                receiver_id: world_id,
            })
            .with_thread_local(EventProcessingSystem::default())
            .with_thread_local(SystemManager::new(RenderPipelineSystem::new(
                MutRef::clone(&window_ref),
                world_id,
            )))
            .with_thread_local(GuiRenderer {
                window: MutRef::clone(&window_ref),
            })
            .with_thread_local(EndFrameSystem {
                window: MutRef::clone(&window_ref),
            })
            .build();
        GameLoop::new(self.world, dispatcher, window_ref)
    }

    fn add_default_shaders(mut renderer: Renderer) -> Renderer {
        let shader = Shader::from_file("default_texture", "shaders/simple_textured.glsl");
        renderer.submit_shader(shader);
        let shader = Shader::from_file("instanced", "shaders/simple_instanced.glsl");
        renderer.submit_shader(shader);
        let shader = Shader::from_file_skybox("skybox", "shaders/skybox.glsl");
        renderer.submit_shader(shader);
        renderer
    }
}

struct WorldBuilder;

impl WorldBuilder {
    pub fn build() -> World {
        let mut world = World::new();

        // Register all my default component types
        world.register::<MeshComponent>();
        world.register::<Material>();
        world.register::<TransformComponent>();
        world.register::<DrawableId>();

        // Insert default resources
        // world.insert(window_channel);
        // world.insert(world_id);
        world
    }
}