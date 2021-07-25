extern crate specs;

use events::*;
use specs::prelude::*;

trait Debuggable {
  fn get_events(&self) -> Vec<ImguiUiEvent>;

  fn emit_events(&self) -> Vec<ImguiUiEvent>;

  fn write_events(&mut self, evts: Vec<ImguiUiEvent>);

  // fn get_debug_panel(&self) -> ImguiPanel;
}
