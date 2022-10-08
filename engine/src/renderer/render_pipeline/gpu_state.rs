use specs::Entity;
use std::borrow::{Borrow, BorrowMut};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::graphics::{
  AssetLibrary, Assets, MaterialComponent, MeshComponent, Shader, ShaderBuilder, ShaderId, TextureBinder, VertexArray,
  VertexArrayBuilder, VertexArrayId,
};
use crate::renderer::RenderQueueConsumer;
use crate::utils::Mat4F;

pub struct GPUState<'a> {
  pub assets: &'a mut AssetLibrary,
  pub textures: TextureBinder,
  pub active_mesh: VertexArrayId,
  pub active_shader: ShaderId,
  pub poly_count: usize,
}

impl<'a> GPUState<'a> {
  pub fn new(assets: &'a mut AssetLibrary, active_mesh: VertexArrayId, active_shader: ShaderId) -> Self {
    let ret = Self {
      assets,
      textures: TextureBinder::new(32), // TODO: Query GPU for how many textures it can have bound at once
      active_mesh,
      active_shader,
      poly_count: 0usize,
    };
    ret.shader_immut().bind();
    ret.active_mesh.bind();
    ret
  }

  pub fn shader(&mut self) -> RwLockWriteGuard<'_, Shader> {
    <AssetLibrary as Assets<ShaderBuilder>>::get_asset_mut(&self.assets, &mut self.active_shader).unwrap()
    // self.assets.get_asset_mut(&mut self.active_shader).unwrap()
  }

  pub fn shader_immut(&self) -> RwLockReadGuard<'_, Shader> {
    <AssetLibrary as Assets<ShaderBuilder>>::get_asset(&self.assets, &self.active_shader).unwrap()
  }

  pub fn bind_shader(&mut self, shader_id: ShaderId) {
    self.active_shader = shader_id;
    self.shader_immut().bind();
  }

  pub fn bind_mesh(&mut self, vai: VertexArrayId) {
    self.active_mesh = vai;
    self.active_mesh.bind();
  }

  pub fn draw(&self) {
    let element_type = self.shader_element_type();
    let vao_opt = <AssetLibrary as Assets<VertexArrayBuilder>>::get_asset(&self.assets, &self.active_mesh);
    vao_opt.map(|vao| vao.draw(&element_type));
  }

  pub fn increment_poly_counter(&mut self) {
    <AssetLibrary as Assets<VertexArrayBuilder>>::get_asset(&self.assets, &self.active_mesh).map(|vao| {
      self.poly_count += vao.poly_count();
    });
  }

  // pub fn upsert_instance(&mut self, entity: &Entity, transform: &Mat4F, material: &Material) {
  //     self.assets
  //         .upsert_instance_data(entity, transform, material, &mut self.textures);
  // }

  pub fn bind_material(&mut self, mtl: &MaterialComponent) {
    let assets = &self.assets;
    <AssetLibrary as Assets<ShaderBuilder>>::get_asset(assets, &self.active_shader).map(|shader| {
      mtl.bind_to(shader.borrow(), &mut self.textures, false);
    });
  }

  pub fn clear_textures(&mut self) {
    self.unbind_textures();
    self.textures.refresh();
  }

  pub fn unbind_textures(&self) {
    for texture_slot in self.textures.bound_slots() {
      self.shader_immut().unbind_texture_slot(*texture_slot);
    }
  }

  fn shader_element_type(&self) -> gl::types::GLenum {
    self.shader_immut().element_type().clone()
  }
}
