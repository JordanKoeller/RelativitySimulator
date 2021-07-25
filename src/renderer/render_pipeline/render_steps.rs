use std::collections::HashMap;
use std::ffi::{CStr, CString};

use either::Either;
use specs::prelude::*;

use renderer::{
  AssetLibrary, GPUState, Mesh, RenderCommand, RenderQueueConsumer, Shader, ShaderId, TextureBinder, Uniform,
};

use ecs::components::{DrawableId, Material};
use physics::TransformComponent;
use utils::Mat4F;

use debug::*;

pub trait RenderStep {}

// The pipeline is eager to start accepting uniforms.
// The shader/mesh are bound, but no uniforms are bound.
// Thus the pipeline is not ready for draw calls yet.
pub struct ReadyToDrawStep;
impl RenderStep for ReadyToDrawStep {}

// Global uniforms/UBOs have been bound. It's time to start
// pushing materials/model matrix.
// Given a RenderQueue, this step will bind a provided
// material/matrix, OR consume as meany elements from the queue
// as necessary to exhaust all instances of this particular mesh.
pub struct ActivatedShaderStep;
impl RenderStep for ActivatedShaderStep {}

// Everything is bound. All Uniforms are pushed to GPU. All that remains
// is to do a draw call.
pub struct SaturatedDrawCallStep;
impl RenderStep for SaturatedDrawCallStep {}

// The last drawable has been pushed to the GPU. Now it's time to either
// proceed to the next mesh/shader in the render queue, or if the queue
// is exhausted, move on to deferred lighting and framebufer drawing.
pub struct FlushedDrawCallStep;
impl RenderStep for FlushedDrawCallStep {}

// All renderables have been drawn. Now it's time for second/third pass
pub struct MeshesDrawnStep;
impl RenderStep for MeshesDrawnStep {}

pub struct RenderPipeline<'a, S: RenderStep> {
  _marker: std::marker::PhantomData<S>,
  state: GPUState<'a>,
}

impl<'a, S: RenderStep> RenderPipeline<'a, S> {
  fn consume<T: RenderStep>(self) -> RenderPipeline<'a, T> {
    RenderPipeline {
      _marker: std::marker::PhantomData::default(),
      state: self.state,
    }
  }
}

impl<'a> RenderPipeline<'a, ReadyToDrawStep> {
  pub fn new<'b>(queue: &mut RenderQueueConsumer<'b>, assets: &'a mut AssetLibrary) -> Option<Self> {
    if let Some(state) = queue.peek() {
      let mut ret = Self {
        _marker: std::marker::PhantomData::default(),
        state: assets.select(&state.drawable),
      };
      ret.state.shader().bind();
      ret.state.mesh().vao.bind();
      Some(ret)
    } else {
      None
    }
  }

  pub fn bind_global_uniforms<'b>(
    mut self,
    uniforms: &[&HashMap<CString, Uniform>],
  ) -> RenderPipeline<'a, ActivatedShaderStep> {
    for mgr in uniforms.iter() {
      for (unif_name, unif_value) in mgr.iter() {
        self.state.shader().set_uniform(&unif_name, unif_value);
      }
    }
    self.consume()
  }
}

impl<'a> RenderPipeline<'a, ActivatedShaderStep> {
  fn activated_on(&self, d_id: &DrawableId) -> bool {
    self.state.id.0 == d_id.0 && self.state.id.1 == d_id.1
  }
  pub fn intake_queue<'b>(
    mut self,
    queue: &mut RenderQueueConsumer<'b>,
    materials: &ReadStorage<'a, Material>,
    models: &ReadStorage<'a, TransformComponent>,
  ) -> RenderPipeline<'a, SaturatedDrawCallStep> {
    if self.state.mesh().instanced() {
      let ret = self.ingress_instances(queue, materials, models);
      ret
    } else {
      let ret = self.ingress_drawable(queue, materials, models);
      ret
    }
  }

  fn ingress_instances<'b>(
    mut self,
    queue: &mut RenderQueueConsumer<'b>,
    materials: &ReadStorage<'a, Material>,
    models: &ReadStorage<'a, TransformComponent>,
  ) -> RenderPipeline<'a, SaturatedDrawCallStep> {
    loop {
      if let Some(dc) = queue.pop_if(|dc| {
        let ret = self.activated_on(&dc.drawable);
        ret
      }) {
        match dc.cmd {
          RenderCommand::Draw => {
            self.state.upsert_instance(
              &dc.entity,
              &models.get(dc.entity).unwrap().matrix(),
              &materials.get(dc.entity).unwrap(),
            );
          }
          RenderCommand::Free => {
            self.state.mesh().clear_instance(&dc.entity);
          }
        }
      } else {
        return self.consume();
      }
    }
  }

  fn ingress_drawable<'b>(
    mut self,
    queue: &mut RenderQueueConsumer<'b>,
    materials: &ReadStorage<'a, Material>,
    models: &ReadStorage<'a, TransformComponent>,
  ) -> RenderPipeline<'a, SaturatedDrawCallStep> {
    if let Some(dc) = queue.next() {
      let model = models.get(dc.entity).unwrap().matrix();
      let mtl = materials.get(dc.entity).unwrap();
      self.state.shader().set_uniform(c_str!("model"), &Uniform::Mat4(model));
      self.state.bind_material(&mtl);
    }
    self.consume()
  }
}

impl<'a> RenderPipeline<'a, SaturatedDrawCallStep> {
  pub fn flush(self) -> RenderPipeline<'a, FlushedDrawCallStep> {
    self.state.mesh_immut().draw(&self.state.shader_immut().element_type);
    self.consume()
  }
}

impl<'a> RenderPipeline<'a, FlushedDrawCallStep> {
  // TODO: Handle moving to the next ReadyStep or the next ActivatedStep
  pub fn proceed<'b>(
    self,
    queue: &mut RenderQueueConsumer<'b>,
  ) -> Either<RenderPipeline<'a, ReadyToDrawStep>, RenderPipeline<'a, ActivatedShaderStep>> {
    if let Some(dc) = queue.peek() {
      if dc.drawable.1 == self.state.id.1 {
        // We already have the appropriate shader active
        let mut ret = RenderPipeline::<'a, ActivatedShaderStep> {
          _marker: std::marker::PhantomData::default(),
          state: self.state.assets.select(&dc.drawable),
        };
        ret.state.clear_textures();
        ret.state.mesh().unbind();
        ret.state.mesh().vao.bind();
        Either::Right(ret)
      } else {
        let mut ret = RenderPipeline::<'a, ReadyToDrawStep> {
          _marker: std::marker::PhantomData::default(),
          state: self.state.assets.select(&dc.drawable),
        };
        ret.state.clear_textures();
        ret.state.mesh().unbind();
        ret.state.shader().unbind();
        ret.state.shader().bind();
        ret.state.mesh().vao.bind();
        Either::Left(ret)
      }
    } else {
      panic!("Tried to proceed a render pipeline with an empty render queue!");
    }
  }
}
