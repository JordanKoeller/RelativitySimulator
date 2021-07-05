use specs::Entity;
use renderer::platform::{VertexArray, ShaderId, InstancingTable, AttributeType, DataBuffer, BufferLayout};
use ecs::{DrawableId, Material};
use utils::Mat4F;
use cgmath::Matrix;


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
      vao, shader_name, registry: None, instance_table: None,
    }
  }

  pub fn new_instanced(mut vao: VertexArray, shader_name: String, attributes: Vec<(String, AttributeType)>, num_instances: u32) -> Self {
    let vbo = DataBuffer::instancing_buffer(BufferLayout::from(&attributes), num_instances);
    let table = InstancingTable::new(attributes);
    vao.add_instancing_buffer(vbo, false);
    Self {
      vao, shader_name, registry: None, instance_table: Some(table)
    }
  }


  pub fn refresh(&mut self) {
    self.vao.refresh();
  }

  pub fn upsert_instance(&mut self, entity: &Entity, transform: &Mat4F, material: &mut Material) {
    if let Some(table) = &mut self.instance_table {
      let mut collector: Vec<f32> = Vec::with_capacity(table.stride());
      let transform_sz = AttributeType::Mat4.width() as usize;
      let transform_ptr = unsafe {
        let ptr = transform.as_ptr();
        std::slice::from_raw_parts(ptr, transform_sz)
      };
      for i in 0..transform_sz {
          collector[i] = transform_ptr[i];
      }
      let offset = table.upsert_instance(entity);
      material.serialize_into(&mut collector, &table.attribute_offsets);
      self.vao.instancing_buffer.as_mut().unwrap().splice_inplace(offset, collector.len(), move |slc| {
        for i in 0..collector.len() {
          slc[i] = collector[i];
        }
      })
    }

  }

}