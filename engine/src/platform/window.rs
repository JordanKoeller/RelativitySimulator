use std::ptr;

use glfw::Context;
use imgui::Context as ImContext;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;

use crate::debug::{gl_debug_output, print_limits};
use crate::utils::Vec2F;

pub type InputEvent = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;
pub type GLFW = glfw::Glfw;

pub struct Window {
    pub events: InputEvent,
    pub glfw_token: GLFW,
    pub window: glfw::Window,
    pub imgui_glfw: ImguiGLFW,
    pub im_context: ImContext,
    pub cursor: bool,
}

impl Default for Window {
    fn default() -> Self {
        Window::new(400, 400, "Game")
    }
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(feature = "debug")]
        glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true)); // comment this line in a release build!

        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        // glfw window creation
        // --------------------
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();

        window.set_all_polling(true);

        // gl: load all OpenGL function pointers
        // ---------------------------------------
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

        // print_limits();

        let mut im_ctx = ImContext::create();

        let imgui_glfw = ImguiGLFW::new(&mut im_ctx, &mut window);
        im_ctx.io_mut().config_flags |= imgui::ConfigFlags::NO_MOUSE_CURSOR_CHANGE;
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        // imgui_glfw.set_cursor_mode();
        // glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_HIDDEN);

        Window {
            events,
            glfw_token: glfw,
            window: window,
            imgui_glfw: imgui_glfw,
            im_context: im_ctx,
            cursor: false,
        }
    }

    pub fn native_window(&self) -> &glfw::Window {
        &self.window
    }

    pub fn toggle_cursor(&mut self) {
        if self.cursor {
            self.cursor = false;
            self.window.set_cursor_mode(glfw::CursorMode::Disabled);
        } else {
            self.cursor = true;
            self.window.set_cursor_mode(glfw::CursorMode::Normal);
        }
    }

    #[allow(dead_code)]
    pub fn set_closed(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn is_open(&self) -> bool {
        !self.native_window().should_close()
    }

    pub fn get_dims_f32(&self) -> Vec2F {
        let (x, y) = self.window.get_size();
        Vec2F::new(x as f32, y as f32)
    }

    pub fn poll_events(&mut self) {
        self.glfw_token.poll_events();
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn clear_intrinsic_canvas(&mut self) {
        unsafe {
            gl::ClearColor(0.1, 0.2, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn clear_framebuffer(&mut self) {
        unsafe {
            gl::ClearColor(0.7, 0.2, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw_ui<'ui>(&mut self, ui: imgui::Ui<'ui>) {
        self.imgui_glfw.draw(ui, &mut self.window);
    }

    pub fn close(&mut self) {
        glfw::terminate();
    }
}
