mod texture_asset;
mod texture_binder;
mod texture_builder;
mod texture_helpers;
mod texture_id;

pub use self::texture_asset::*;
pub use self::texture_binder::*;
pub use self::texture_builder::*;
pub use self::texture_helpers::*;
pub use self::texture_id::*;

use crate::datastructures::GenericRegistry;
pub type TextureRegistry = GenericRegistry<TextureBuilder>;
