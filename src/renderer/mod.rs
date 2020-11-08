pub mod platform;
pub mod renderer;
pub mod renderer_config;
pub mod modeling;
pub mod camera;
pub mod ui;

pub use self::platform::*;
pub use self::renderer::*;
pub use self::modeling::*;
pub use self::camera::*;
pub use self::ui::*;
pub use self::renderer_config::*;

pub const LIGHT_SPEED: f32 = 12.0;
