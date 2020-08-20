
pub mod shader;
pub mod shader_manager;
pub mod modeling;
pub mod uniform;
mod shader_preprocessor;

pub use self::shader::Shader;
pub use self::shader_manager::ShaderManager;
pub use self::uniform::{UniformType, UniformValue, UniformManager};

use self::shader_preprocessor::shader_preprocessor;