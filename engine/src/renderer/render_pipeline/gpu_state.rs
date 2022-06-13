use specs::Entity;

use crate::graphics::{AssetLibrary, MaterialComponent, MeshComponent, Shader, ShaderId, TextureBinder, VertexArrayId};
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

    pub fn shader(&mut self) -> &mut Shader {
        self.assets.get_shader_mut(&self.active_shader).unwrap()
    }

    pub fn shader_immut(&self) -> &Shader {
        self.assets.get_shader(&self.active_shader).unwrap()
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
        let vao_opt = self.assets.get_vertex_array(&self.active_mesh);
        vao_opt.map(|vao| vao.draw(&element_type));
    }

    pub fn increment_poly_counter(&mut self) {
        self.assets.get_vertex_array(&self.active_mesh).map(|vao| {
            self.poly_count += vao.poly_count();
        });
    }

    // pub fn upsert_instance(&mut self, entity: &Entity, transform: &Mat4F, material: &Material) {
    //     self.assets
    //         .upsert_instance_data(entity, transform, material, &mut self.textures);
    // }

    pub fn bind_material(&mut self, mtl: &MaterialComponent) {
        self.assets.get_shader(&self.active_shader).map(|shader| {
            mtl.bind_to(shader, &mut self.textures, false);
        });
    }

    pub fn clear_textures(&mut self) {
        self.textures.refresh();
    }

    fn shader_element_type(&self) -> gl::types::GLenum {
        self.shader_immut().element_type().clone()
    }
}
