use std::clone::Clone;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use utils::*;

use renderer::platform::VertexArray;
use renderer::*;

use events::{Event, EventChannel, EventPayload, KeyCode, ReceiverID, WindowEvent};

type TransformStack = Vec<Mat4F>;

struct Screen {
  pub screen_quad: VertexArray,
  pub shader: Shader,
  pub framebuffer: Framebuffer,
}

pub struct Renderer {
  // Screen
  screen: Screen,
  // Shader/Uniform Management
  config_uniforms: HashMap<CString, Uniform>, // Long-term uniforms
  common_uniforms: HashMap<CString, Uniform>, // common uniforms, change every frame

  // Asset management
  assets: AssetLibrary,
  queued_drawables: MultiMap<ShaderId, DrawCommand>,
  queued_overlays: Vec<Overlay>,

  // Config
  config: RendererConfig,
  receiver_id: ReceiverID,

  // Transform Stack
  transform_stack: TransformStack,
}

impl Default for Renderer {
  fn default() -> Self {
    Renderer {
      screen: create_screen(1600, 900),
      config_uniforms: HashMap::new(),
      common_uniforms: HashMap::new(),
      assets: AssetLibrary::default(),
      queued_drawables: MultiMap::new(),
      queued_overlays: Vec::new(),
      config: RendererConfig::default(),
      receiver_id: 0,
      transform_stack: vec![identity()],
    }
  }
}

impl Renderer {
  // Constructor
  pub fn new(screen_dims: Vec2F, channel: &mut EventChannel<WindowEvent>) -> Renderer {
    let receiver_id = channel.register_with_subs(&[
      WindowEvent::new(Event::WindowResized),
      WindowEvent::new(Event::KeyPressed(KeyCode::Tab)),
    ]);
    Renderer {
      screen: create_screen(screen_dims.x as i32, screen_dims.y as i32),
      config_uniforms: HashMap::new(),
      common_uniforms: HashMap::new(),
      assets: AssetLibrary::default(),
      queued_drawables: MultiMap::new(),
      queued_overlays: Vec::new(),
      config: RendererConfig::default(),
      receiver_id,
      transform_stack: vec![identity()],
    }
  }

  // Some general getters/setters
  pub fn set_dims(&mut self, dims: Vec2F) {
    self.screen.framebuffer.resize(Vec2I::new(dims.x as i32, dims.y as i32));
  }

  pub fn submit_common_uniform(&mut self, name: CString, uniform: Uniform, lifecycle: UniformLifecycle) {
    match lifecycle {
      UniformLifecycle::Frame => {
        self.common_uniforms.insert(name, uniform);
      }
      UniformLifecycle::Runtime => {
        self.config_uniforms.insert(name, uniform);
      }
    }
  }

  pub fn submit_shader(&mut self, shader: Shader) {
    self.assets.register_shader(shader);
  }

  pub fn submit_model(&mut self, model: DrawableState) -> DrawableId {
    self.assets.register_asset(model)
  }

  pub fn submit(&mut self, cmd: DrawCommand) {
    let s_id = self.assets.get_asset(&cmd.id).shader_id.clone();
    // let s_id = self.assets.get_shader(&active_shader);
    self.queued_drawables.push(s_id, cmd);
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

  pub fn start_scene<'a>(&mut self, camera: Camera<'a>, fps: f32) {
    // self.process_all_events();
    self.extract_camera_uniforms(&camera);

    let mut overlay = self.ui_box("Camera Uniforms");
    overlay.push(OverlayLine::HLine);
    overlay.push(OverlayLine::LabelText(
      "Position".to_string(),
      to_string!(camera.position),
    ));
    // overlay.push(OverlayLine::LabelText("Front".to_string(), to_string!(camera.front())));
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
    overlay.push(OverlayLine::IntInput(format!("Frame Time {:.4}", fps).to_string(), (fps * 1000.0) as i32));
    self.submit_2d(overlay);
  }

  pub fn init_frame(&mut self, window: &mut Window) {
    self.screen.framebuffer.bind();
    unsafe {
      gl::Enable(gl::DEPTH_TEST);
    }
    window.clear_framebuffer();
  }

