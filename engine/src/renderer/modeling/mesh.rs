use crate::ecs::{DrawableId, Material};
use crate::renderer::platform::{
    AttributeType, BufferLayout, DataBuffer, InstancingTable, Shader, ShaderId, TextureBinder, VertexArray,
};
use crate::utils::Mat4F;
use cgmath::Matrix;
use specs::Entity;
use std::ffi::c_void;

use crate::debug::*;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vao: VertexArray,
    pub shader_name: String,
    pub registry: Option<DrawableId>,
    pub instance_table: Option<InstancingTable>,
}

impl Mesh {
    pub fn new(vao: VertexArray, shader_name: String) -> Self {
        Self {
            vao,
            shader_name,
            registry: None,
            instance_table: None,
        }
    }

    pub fn new_instanced(
        mut vao: VertexArray,
        shader_name: String,
        attributes: Vec<(String, AttributeType)>,
        num_instances: u32,
    ) -> Self {
        let vbo = DataBuffer::instancing_buffer(BufferLayout::from(&attributes), num_instances);
        let table = InstancingTable::new(attributes);
        vao.add_instancing_buffer(vbo, false);
        Self {
            vao,
            shader_name,
            registry: None,
            instance_table: Some(table),
        }
    }

    pub fn instanced(&self) -> bool {
        self.instance_table.is_some()
    }

    pub fn draw(&self, elem_type: &gl::types::GLenum) {
        if self.instanced() {
            let num_instances = self.instance_table.as_ref().unwrap().num_instances();
            self.vao.draw_instanced(elem_type, num_instances);
        } else {
            self.vao.draw(elem_type);
        }
    }

    pub fn refresh(&mut self) {
        self.vao.refresh();
    }

    pub fn upsert_instance(
        &mut self,
        entity: &Entity,
        transform: &Mat4F,
        material: &Material,
        texture_binder: &mut TextureBinder,
        shader: &Shader,
    ) {
        if let Some(table) = &mut self.instance_table {
            let mut collector: Vec<f32> = (0..table.stride()).into_iter().map(|_| 0f32).collect();
            let len = collector.len();
            let transform_sz = AttributeType::Mat4.width() as usize;
            let transform_ptr = unsafe {
                let ptr = transform.as_ptr();
                std::slice::from_raw_parts(ptr, transform_sz)
            };
            for i in 0..transform_sz {
                collector[i] = transform_ptr[i];
            }
            let offset = table.upsert_instance(entity);
            material.serialize_into(
                &mut collector[transform_sz..len],
                &table.attribute_offsets,
                texture_binder,
                shader,
            );
            self.vao
                .instancing_buffer
                .as_mut()
                .unwrap()
                .splice_inplace(offset, offset + collector.len(), move |slc| {
                    for i in 0..collector.len() {
                        slc[i] = collector[i];
                    }
                })
        }
    }

    pub fn clear_instance(&mut self, entity: &Entity) {
        if let Some(table) = &mut self.instance_table {
            let offset = table.remove_instance(entity);
            let stride = table.stride();
            self.vao
                .instancing_buffer
                .as_mut()
                .unwrap()
                .splice_inplace(offset, offset + stride, move |s| {
                    for i in 0..s.len() {
                        s[i] = 1f32;
                    }
                });
        }
    }

    pub fn unbind(&self) {
        self.vao.unbind();
    }
}
