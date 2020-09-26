use std::clone::Clone;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use utils::*;

use cgmath::prelude::*;

use renderer::platform::VertexArray;
use renderer::Camera;
use renderer::RenderCommand;
use renderer::Shader;
use renderer::Window;
use renderer::{Uniform, UniformLifecycle};

use renderer::{Overlay, OverlayLine};

type ShaderLibrary = HashMap<String, Shader>;

pub struct Renderer {
  screen_dims: Vec2F,
  // Shader/Uniform Management
  shader_library: ShaderLibrary,
  config_uniforms: HashMap<CString, Uniform>, // Long-term uniforms
  common_uniforms: HashMap<CString, Uniform>, // common uniforms, change every frame

  // Asset management
  queued_drawables: MultiMap<String, RenderCommand>,
  queued_overlays: Vec<Overlay>,
}

impl Renderer {
  // Constructor
  pub fn new(screen_dims: Vec2F) -> Renderer {
    Renderer {
      screen_dims,
      shader_library: ShaderLibrary::new(),
      config_uniforms: HashMap::new(),
      common_uniforms: HashMap::new(),
      queued_drawables: MultiMap::new(),
      queued_overlays: Vec::new(),
    }
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

  pub fn submit_shader(&mut self, shader: Shader) {
    self.shader_library.insert(shader.name().to_string(), shader);
  }

  pub fn submit(&mut self, cmd: RenderCommand) {
    self.queued_drawables.push(cmd.drawable.shader_name().to_string(), cmd);
  }

  pub fn ui_box(&self, title: &str) -> Overlay {
    Overlay::new(title)
  }

  pub fn submit_2d(&mut self, cmd: Overlay) {
    self.queued_overlays.push(cmd);
  }

  // Methods that do something instead of just get/set things

  pub fn start_scene(&mut self, camera: &dyn Camera) {

    self.extract_camera_uniforms(camera);

    let mut overlay = self.ui_box("Camera Uniforms");
    overlay.push(OverlayLine::HLine);
    overlay.push(OverlayLine::LabelText("Position".to_string(), to_string!(camera.position())));
    overlay.push(OverlayLine::LabelText("Beta:".to_string(), format!("{0:.3}", camera.beta())));
    overlay.push(OverlayLine::LabelText("Gamma:".to_string(), format!("{0:.3}", camera.gamma())));
    self.submit_2d(overlay);
  }


  pub fn draw_scene(&mut self, window: &mut Window) {
    let mut shader_name = "".to_string();
    for (s_name, render_cmd) in self.queued_drawables.iter() {
      let active_shader = self.shader_library.get(s_name).unwrap();
      let tmp_name = s_name.clone();
      if tmp_name != shader_name {
        shader_name = tmp_name;
        self.switch_shader(active_shader);
      }
      for (unif_name, unif_value) in render_cmd.drawable.material() {
        active_shader.set_uniform(&unif_name, unif_value.clone());
      }
      if let Some(texture_list) = render_cmd.drawable.textures() {
        for (i, (tex_name, tex_value)) in texture_list.iter().enumerate() {
          active_shader.set_texture(i as u32, tex_name, tex_value);
        }
      }
      render_cmd.drawable.vertex_array().bind();
      render_cmd.drawable.vertex_array().draw();
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

  fn switch_shader(&self, shader: &Shader) {
    shader.bind();
    for (unif_name, unif_value) in self.config_uniforms.iter() {
      shader.set_uniform(&unif_name, unif_value.clone());
    }
    for (unif_name, unif_value) in self.common_uniforms.iter() {
      shader.set_uniform(&unif_name, unif_value.clone());
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
    self
      .common_uniforms
      .insert(CString::new("lorentzFlag").unwrap(), Uniform::Int(0));
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
}
