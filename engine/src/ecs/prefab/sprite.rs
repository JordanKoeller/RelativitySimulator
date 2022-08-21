use cgmath::prelude::*;
use specs::prelude::*;
use specs::world::LazyBuilder;

use crate::ecs::PrefabBuilder;
use crate::utils::{Vec2F, Vec3F};

struct SpriteBuilderState {
  path: String,
  aspect_ratio: f32,
}

// struct SpriteBuilder;

// impl PrefabBuilder for SpriteBuilder {
//     type PrefabState = SpriteBuilderState;

//     fn build(&self, entity_builder: LazyBuilder<'_>, state: Self::PrefabState) -> LazyBuilder<'_> {
//         entity_builder
//     }

// }

static QUAD_VERTICES: [f32; 32] = [
  0.5f32, 0.5f32, 0.0f32, 0f32, 0f32, -1f32, 1.0f32, 1.0f32, // top right
  0.5f32, -0.5f32, 0.0f32, 0f32, 0f32, -1f32, 1.0f32, 0.0f32, // bottom right
  -0.5f32, -0.5f32, 0.0f32, 0f32, 0f32, -1f32, 0.0f32, 0.0f32, // bottom left
  -0.5f32, 0.5f32, 0.0f32, 0f32, 0f32, -1f32, 0.0f32, 1.0f32, // top left
];

static QUAD_INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];
