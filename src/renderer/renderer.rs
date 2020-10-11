use std::clone::Clone;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use utils::*;

use cgmath::prelude::*;

use renderer::platform::VertexArray;
use renderer::Camera;
use renderer::DrawableMemo;
use renderer::RenderCommand;
use renderer::Shader;
use renderer::Window;
use renderer::{Drawable, Overlay, OverlayLine, RendererConfig};
use renderer::{Uniform, UniformLifecycle};

use events::{Event, EventDispatcher, EventPayload, EventReceiver, EventReceiverState, KeyCode, WithEventReceiver};

type ShaderLibrary = HashMap<String, Box<dyn Shader>>;

pub struct Renderer {
  screen_dims: Vec2F,
  // Shader/Uniform Management
  shader_library: ShaderLibrary,
  config_uniforms: HashMap<CString, Uniform>, // Long-term uniforms
  common_uniforms: HashMap<CString, Uniform>, // common uniforms, change every frame

  // Asset management
  queued_drawables: MultiMap<String, DrawableMemo>,
  queued_overlays: Vec<Overlay>,

  // Config
  config: RendererConfig,
  event_receiver: EventReceiverState<Self>,
}

impl Renderer {
  // Constructor
  pub fn new(screen_dims: Vec2F, dispatcher: MutRef<dyn EventDispatcher>) -> Renderer {
    let evts = EventReceiverState::new(dispatcher, 1);
    let mut ret = Renderer {
      screen_dims,
      shader_library: ShaderLibrary::new(),
      config_uniforms: HashMap::new(),
      common_uniforms: HashMap::new(),
      queued_drawables: MultiMap::new(),
      queued_overlays: Vec::new(),
      config: RendererConfig::default(),
      event_receiver: evts,
    };

    ret.set_events();
    ret
  }

  // Some general getters/setters
  pub fn set_dims(&mut self, dims: Vec2F) {
    self.screen_dims = dims;
  }

  pub fn submit_common_uniform(&mut self, name: CString, uniform: Uniform, lifecycle: UniformLifecycle) {
    match lifecycle {
      UniformLifecycle::Frame => {
        self.common_uniforms.insert(name, uniform);
      }
      UniformLifecycle::Runtime => {
        self.config_uniforms.insert(name, uniform);
        // TODO: Do something to send the uniform to all the shaders
      }
    }
  }

  pub fn submit_shader(&mut self, shader: Box<dyn Shader>) {
    self.shader_library.insert(shader.name().to_string(), shader);
  }

  pub fn submit(&mut self, cmd: RenderCommand) {
    match cmd {
      RenderCommand::SingleDrawable(drawable) => self.queued_drawables.push(drawable.shader_name.clone(), drawable),
      // RenderCommand::MultiDrawable(drawables) => for d in drawables.iter() {
      //   self.queued_drawables.push(d.shader_name().to_string(), Ref::clone(d));
      // }
    }
    // self.queued_drawables.push(cmd.drawable.shader_name().to_string(), cmd);
  }

  pub fn ui_box(&self, title: &str) -> Overlay {
    Overlay::new(title)
  }

  pub fn submit_2d(&mut self, cmd: Overlay) {
    self.queued_overlays.push(cmd);
  }

  pub fn submit_config(&mut self, config: RendererConfig) {
    self.submit_common_uniform(
      CString::from(c_str!("lorentzFlag")),
      Uniform::Int(config.relativity_mode()),
      UniformLifecycle::Runtime,
    );
    self.config = config;
  }

  // Methods that do something instead of just get/set things

  pub fn start_scene(&mut self, camera: &dyn Camera) {
    self.process_all_events();
    self.extract_camera_uniforms(camera);

    let mut overlay = self.ui_box("Camera Uniforms");
    overlay.push(OverlayLine::HLine);
    overlay.push(OverlayLine::LabelText(
      "Position".to_string(),
      to_string!(camera.position()),
    ));
    overlay.push(OverlayLine::LabelText("Front".to_string(), to_string!(camera.front())));
    overlay.push(OverlayLine::LabelText(
      "Beta:".to_string(),
      format!("{0:.3}", camera.beta()),
    ));
    overlay.push(OverlayLine::LabelText(
      "Gamma:".to_string(),
      format!("{0:.3}", camera.gamma()),
    ));
    overlay.push(OverlayLine::LabelText(
      "Render Mode".to_string(),
      format!("{:?}", self.config.mode),
    ));
    self.submit_2d(overlay);
  }

