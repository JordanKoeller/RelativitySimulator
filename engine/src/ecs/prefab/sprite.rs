use cgmath::prelude::*;
use specs::prelude::*;
use specs::world::LazyBuilder;

use crate::ecs::PrefabBuilder;
use crate::utils::{Vec2F, Vec3F};

struct SpriteBuilderState {
    path: String,
    aspect_ratio: f64,
}

// struct SpriteBuilder;

// impl PrefabBuilder for SpriteBuilder {
//     type PrefabState = SpriteBuilderState;

//     fn build(&self, entity_builder: LazyBuilder<'_>, state: Self::PrefabState) -> LazyBuilder<'_> {
//         entity_builder
//     }

// }

static QUAD_VERTICES: [f64; 32] = [
    0.5f64, 0.5f64, 0.0f64, 0f64, 0f64, -1f64, 1.0f64, 1.0f64, // top right
    0.5f64, -0.5f64, 0.0f64, 0f64, 0f64, -1f64, 1.0f64, 0.0f64, // bottom right
    -0.5f64, -0.5f64, 0.0f64, 0f64, 0f64, -1f64, 0.0f64, 0.0f64, // bottom left
    -0.5f64, 0.5f64, 0.0f64, 0f64, 0f64, -1f64, 0.0f64, 1.0f64, // top left
];

static QUAD_INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];
