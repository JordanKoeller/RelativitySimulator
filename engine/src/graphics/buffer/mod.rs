mod buffer_config;
mod buffer_helpers;
mod buffer_interfaces;
mod buffer_layout;
mod data_buffer;
mod data_buffer_builder;
mod index_buffer;
mod vertex_array;
mod vertex_array_builder;
mod vertex_array_id;

pub use self::buffer_config::*;
pub use self::buffer_helpers::*;
pub use self::buffer_interfaces::*;
pub use self::buffer_layout::*;
pub use self::data_buffer::*;
pub use self::data_buffer_builder::*;
pub use self::index_buffer::*;
pub use self::vertex_array::*;
pub use self::vertex_array_builder::*;
pub use self::vertex_array_id::*;

use crate::datastructures::GenericRegistry;
pub type VertexArrayRegistry = GenericRegistry<VertexArrayBuilder>;
