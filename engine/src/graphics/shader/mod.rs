

mod shader;
mod shader_builder;
mod shader_preprocessor;
mod shader_id;
mod shader_delegates;

use shader_delegates::*;
pub use self::shader_builder::*;

pub use self::shader::Shader;
pub use self::shader_id::ShaderId;

use crate::datastructures::GenericRegistry;
pub type ShaderRegistry = GenericRegistry<ShaderBuilder>;