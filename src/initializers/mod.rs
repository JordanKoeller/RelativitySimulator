mod shader_preprocessor;
mod shader_factory;
mod model_factory;
mod factory;

pub mod window_init;
pub mod shader_spec;
pub mod mesh_spec;
pub mod resource_manager;
pub mod asset_spec;

use self::factory::Factory;
use self::shader_factory::ShaderFactory;
use self::model_factory::GLBufferFactory;

// pub use self::asset_factory::AssetFactory;
pub use self::window_init::window_init;
pub use self::mesh_spec::{GLSpec, AttributeTypes};
pub use self::shader_spec::{ShaderSpec, NormalShaderSpec};
pub use self::resource_manager::{ResourceManger, AssetManager};
pub use self::asset_spec::AssetSpec;