use specs::prelude::*;

use gui::DebugPanel;

pub trait SystemDelegate<'a> {
  type SystemData: specs::SystemData<'a>;

  fn run(&mut self, resources: Self::SystemData);
  
  fn setup(&mut self, _world: &mut World) {}

  fn update_debugger(&mut self, _resources: &mut Self::SystemData, _debugger: &mut DebugPanel) {}

  fn setup_debug_panel(&mut self, _world: &mut World) -> Option<DebugPanel> {None}
}

pub struct SystemManager<Delegate>
where for <'a> Delegate: SystemDelegate<'a> {
  entity: Option<Entity>,
  delegate: Delegate,
}

impl<'a, Delegate> System<'a> for SystemManager<Delegate>
where for <'b> Delegate: SystemDelegate<'b> {
  type SystemData = (
    <Delegate as SystemDelegate<'a>>::SystemData,
    WriteStorage<'a, DebugPanel>,
    Entities<'a>
  );

  fn run(&mut self, (mut delegate_data, mut gui, _entities): Self::SystemData) {
    if let Some(entity) = self.entity {
      if let Some(mut ui_panel) = gui.get_mut(entity) {
        self.delegate.update_debugger(&mut delegate_data, &mut ui_panel);
      }
    }
    self.delegate.run(delegate_data);
  }

  fn setup(&mut self, world: &mut World) {
    world.register::<DebugPanel>();
    self.delegate.setup(world);
    if let Some(ui_panel) = self.delegate.setup_debug_panel(world) {
      let ett = world.create_entity().with(ui_panel).build();
      self.entity = Some(ett);
    }
  }
}

impl<Delegate> Default for SystemManager<Delegate>
where for <'a> Delegate: SystemDelegate<'a> + Default {
  fn default() -> Self {
    Self {
      entity: None,
      delegate: Delegate::default(),
    }
  }
}

impl <Delegate> SystemManager<Delegate>
where for <'a> Delegate: SystemDelegate<'a> {
  pub fn new(delegate: Delegate) -> Self {
    Self {
      entity: None,
      delegate,
    }
  }
}

