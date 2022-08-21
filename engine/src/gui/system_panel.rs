use specs::prelude::*;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::{ControlPanel, ControlPanelBuilder, ControlPanels};
use crate::datastructures::{GenericRegistry, Registry, RegistryItem};
use crate::ecs::{MonoBehavior, SystemUtilities, WorldProxy};

pub trait SystemDebugger<'a>: MonoBehavior<'a> + 'static {
  fn create_panel(&self) -> ControlPanelBuilder;

  fn get_panel<'b: 'a>(&self, utilities: &'b SystemUtilities<'a>) -> RwLockReadGuard<'b, ControlPanel> {
    let type_id = TypeId::of::<Self>();
    utilities
      .control_panel(type_id)
      .map(|cp| cp.read().unwrap())
      .expect("Could not find Control panel for system")
  }

  fn get_write_panel<'b: 'a>(&self, utilities: &'b SystemUtilities<'a>) -> RwLockWriteGuard<'b, ControlPanel> {
    let type_id = TypeId::of::<Self>();
    utilities
      .control_panel(type_id)
      .map(|cp| cp.write().unwrap())
      .expect("Could not find Control panel for system")
  }

  fn register_debugger(&self, world: &WorldProxy) {
    let type_id = TypeId::of::<Self>();
    let mut panels = world.write_resource::<ControlPanels>();
    panels.add(type_id, self.create_panel());
  }
}
