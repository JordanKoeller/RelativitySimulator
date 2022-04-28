pub mod platform;
pub mod render_command;
pub mod render_queue;
pub mod renderer;
pub mod renderer_config;
pub mod render_pipeline;

pub use self::platform::*;
pub use self::render_pipeline::*;
pub use self::renderer::*;
pub use self::render_command::*;
pub use self::render_queue::*;
pub use self::renderer_config::*;
pub const LIGHT_SPEED: f32 = 12.0;
