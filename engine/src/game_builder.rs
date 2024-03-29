use specs::prelude::*;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Duration;

use crate::debug::DebugMetricsSystem;
use crate::ecs::{systems::*, GuidMap, GuidRegistrySystem, Guid, EntityTree};
use crate::ecs::{EntityManager, PrefabBuilder, Sys, SystemUtilities, WorldProxy};
use crate::events::{Event, EventChannel, KeyCode, ReceiverId, StatelessEventChannel, WindowEvent};
use crate::game_loop::GameLoop;
use crate::graphics::{AssetLibrary, Assets, ShaderBuilder, ShaderDepthFunction};
use crate::graphics::{MaterialComponent, MeshComponent};
use crate::gui::{ControlPanel, ControlPanels, GuiRenderer};
use crate::physics::TransformComponent;
use crate::platform::Window;
use crate::renderer::Renderer;
use crate::utils::{GetMutRef, MutRef, RunningState, Timestep, Vec2F};

struct RendererBuilder {
  dims: Vec2F,
  receiver_id: ReceiverId,
}

impl RendererBuilder {
  pub fn empty() -> Self {
    RendererBuilder {
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
      WindowEvent::new(Event::KeyPressed(KeyCode::One)),
    ]);
    self
  }

  pub fn build(self) -> Renderer {
    Renderer::new(self.dims, self.receiver_id)
  }
}

pub enum GameLoopPhase {}

pub struct GameBuilder<'a, 'b> {
  render_builder: RendererBuilder,
  shaders: Vec<ShaderBuilder>,
  dispatcher_builder: DispatcherBuilder<'a, 'b>,
  world: World,
  window: Window,
}

impl<'a, 'b> GameBuilder<'a, 'b> {
  // COnstructs an empty GameBuilder instance, to be populated later using a builder pattern.
  pub fn new(window: Window) -> Self {
    let world = WorldBuilder::build();

    let mut ret = Self {
      render_builder: RendererBuilder::empty(),
      shaders: Vec::new(),
      dispatcher_builder: DispatcherBuilder::new(),
      world,
      window,
    };
    ret.add_default_shaders();
    ret
  }

  pub fn add_shader(mut self, filename: &str) -> Self {
    // let fname_parts: Vec<&str> = filename.split("/").collect();
    // let fname: &str = fname_parts[fname_parts.len() - 1].split(".").next().unwrap();
    let builder = ShaderBuilder::default().with_source_file(filename);
    self.shaders.push(builder);
    // self.shaders.push(Shader::from_file(filename, fname));
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

  pub fn with_prefab<B>(self, builder: &mut B, state: B::PrefabState) -> Self
  where
    B: PrefabBuilder,
  {
    {
      let api = self.world.system_data::<SystemUtilities>();
      builder.build(&api, state);
    }
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
    self.world.insert(renderer);

    // Bind the remaining resources to the world
    self.world.insert(window_channel);
    self.world.insert(Timestep::default());
    self.world.insert(RunningState::default());
    self.world.insert(ControlPanels::default());
    self.world.insert(GuidMap::default());
    // self.world.insert(Actor::new());

    // Register some components

    // Set up dispatcher with rendering systems.
    let window_ref = GetMutRef(self.window);

    let start_system = Sys::new(StartFrameSystem {
      window: MutRef::clone(&window_ref),
      receiver_id: world_id,
    });

    let gui_renderer = GuiRenderer {
      window: MutRef::clone(&window_ref),
    };

    let end_system = EndFrameSystem {
      window: MutRef::clone(&window_ref),
    };

    let dispatcher = self
      .dispatcher_builder
      .with(Sys::<DebugMetricsSystem>::default(), "debug", &[])
      .with(GuidRegistrySystem::default(), "guid_registry", &[])
      .with_thread_local(start_system)
      .with_thread_local(RegisterDrawableSystem::default())
      .with_thread_local(RenderPipelineSystem::new(MutRef::clone(&window_ref), world_id))
      .with_thread_local(gui_renderer)
      .with_thread_local(end_system)
      .build();
    GameLoop::new(self.world, dispatcher, window_ref)
  }

  fn add_default_shaders(&mut self) {
    let world = WorldProxy::new(&mut self.world);
    let utils = world.utilities();
    let assets = utils.assets();
    assets.get_or_create("default_texture", || {
      ShaderBuilder::default().with_source_file("shaders/simple_textured.glsl")
    });
    assets.get_or_create("debug_normals", || {
      ShaderBuilder::default().with_source_file("shaders/debug/normals.glsl")
    });
    assets.get_or_create("instanced", || {
      ShaderBuilder::default().with_source_file("shaders/simple_instanced.glsl")
    });
    assets.get_or_create("skybox", || {
      ShaderBuilder::default()
        .with_depth_function(ShaderDepthFunction::LEQUAL)
        .with_source_file("shaders/skybox.glsl")
    });
  }
}

struct WorldBuilder;

impl WorldBuilder {
  pub fn build() -> World {
    let mut world = World::new();

    // Register all my default component types
    world.register::<MeshComponent>();
    world.register::<MaterialComponent>();
    world.register::<TransformComponent>();
    world.register::<MeshComponent>();
    world.register::<EntityManager>();
    world.register::<EntityTree>();
    world.register::<Guid>();
    world.insert(AssetLibrary::default());
    SystemUtilities::setup(&mut world);
    // SystemUtilities::setup(world);

    // Insert default resources
    // world.insert(window_channel);
    // world.insert(world_id);
    world
  }
}
