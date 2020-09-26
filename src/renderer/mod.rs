pub mod platform;
pub mod renderer;
pub mod render_command;
pub mod modeling;
pub mod camera;
pub mod ui;

pub use self::platform::*;
pub use self::renderer::*;
pub use self::render_command::*;
pub use self::modeling::*;
pub use self::camera::*;
pub use self::ui::*;

pub const LIGHT_SPEED: f32 = 36.0;