  pub fn draw_scene(&mut self, window: &mut Window) {
    let mut shader_name = "".to_string();
    for (s_name, drawable) in self.queued_drawables.iter() {
      let active_shader = self.shader_library.get(s_name).unwrap();
      let tmp_name = s_name.clone();
      if tmp_name != shader_name {
        shader_name = tmp_name;
        self.switch_shader(active_shader);
      }
      let material = &drawable.material;
      let mut texture_slot = 1;
      for (unif_name, unif) in material.uniforms() {
        match unif {
          Uniform::Texture(tex) => {
            active_shader.set_texture(texture_slot, unif_name, tex);
            texture_slot += 1;
          }
          Uniform::CubeMap(tex) => {
            active_shader.set_texture(texture_slot, unif_name, tex);
            texture_slot += 1;
          }
          _ => active_shader.set_uniform(&unif_name, &unif),
        }
      }
      active_shader.set_uniform(c_str!("model"), &Uniform::Mat4(*drawable.transform));
      drawable.vertex_array.bind();
      drawable.vertex_array.draw();
    }
    self.draw_imgui(window);
    self.queued_overlays.clear();
    self.queued_drawables.clear();
    self.common_uniforms.clear();
  }

  // Private helper functions

  fn draw_imgui(&mut self, window: &mut Window) {
    let mut y = 10f32;
    for overlay in self.queued_overlays.iter() {
      let ui = window.imgui_glfw.frame(&mut window.window, &mut window.im_context);
      overlay.render(&ui, y.clone());
      window.imgui_glfw.draw(ui, &mut window.window);
      y += overlay.height() + 10f32;
    }
  }

  fn switch_shader(&self, shader: &Box<dyn Shader>) {
    shader.bind();
    for (unif_name, unif_value) in self.config_uniforms.iter() {
      shader.set_uniform(&unif_name, unif_value);
    }
    for (unif_name, unif_value) in self.common_uniforms.iter() {
      shader.set_uniform(&unif_name, unif_value);
    }
  }

  fn extract_camera_uniforms(&mut self, camera: &dyn Camera) {
    self
      .common_uniforms
      .insert(CString::new("view").unwrap(), Uniform::Mat4(camera.view_matrix()));
    self.common_uniforms.insert(
      CString::new("projection").unwrap(),
      Uniform::Mat4(camera.projection_matrix(&self.screen_dims)),
    );
    self
      .common_uniforms
      .insert(CString::new("beta").unwrap(), Uniform::Float(camera.beta()));
    self
      .common_uniforms
      .insert(CString::new("gamma").unwrap(), Uniform::Float(camera.gamma()));
    self.common_uniforms.insert(
      CString::new("cameraPos").unwrap(),
      Uniform::Vec3(camera.position().clone()),
    );
    self.common_uniforms.insert(
      CString::new("changeOfBasis").unwrap(),
      Uniform::Mat3(camera.velocity_basis_matrix()),
    );
    self.common_uniforms.insert(
      CString::new("changeOfBasisInverse").unwrap(),
      Uniform::Mat3(camera.velocity_inverse_basis_matrix()),
    );
  }

  fn set_events(&mut self) {
    self.subscribe_to(Event::KeyPressed(KeyCode::Tab), |renderer, _| {
      let mut config = renderer.config.clone();
      config.mode = config.mode.rotate();
      renderer.submit_config(config);
    });
    self.subscribe_to(Event::WindowResized, |renderer, (_, payload)| {
      match payload {
        Some(EventPayload::WindowSize(dims)) => {
          renderer.set_dims(dims);
        }
        _ => {}
      }
      // renderer.submit_config(config);
    });
  }
}

impl WithEventReceiver for Renderer {
  fn state(&self) -> &EventReceiverState<Self> {
    &self.event_receiver
  }
  fn state_mut(&mut self) -> &mut EventReceiverState<Self> {
    &mut self.event_receiver
  }
}
