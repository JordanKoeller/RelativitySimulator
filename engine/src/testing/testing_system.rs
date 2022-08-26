use specs::prelude::*;

pub struct TestingEcs<'a, 'b> {
  world: World,
  dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> TestingEcs<'a, 'b> {
  pub fn run(&mut self) {
    self.dispatcher.run_now(&self.world);
    self.world.maintain();
  }

  pub fn entities(&self) -> Vec<Entity> {
    self.world.entities().join().collect()
  }
}

pub struct TestingEcsBuilder<'a, 'b> {
  dispatcher_builder: DispatcherBuilder<'a, 'b>,
  world: World,
}

impl<'a, 'b> TestingEcsBuilder<'a, 'b> {
  pub fn new() -> Self {
    Self {
      world: World::new(),
      dispatcher_builder: DispatcherBuilder::new(),
    }
  }

  pub fn with_system<T>(mut self, system: T) -> Self
  where
    T: for<'c> RunNow<'c> + 'b,
  {
    self.dispatcher_builder.add_thread_local(system);
    self
  }

  pub fn with_resource<R: Resource>(mut self, r: R) -> Self {
    self.world.insert(r);
    self
  }

  pub fn register_component<C: Component>(mut self) -> Self
  where
    C::Storage: Default,
  {
    self.world.register::<C>();
    self
  }

  pub fn with_entity<F>(mut self, f: F) -> Self
  where
    F: FnOnce(EntityBuilder) -> Entity,
  {
    let entity_builder = self.world.create_entity();
    f(entity_builder);
    self
  }

  pub fn build(mut self) -> TestingEcs<'a, 'b> {
    let mut dispatcher = self.dispatcher_builder.build();
    dispatcher.setup(&mut self.world);
    TestingEcs {
      world: self.world,
      dispatcher,
    }
  }
}

pub struct TestingSystem<'a, T, D, F>
where
  D: SystemData<'a>,
  F: Fn(&mut T, D) -> ()
{
  pub state: T,
  closure: F,
  phantom: std::marker::PhantomData<&'a D>
}

impl<'a, T, D, F> System<'a> for TestingSystem<'a, T, D, F>
where 
  D: SystemData<'a>,
  F: Fn(&mut T, D) -> (),
{
  type SystemData = D;

  fn run(&mut self, data: Self::SystemData) {
    (self.closure)(&mut self.state, data);
  }
}

impl<'a, T, D, F> TestingSystem<'a, T, D, F>
where 
  D: SystemData<'a>,
  F: Fn(&mut T, D) -> (),
{
  pub fn new(state: T, closure: F) -> Self {
    Self {
      state,
      closure,
      phantom: std::marker::PhantomData::default(),
    }
  }
}