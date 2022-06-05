use crate::debug::{gl_debug_output, print_limits};
use glfw::Context;
use std::ptr;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

pub struct TestGLContext {
    glfw_token: glfw::Glfw,
    window: glfw::Window,
}

impl Default for TestGLContext {
    fn default() -> Self {
        let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true)); // comment this line in a release build!
        glfw.window_hint(glfw::WindowHint::Visible(false)); // Prevents window from opening in tester code.
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        // glfw window creation
        // --------------------
        let (mut window, _) = glfw
            .create_window(640, 480, "Testing Window Context", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        unsafe {
            let mut flags = 0;
            gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut flags);
            if flags as u32 & gl::CONTEXT_FLAG_DEBUG_BIT != 0 {
                gl::Enable(gl::DEBUG_OUTPUT);
                gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS); // makes sure errors are displayed synchronously
                gl::DebugMessageCallback(gl_debug_output, ptr::null());
                gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, ptr::null(), gl::TRUE);
            } else {
                println!("======================================================================");
                println!("Debug Context not active! Check if your driver supports the extension.");
                println!("======================================================================");
            }
            gl::Enable(gl::MULTISAMPLE);
            glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::ClearColor(0.2, 0.1, 0.0, 1.0);
            // glfw.set_swap_interval(glfw::SwapInterval::None);
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
        TestGLContext {
            glfw_token: glfw,
            window,
        }
    }
}

unsafe impl Sync for TestGLContext {}
unsafe impl Send for TestGLContext {}

lazy_static! {
    pub static ref GL_CONTEXT: Arc<Mutex<TestGLContext>> = Arc::new(Mutex::new(TestGLContext::default()));
}

pub fn get_context() -> MutexGuard<'static, TestGLContext> {
    GL_CONTEXT.lock().unwrap()
}
