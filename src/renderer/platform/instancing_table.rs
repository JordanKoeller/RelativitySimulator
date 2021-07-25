use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::num::NonZeroI32;

use specs::prelude::*;

use ecs::{DrawableId, Material};
use renderer::platform::{AttributeType, DataBuffer, ShaderId, VertexArray};

#[derive(Debug, Clone)]
pub struct InstancingTable {
  pub attribute_offsets: Vec<(String, AttributeType)>, // map from uniform name to its offset in an element
  stride_cache: usize,
  num_instances: usize,
  instances_table: BTreeMap<Entity, usize>,
  holes: VecDeque<usize>,
}

impl InstancingTable {
  pub fn new(offsets: Vec<(String, AttributeType)>) -> Self {
    let stride_cache = offsets.iter().map(|x| x.1.width()).sum::<u32>() as usize;
    Self {
      attribute_offsets: offsets,
      instances_table: BTreeMap::default(),
      stride_cache,
      num_instances: 0,
      holes: VecDeque::new(),
    }
  }

  pub fn stride(&self) -> usize {
    self.stride_cache
  }

  pub fn upsert_instance(&mut self, entity: &Entity) -> usize {
    if let Some(table_ent) = self.instances_table.get(&entity) {
      self.calc_offset(table_ent)
    } else {
      let entry_pt = self.get_entry_location();
      self.instances_table.insert(entity.clone(), entry_pt);
      if entry_pt > self.num_instances {
        self.num_instances = entry_pt;
      }
      self.calc_offset(&entry_pt)
    }
  }

  pub fn remove_instance(&mut self, entity: &Entity) -> usize {
    if let Some(table_entry) = self.instances_table.remove(&entity) {
      self.holes.push_back(table_entry);
      self.calc_offset(&table_entry)
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
    self.num_instances + 1
  }

  pub fn len(&self) -> usize {
    self.instances_table.len() - self.holes.len()
  }

  fn calc_offset(&self, ind: &usize) -> usize {
    ind * self.stride_cache
  }
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  fn _create_test_table() -> (InstancingTable, [Entity; 4], World) {
    let table = InstancingTable::new(vec![
      ("model".to_string(), AttributeType::Mat4),
      ("diffuse_index".to_string(), AttributeType::Int),
      ("diffuse".to_string(), AttributeType::Float3),
    ]);
    let mut world = World::new();
    let entities: [Entity; 4] = [
      world.create_entity().build(),
      world.create_entity().build(),
      world.create_entity().build(),
      world.create_entity().build(),
    ];
    (table, entities, world)
  }

  #[test]
  fn test_constructs_instancing_table() {
    InstancingTable::new(vec![
      ("model".to_string(), AttributeType::Mat4),
      ("diffuse_index".to_string(), AttributeType::Int),
      ("diffuse".to_string(), AttributeType::Float3),
    ]);
  }

  #[test]
  fn test_calcs_stride() {
    let table = InstancingTable::new(vec![
      ("model".to_string(), AttributeType::Mat4),
      ("diffuse_index".to_string(), AttributeType::Int),
      ("diffuse".to_string(), AttributeType::Float3),
    ]);
    assert_eq!(table.stride(), 20);
  }

  #[test]
  fn test_push_entity() {
    let (mut table, entities, _) = _create_test_table();
    let std = table.stride();
    entities.iter()
      .map(|e| table.upsert_instance(e))
      .zip(entities)
      .enumerate()
      .for_each(|(ind, (pos, e))| {
        assert_eq!(ind * std, pos);
      });
  }

  #[test]
  fn test_push_does_not_duplicate() {
    let (mut table, entities, _) = _create_test_table();
    let std = table.stride();
    entities.iter()
      .map(|e| table.upsert_instance(e))
      .zip(entities)
      .enumerate()
      .for_each(|(ind, (pos, e))| {
        assert_eq!(ind * std, pos);
      });
    entities.iter()
    .map(|e| table.upsert_instance(e))
    .zip(entities)
    .enumerate()
    .for_each(|(ind, (pos, e))| {
      assert_eq!(ind * std, pos);
    });
  }


  #[test]
  fn test_pop_entity_returns_offset_position() {
    let (mut table, entities, _) = _create_test_table();
    entities.iter().for_each(|e| {table.upsert_instance(e);});
    let offset = table.upsert_instance(&entities[0]);
    assert_eq!(offset, table.remove_instance(&entities[0]));
  }

  #[test]
  fn test_popping_entity_frees_position_for_next_insertion() {
    let (mut table, entities, mut world) = _create_test_table();
    entities.iter().for_each(|e| {table.upsert_instance(e);});
    let offset = table.remove_instance(&entities[0]);
    let next_e = world.create_entity().build();
    let next_o = table.upsert_instance(&next_e);
    assert_eq!(next_o, offset);
  }

}
