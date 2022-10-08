mod shader;
mod shader_builder;
mod shader_delegates;
mod shader_id;
mod shader_preprocessor;

pub use self::shader_builder::*;
use shader_delegates::*;

pub use self::shader::Shader;
pub use self::shader_delegates::ShaderDepthFunction;
pub use self::shader_id::ShaderId;

use crate::datastructures::GenericRegistry;
pub type ShaderRegistry = GenericRegistry<ShaderBuilder>;
