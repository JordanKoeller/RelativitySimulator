
use glfw::Context as GLFWContext;
use gl;

pub struct Context {
  pub window: glfw::Window,
}

impl Context {
  pub fn new(mut window: glfw::Window) -> Context {
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
      gl::Enable(gl::DEPTH_TEST);
      gl::Enable(gl::MULTISAMPLE);
      gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
    }
    Context {
      window
    }
  }

  pub fn swap_buffers(&mut self) {
    self.window.swap_buffers();
  }
}