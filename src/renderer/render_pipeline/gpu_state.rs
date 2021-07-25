use specs::Entity;
use renderer::{
  TextureBinder, Mesh, Shader, ShaderId,
  RenderQueueConsumer, AssetLibrary
};

use ecs::components::{
  Material, DrawableId,
};

use utils::Mat4F;

pub struct GPUState<'a> {
  pub assets: &'a mut AssetLibrary,
  pub textures: TextureBinder,
  pub id: DrawableId,
}

impl<'a> GPUState<'a> {
  pub fn new(assets: &'a mut AssetLibrary, id: DrawableId) -> Self {
    Self {
      assets,
      textures: TextureBinder::new(3),
      id
    }
  }

  pub fn mesh(&mut self) -> &mut Mesh {
    self.assets.get_active_asset_mut()
  }

  pub fn shader(&mut self) -> &mut Shader {
    self.assets.get_active_shader_mut()
  }

  pub fn mesh_immut(&self) -> &Mesh {
    self.assets.get_active_asset()
  }

  pub fn shader_immut(&self) -> &Shader {
    self.assets.get_active_shader()
  }

  pub fn upsert_instance(
    &mut self,
    entity: &Entity,
    transform: &Mat4F,
    material: &Material,
  ) {
    self.assets.upsert_instance_data(entity, transform, material, &mut self.textures);
  }

  pub fn bind_material(&mut self, mtl: &Material) {
    mtl.bind_to(self.assets.get_active_shader(), &mut self.textures);
  }

  pub fn clear_textures(&mut self) {
    self.textures.refresh();
  }

}

