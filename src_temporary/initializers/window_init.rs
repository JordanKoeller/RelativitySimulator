
use glfw::Context;

pub type InputEvent = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;
pub type GLFW = glfw::Glfw;

pub fn window_init(width: u32, height: u32, title: &str) -> (glfw::Window, InputEvent, GLFW) {
    // let mut first_move = true;
    // let mut last_x: f32 = width as f32 / 2.0;
    // let mut last_y: f32 = height as f32 / 2.0;
    // // timing
    // let mut delta_time: f32; // time between current frame and last frame
    // let mut last_frame: f32 = 0.0;
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
    // glfw window creation
    // --------------------
    let (mut window, events) = glfw
        .create_window(
            width,
            height,
            title,
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    // let (mut window2, events2) = glfw
    // .create_window(SCR_WIDTH, SCR_HEIGHT, "Helper Window", glfw::WindowMode::Windowed)
    // .expect("Failed to create GLFW window");
    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_key_polling(true);
    // tell GLFW to capture our mouse
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        // gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
    }
    
    (window, events, glfw)
}
