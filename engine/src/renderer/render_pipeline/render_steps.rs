use std::collections::HashMap;
use std::ffi::{CStr, CString};

use either::Either;
use specs::prelude::*;

use crate::renderer::{
     GPUState, RenderCommand, RenderQueueConsumer,
};

use crate::graphics::{
    MeshComponent, Shader, ShaderId, TextureBinder, Uniform, TextureId, AssetLibrary, MaterialComponent
};

use crate::physics::TransformComponent;
use crate::utils::Mat4F;

use crate::debug::*;

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
        if let Some(mut state) = queue.peek() {
            let mut ret = Self {
                _marker: std::marker::PhantomData::default(),
                state: GPUState::new(assets, state.mesh_component)
            };
            ret.state.bind_shader();
            ret.state.bind_mesh();
            // ret.state.bind();
            Some(ret)
        } else {
            None
        }
    }

    pub fn bind_global_uniforms<'b>(
        mut self,
        uniforms: &[&HashMap<String, Uniform>],
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
    fn activated_on(&self, mesh: &MeshComponent) -> bool {
        self.state.active_mesh == mesh.clone()
    }
    pub fn intake_queue<'b>(
        self,
        queue: &mut RenderQueueConsumer<'b>,
        materials: &ReadStorage<'a, MaterialComponent>,
        models: &ReadStorage<'a, TransformComponent>,
    ) -> RenderPipeline<'a, SaturatedDrawCallStep> {
            let ret = self.ingress_drawable(queue, materials, models);
            ret
    }

    fn ingress_drawable<'b>(
        mut self,
        queue: &mut RenderQueueConsumer<'b>,
        materials: &ReadStorage<'a, MaterialComponent>,
        models: &ReadStorage<'a, TransformComponent>,
    ) -> RenderPipeline<'a, SaturatedDrawCallStep> {
        if let Some(dc) = queue.next() {
            let model = models.get(dc.entity).unwrap().matrix();
            let mtl = materials.get(dc.entity).unwrap();
            self.state.shader().set_uniform("model", &Uniform::Mat4(model));
            self.state.bind_material(&mtl);
        }
        self.consume()
    }
}

impl<'a> RenderPipeline<'a, SaturatedDrawCallStep> {
    pub fn flush(mut self) -> RenderPipeline<'a, FlushedDrawCallStep> {
        self.state.draw();
        // self.state.mesh_immut().draw(&self.state.shader_immut().element_type);
        self.consume()
    }
}

impl<'a> RenderPipeline<'a, FlushedDrawCallStep> {
    // TODO: Handle moving to the next ReadyStep or the next ActivatedStep
    pub fn proceed<'b>(
        self,
        queue: &mut RenderQueueConsumer<'b>,
    ) -> Either<RenderPipeline<'a, ReadyToDrawStep>, RenderPipeline<'a, ActivatedShaderStep>> {
        if let Some( draw_call) = queue.peek() {
            if draw_call.mesh_component.shader_id == self.state.active_mesh.shader_id {
            // if dc.drawable.1 == self.state.id.1 {
                // We already have the appropriate shader active
                let mut ret = RenderPipeline::<'a, ActivatedShaderStep> {
                    _marker: std::marker::PhantomData::default(),
                    state: GPUState::new(self.state.assets, draw_call.mesh_component),
                };
                ret.state.clear_textures();
                ret.state.bind_mesh();
                Either::Right(ret)
            } else {
                let mut ret = RenderPipeline::<'a, ReadyToDrawStep> {
                    _marker: std::marker::PhantomData::default(),
                    state: GPUState::new(self.state.assets, draw_call.mesh_component),
                };
                ret.state.clear_textures();
                ret.state.bind_shader();
                ret.state.bind_mesh();
                Either::Left(ret)
            }
        } else {
            panic!("Tried to proceed a render pipeline with an empty render queue!");
        }
    }
}
