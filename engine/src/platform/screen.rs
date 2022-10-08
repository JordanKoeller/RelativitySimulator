use crate::utils::*;
use either::Either;
use std::clone::Clone;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::RwLockReadGuard;

use specs::prelude::*;

use crate::datastructures::{AVLTree, AVLTreeIterator, RegistryItem};
use crate::debug::*;
use crate::graphics::{
  AssetLibrary, AttributeType, BufferConfig, BufferLayout, DataBuffer, DataBufferBuilder, IndexBuffer,
  IndexBufferBuilder, MaterialComponent, MeshComponent, Shader, ShaderBuilder, Uniform, UniformLifecycle, VertexArray,
  VertexArrayBuilder,
};
use crate::platform::Window;
use crate::renderer::render_pipeline::*;
use crate::renderer::{DrawCall, Framebuffer, RenderQueueConsumer, RendererConfig};

use crate::ecs::Camera;

use crate::events::{Event, EventChannel, EventPayload, KeyCode, ReceiverId, StatelessEventChannel, WindowEvent};
use crate::physics::TransformComponent;

type TransformStack = Vec<Mat4F>;

pub struct Screen {
  screen_quad: VertexArray,
  shader: Shader,
  framebuffer: Framebuffer,
}

impl Screen {
  pub fn new(x_dim: i32, y_dim: i32) -> Self {
    let verts = vec![
      // Positions  // uv
      -1f32, 1f32, 0f32, 1f32, -1f32, -1f32, 0f32, 0f32, 1f32, -1f32, 1f32, 0f32, -1f32, 1f32, 0f32, 1f32, 1f32, -1f32,
      1f32, 0f32, 1f32, 1f32, 1f32, 1f32,
    ];
    let inds = vec![0, 1, 2, 3, 4, 5];
    let screen_quad = VertexArrayBuilder::default()
      .with_vertex_buffer(
        DataBufferBuilder::default()
          .with_layout(BufferLayout::new(vec![AttributeType::Float2, AttributeType::Float2]))
          .with_data(verts)
          .with_config(BufferConfig::static_vbo()),
      )
      .with_index_buffer(IndexBufferBuilder::default().with_data(inds))
      .build();
    let shader = ShaderBuilder::default()
      .with_source_file("shaders/screen_shader.glsl")
      .build();
    Self {
      framebuffer: Framebuffer::from_dims(x_dim, y_dim),
      shader,
      screen_quad,
    }
  }

  pub fn draw_framebuffer_contents(&self) {
    self.shader.bind();
    let texture_slot = 32;
    self.framebuffer.bind_texture_slot(texture_slot);
    let buffer_texture = Uniform::Int(texture_slot as i32);
    self.shader.set_uniform("tex", &buffer_texture);
    self.screen_quad.bind();
    self.screen_quad.draw(self.shader.element_type());
    self.screen_quad.unbind();
    self.framebuffer.unbind_texture_slot(texture_slot);
    self.shader.unbind();
  }

  pub fn bind_framebuffer(&self) {
    self.framebuffer.bind();
  }

  pub fn unbind_framebuffer(&self) {
    self.framebuffer.unbind();
  }

  pub fn aspect_ratio(&self) -> f32 {
    let dims = &self.framebuffer.spec.dims;
    (dims.x as f32) / (dims.y as f32)
  }

  pub fn set_framebuffer(&mut self, fb: Framebuffer) {
    self.framebuffer = fb;
  }
}
