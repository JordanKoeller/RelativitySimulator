use specs::prelude::*;
use specs::world::LazyBuilder;



use events::{EventChannel, ReceiverID, StatefulEventChannel};

use renderer::modeling::Drawable;

/////////////////////////////////////////////
// CRUD EVENTS
/////////////////////////////////////////////

#[derive(Clone, Eq, Debug)]
pub enum EntityCrudEvent {
  Create,
  Update(Entity),
  NOOP,
}

impl EntityCrudEvent {
  fn index(&self) -> u32 {
    match self {
      EntityCrudEvent::Create => 0,
      EntityCrudEvent::NOOP => 1,
      EntityCrudEvent::Update(_) => 2,
    }
  }
}

impl std::hash::Hash for EntityCrudEvent {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      EntityCrudEvent::Create => 0.hash(state),
      EntityCrudEvent::NOOP => 1.hash(state),
      EntityCrudEvent::Update(e) => {
        2.hash(state);
        e.hash(state);
      },
    }
    // self.index().hash(state)
  }
}

impl PartialEq for EntityCrudEvent {
  fn eq(&self, other: &Self) -> bool {
    if self.index() != 2 {
      self.index() == other.index()
    } else {
      if let EntityCrudEvent::Update(e1) = self {
        if let EntityCrudEvent::Update(e2) = other {
          e1 == e2
        } else {
          false
        }
      } else {
        false
      }
    }
  }
}

impl Default for EntityCrudEvent {
  fn default() -> Self {
    Self::NOOP
  }
}

/////////////////////////////////////////////
// Entity Constructor
/////////////////////////////////////////////

pub enum MyBuilder<'a, 'b> {
  Builder(EntityBuilder<'a>),
  LazyBuilder(LazyBuilder<'b>),
}

impl<'a, 'b> MyBuilder<'a, 'b> {
  pub fn with<C: Component + Send + Sync>(self, component: C) -> Self {
    match self {
      MyBuilder::Builder(b) => MyBuilder::Builder(b.with(component)),
      MyBuilder::LazyBuilder(lb) => MyBuilder::LazyBuilder(lb.with(component)),
    }
  }

  pub fn with_drawable<D: Drawable>(self, d: &D) -> Self {
    self.with(d.material()).with(d.mesh_component())
  }

  pub fn build(self) -> Entity {
    match self {
      MyBuilder::Builder(b) => b.build(),
      MyBuilder::LazyBuilder(lb) => lb.build(),
    }
  }
}

// /////////////////////////////////////////////
// // Entity Delegate for doing crud things on entities
// /////////////////////////////////////////////

pub trait EntityDelegate<'a> {
  type State: Sync + Send + Clone + std::fmt::Debug + Default + 'static;
  type EntityResources: SystemData<'a>;

  fn create<'b, F: Fn() -> MyBuilder<'a, 'b>>(
    &self,
    state: &Self::State,
    resources: &mut Self::EntityResources,
    constructor: F,
  ) -> Vec<Entity>;

  fn update(&self, state: &Self::State, resources: &mut Self::EntityResources, entity_id: &Entity) {
    panic!("EntityDelegate::create not implemented for {}", std::any::type_name::<Self>());
  }

  fn setup_delegate(&mut self, world: &mut World) {}

}

/////////////////////////////////////////////
// Entity manager system definition
/////////////////////////////////////////////

pub struct EntityManager<Delegate>
where
  for<'a> Delegate: EntityDelegate<'a> + Default,
{
  delegate: Delegate,
  event_receiver_id: ReceiverID,
}

impl<Delegate> EntityManager<Delegate>
where
  for<'a> Delegate: EntityDelegate<'a> + Default,
{
  fn new(delegate: Delegate) -> Self {
    Self {
      delegate,
      event_receiver_id: usize::MAX,
    }
  }
}

impl<Delegate> Default for EntityManager<Delegate>
where
  for<'a> Delegate: EntityDelegate<'a> + Default,
{
  fn default() -> Self {
    Self {
      delegate: Delegate::default(),
      event_receiver_id: usize::MAX,
    }
  }
}

impl<'a, Delegate> System<'a> for EntityManager<Delegate>
where
  for<'b> Delegate: EntityDelegate<'b> + Default,
{
  type SystemData = (
    <Delegate as EntityDelegate<'a>>::EntityResources,
    Entities<'a>,
    Read<'a, LazyUpdate>,
    Write<'a, StatefulEventChannel<EntityCrudEvent, <Delegate as EntityDelegate<'a>>::State>>,
  );

  fn run(&mut self, (mut resource_storage, entities, updater, mut crud_events): Self::SystemData) {
    crud_events.for_each(&self.event_receiver_id, |ep| {
      match ep.0 {
        EntityCrudEvent::NOOP => {},
        EntityCrudEvent::Create => {
          self.delegate.create(&ep.1, &mut resource_storage, || {
            MyBuilder::LazyBuilder(updater.create_entity(&entities))
          });
        },
        EntityCrudEvent::Update(updating_entity) => {
          self.delegate.update(&ep.1, &mut resource_storage, &updating_entity);
        }
      };
    });
    crud_events.clear_events();
  }

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.delegate.setup_delegate(world);
    let mut channel = world.fetch_mut::<StatefulEventChannel<EntityCrudEvent, Delegate::State>>();
    self.event_receiver_id = channel.register_with_subs(&[
      EntityCrudEvent::Create
    ]);
  }
}
