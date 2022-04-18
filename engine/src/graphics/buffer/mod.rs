mod buffer_config;
mod buffer_layout;
mod index_buffer;
mod data_buffer;
mod buffer_helpers;
mod vertex_array;
mod vertex_array_id;
mod vertex_array_builder;
mod data_buffer_builder;

pub use self::data_buffer_builder::*;
pub use self::vertex_array_id::*;
pub use self::vertex_array_builder::*;
pub use self::vertex_array::*;
pub use self::buffer_helpers::*;
pub use self::buffer_config::*;
pub use self::buffer_layout::*;
pub use self::index_buffer::*;
pub use self::data_buffer::*;

use crate::datastructures::GenericRegistry;
pub type VertexArrayRegistry = GenericRegistry<VertexArrayBuilder>;