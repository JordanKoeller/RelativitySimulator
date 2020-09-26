use glfw::Context;
use imgui::Context as ImContext;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;

pub type InputEvent = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;
pub type GLFW = glfw::Glfw;

pub struct Window {
  pub events: InputEvent,
  pub glfw_token: GLFW,
  pub window: glfw::Window,
  pub imgui_glfw: ImguiGLFW,
  pub im_context: ImContext,
}

impl Window {
  pub fn new(width: u32, height: u32, title: &str) -> Window {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Samples(Some(8)));
    // glfw window creation
    // --------------------
    let (mut window, events) = glfw
      .create_window(width, height, title, glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window");

    window.make_current();
    // window.set
    window.set_cursor_pos_polling(true);
    // window.set_cursor_enter_polling(true);
    window.set_key_polling(true);
    window.set_scroll_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
      // gl::Enable(gl::CULL_FACE);
      // gl::Enable(gl::DEPTH_TEST);
      // gl::Enable(gl::MULTISAMPLE);
      // gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
      gl::Enable(gl::BLEND);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
      gl::Enable(gl::DEPTH_TEST);
      gl::DepthFunc(gl::LESS);
      gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    }

    let mut im_ctx = ImContext::create();

    let imgui_glfw = ImguiGLFW::new(&mut im_ctx, &mut window);
    im_ctx.io_mut().config_flags |= imgui::ConfigFlags::NO_MOUSE_CURSOR_CHANGE;
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    // imgui_glfw.set_cursor_mode()
    // glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_HIDDEN);

    Window {
      events,
      glfw_token: glfw,
      window: window,
      imgui_glfw: imgui_glfw,
      im_context: im_ctx,
    }
  }

  pub fn native_window(&self) -> &glfw::Window {
    &self.window
  }

  pub fn disable_cursor(&mut self) {
    self.window.set_cursor_mode(glfw::CursorMode::Disabled);
  }

  pub fn enable_cursor(&mut self) {
    self.window.set_cursor_mode(glfw::CursorMode::Normal);
  }

  pub fn set_closed(&mut self) {
    self.window.set_should_close(true);
  }

  pub fn is_open(&self) -> bool {
    !self.native_window().should_close()
  }

  pub fn poll_events(&mut self) {
    self.glfw_token.poll_events();
  }

  pub fn swap_buffers(&mut self) {
    self.window.swap_buffers();
  }

  pub fn frame_end(&mut self) {
    self.swap_buffers();
  }

  pub fn frame_start(&mut self) {
    self.poll_events();
    unsafe {
      gl::ClearColor(0.1, 0.2, 0.3, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
  }
}
