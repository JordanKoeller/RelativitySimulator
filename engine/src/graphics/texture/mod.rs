mod texture_asset;
mod texture_builder;
mod texture_helpers;
mod texture_id;
mod texture_binder;

use self::texture_binder::*;
pub use self::texture_id::*;
pub use self::texture_helpers::*;
pub use self::texture_asset::*;
pub use self::texture_builder::*;



use crate::datastructures::GenericRegistry;
pub type TextureRegistry = GenericRegistry<TextureBuilder>;