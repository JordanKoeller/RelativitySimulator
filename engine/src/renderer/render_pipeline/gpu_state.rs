use specs::Entity;

use crate::graphics::{AssetLibrary, MaterialComponent, MeshComponent, Shader, ShaderId, TextureBinder};
use crate::renderer::RenderQueueConsumer;
use crate::utils::Mat4F;

pub struct GPUState<'a> {
    pub assets: &'a mut AssetLibrary,
    pub textures: TextureBinder,
    pub active_mesh: MeshComponent,
}

impl<'a> GPUState<'a> {
    pub fn new(assets: &'a mut AssetLibrary, active_mesh: MeshComponent) -> Self {
        Self {
            assets,
            textures: TextureBinder::new(3),
            active_mesh,
        }
    }

    pub fn mesh(&mut self) -> &mut MeshComponent {
        &mut self.active_mesh
    }

    pub fn shader(&mut self) -> &mut Shader {
        self.assets.get_shader_mut(&self.active_mesh.shader_id).unwrap()
    }

    pub fn shader_immut(&mut self) -> &Shader {
        self.assets.get_shader(&self.active_mesh.shader_id).unwrap()
    }

    pub fn bind_shader(&mut self) {
        self.shader().bind();
        self.mesh().vertex_array_id.bind();
    }

    pub fn bind_mesh(&mut self) {
        self.shader().bind();
        self.mesh().vertex_array_id.bind();
    }

    pub fn draw(&mut self) {
        let shader = self.assets.get_shader(&self.active_mesh.shader_id).unwrap();
        let vertex_array = self.assets.get_vertex_array(&self.active_mesh.vertex_array_id);
        vertex_array.unwrap().draw(shader.element_type());
    }

    // pub fn upsert_instance(&mut self, entity: &Entity, transform: &Mat4F, material: &Material) {
    //     self.assets
    //         .upsert_instance_data(entity, transform, material, &mut self.textures);
    // }

    pub fn bind_material(&mut self, mtl: &MaterialComponent) {
        let shader = self.assets.get_shader(&self.active_mesh.shader_id).unwrap();
        mtl.bind_to(shader, &mut self.textures);
    }

    pub fn clear_textures(&mut self) {
        self.textures.refresh();
    }
}
