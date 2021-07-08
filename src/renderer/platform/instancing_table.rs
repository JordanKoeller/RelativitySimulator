use std::collections::HashMap;
use std::collections::VecDeque;

use specs::Entity;

use renderer::platform::{VertexArray, ShaderId, DataBuffer, AttributeType};
use ecs::{DrawableId, Material};


#[derive(Debug, Clone)]
pub struct InstancingTable {
  pub attribute_offsets: Vec<(String, AttributeType)>, // map from uniform name to its offset in an element
  stride_cache: usize,
  instances_table: HashMap<Entity, usize>,
  holes: VecDeque<usize>,
}

impl InstancingTable {
  pub fn new(offsets: Vec<(String, AttributeType)>) -> Self {
    let stride_cache = offsets.iter().map(|x| x.1.width()).sum::<u32>() as usize;
    Self {
      attribute_offsets: offsets,
      instances_table: HashMap::default(),
      stride_cache,
      holes: VecDeque::new(),
    }
  }

  pub fn stride(&self) -> usize {
    self.stride_cache
  }

  pub fn upsert_instance(&mut self, entity: &Entity) -> usize {
    if let Some(table_ent) = self.instances_table.get(entity) {
      self.calc_offset(table_ent)
    } else {
      let entry_pt = self.get_entry_location();
      self.instances_table.insert(entity.clone(), entry_pt);
      self.calc_offset(&entry_pt)
    }
  }

  pub fn remove_instance(&mut self, entity: &Entity) -> usize {
    let entry_opt = self.instances_table.get(entity).map(|x| x.clone());
    if let Some(table_entry) = entry_opt {
      self.holes.push_back(table_entry);
      self.instances_table.remove(entity);
      table_entry
    } else {
      panic!("Tried to remove an entity that does not exist in this InstancingTable!");
    }
  }

  fn get_entry_location(&mut self) -> usize {
    if let Some(loc) = self.holes.pop_front() {
      loc
    } else {
      self.instances_table.len()
    }
  }
  
  pub fn num_instances(&self) -> usize {
    self.instances_table.len()
  }

  pub fn len(&self) -> usize {
    self.instances_table.len() - self.holes.len()
  }

  fn calc_offset(&self, ind: &usize) -> usize {
    ind * self.stride_cache
  }

}