  pub fn end_frame(&mut self, window: &mut Window) {
    self.screen.framebuffer.unbind();
    unsafe {
      gl::Disable(gl::DEPTH_TEST);
      // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }
    window.clear_framebuffer();
    self.screen.shader.bind();
    self
      .screen
      .shader
      .set_texture(1, c_str!("tex"), &self.screen.framebuffer.texture());
    self.screen.screen_quad.bind();
    self.screen.screen_quad.draw(&self.screen.shader.element_type);
    self.screen.shader.unbind();

    window.swap_buffers();
  }

  pub fn draw_scene(&mut self, window: &mut Window) {
    let mut active_shader: ShaderId = ShaderId(usize::MAX);
    for (s_id, cmd) in self.queued_drawables.iter() {
      let memo = self.assets.get_asset(&cmd.id);
      if s_id != &active_shader {
        active_shader = s_id.clone();
        self.switch_shader(self.assets.get_shader(&active_shader));
      }
      let mut texture_slot = 1;
      for (unif_name, unif) in memo.material.uniforms() {
        match unif {
          Uniform::Texture(tex) => {
            self
              .assets
              .get_shader(&active_shader)
              .set_texture(texture_slot, unif_name, tex);
            texture_slot += 1;
          }
          Uniform::CubeMap(tex) => {
            self
              .assets
              .get_shader(&active_shader)
              .set_texture(texture_slot, unif_name, tex);
            texture_slot += 1;
          }
          _ => self.assets.get_shader(&active_shader).set_uniform(&unif_name, &unif),
        }
      }
      self
        .assets
        .get_shader(&active_shader)
        .set_uniform(c_str!("model"), &Uniform::Mat4(cmd.transform.0));
      memo.vertex_array.bind();
      memo
        .vertex_array
        .draw(&self.assets.get_shader(&active_shader).element_type);
    }
    self.draw_imgui(window);
    self.queued_overlays.clear();
    self.queued_drawables.clear();
    self.common_uniforms.clear();
  }

  // Private helper functions

  fn draw_imgui(&mut self, window: &mut Window) {
    let mut y = 10f32;
    for i in 0..self.queued_overlays.len() {
      let overlay = self.queued_overlays.get_mut(i).unwrap();
      let ui = window.imgui_glfw.frame(&mut window.window, &mut window.im_context);
      overlay.render(&ui, y.clone());
      window.imgui_glfw.draw(ui, &mut window.window);
      y += overlay.height() + 10f32;
    }
  }

  fn switch_shader(&self, shader: &Shader) {
    shader.bind();
    for (unif_name, unif_value) in self.config_uniforms.iter() {
      shader.set_uniform(&unif_name, unif_value);
    }
    for (unif_name, unif_value) in self.common_uniforms.iter() {
      shader.set_uniform(&unif_name, unif_value);
    }
  }

  fn extract_camera_uniforms<'a>(&mut self, camera: &Camera<'a>) {
    self
      .common_uniforms
      .insert(CString::new("view").unwrap(), Uniform::Mat4(camera.view_matrix()));
    let f32_dims = Vec2F::new(
      self.screen.framebuffer.spec.dims.x as f32,
      self.screen.framebuffer.spec.dims.y as f32,
    );
    self.common_uniforms.insert(
      CString::new("projection").unwrap(),
      Uniform::Mat4(camera.projection_matrix(&f32_dims)),
    );
    self
      .common_uniforms
      .insert(CString::new("beta").unwrap(), Uniform::Float(camera.beta()));
    self
      .common_uniforms
      .insert(CString::new("gamma").unwrap(), Uniform::Float(camera.gamma()));
    self.common_uniforms.insert(
      CString::new("cameraPos").unwrap(),
      Uniform::Vec3(camera.position.clone()),
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

  pub fn process_events(&mut self, chanel: &mut EventChannel<WindowEvent>) {
    chanel
      .read(&self.receiver_id)
      .for_each(move |(window_event, _): (&WindowEvent, &())| match &window_event.code {
        Event::WindowResized => {
          if let Some(payload) = &window_event.payload {
            match payload {
              EventPayload::WindowSize(new_sz) => self.set_dims(new_sz.clone()),
              _ => {}
            }
          }
        }
        Event::KeyPressed(KeyCode::Tab) => {
          let new_config = RendererConfig::new(self.config.clone().mode.rotate());
          self.submit_config(new_config);
        }
        _ => {}
      });
  }
}

fn create_screen(w: i32, h: i32) -> Screen {
  let verts = vec![
    // Positions  // uv
    -1f32, 1f32, 0f32, 1f32, -1f32, -1f32, 0f32, 0f32, 1f32, -1f32, 1f32, 0f32, -1f32, 1f32, 0f32, 1f32, 1f32, -1f32,
    1f32, 0f32, 1f32, 1f32, 1f32, 1f32,
  ];

  let inds = vec![0, 1, 2, 3, 4, 5];
  let screen_quad = VertexArray::new(
    vec![VertexBuffer::create(
      verts,
      BufferLayout::new(vec![AttributeType::Float2, AttributeType::Float2]),
    )],
    IndexBuffer::create(inds),
  );
  let shader = Shader::from_file("renderer_screen", "shaders/screen_shader.glsl");

  Screen {
    framebuffer: Framebuffer::dims(w, h),
    shader,
    screen_quad,
  }
}
