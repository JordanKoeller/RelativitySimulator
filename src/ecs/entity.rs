use specs::prelude::*;
use specs::world::LazyBuilder;

use events::{EventWithPayload, ReceiverID};

/////////////////////////////////////////////
// CRUD EVENTS
/////////////////////////////////////////////

#[derive(Clone, Eq, Debug)]
pub enum EntityCrudEvent {
  Create,
  Update(Entity),
  Delete(Entity),
  NOOP,
}

impl EntityCrudEvent {
  fn index(&self) -> u32 {
    match self {
      EntityCrudEvent::Create => 0,
      EntityCrudEvent::Update(_) => 1,
      EntityCrudEvent::Delete(_) => 2,
      EntityCrudEvent::NOOP => 3,
    }
  }
}

impl std::hash::Hash for EntityCrudEvent {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.index().hash(state)
  }
}

impl PartialEq for EntityCrudEvent {
  fn eq(&self, other: &Self) -> bool {
    self.index() == other.index()
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

// pub enum BuilderSpawner<'a> {
//   Builder(EntityBuilder<'a>),
//   LazyBuilder(Entity, &'a LazyUpdate),
// }

// impl<'a> BuilderSpawner<'a> {
//   fn add<C: Component + Send + Sync>(self, component: C) -> Self {
//     match self {
//       BuilderSpawner::Builder(builder) => BuilderSpawner::Builder(builder.with(component)),
//       BuilderSpawner::LazyBuilder(ent, updater) => {
//         updater.insert(ent, component);
//         self
//       }
//     }
//   }

//   fn build(self) -> Entity {
//     match self {
//       BuilderSpawner::Builder(builder) => builder.build(),
//       BuilderSpawner::LazyBuilder(ent, updater) => {
//         ent
//       }
//     }
//   }
// }

// pub trait EntityConstructor {
//   fn new_entity<'a>(self) -> (BuilderSpawner<'a>, Self);
//   // fn add<C: Component + Send + Sync>(self, component: C) -> Self;
//   // fn build(&mut self) -> Entity;
// }

// pub struct LazyUpdater<'a, 'b> {
//   entity: Option<Entity>,
//   entities: &'b Entities<'a>,
//   updater: &'b LazyUpdate,
// }
// impl<'a, 'b> LazyUpdater<'a, 'b> {
//     pub fn new(entities: &'b Entities<'a>, updater: &'b LazyUpdate) -> Self {
//     Self {
//       entity: None,
//       entities: entities,
//       updater,
//     }
//   }
// }
// impl<'a, 'b> EntityConstructor for LazyUpdater<'a, 'b> {
//   fn new_entity<'c>(self) -> (BuilderSpawner<'c>, Self) {
//     (
//       BuilderSpawner::LazyBuilder(self.entities.create(), self.updater),
//       self
//     )
//   }
//   // fn add<C: Component + Send + Sync>(self, component: C) -> Self {
//   //   self.updater.insert(self.entity.unwrap(), component);
//   //   self
//   // }
//   // fn build(&mut self) -> Entity {
//   //   self.entity.unwrap().clone()
//   // }
//   // fn new_entity(self) -> Self {
//   //   LazyUpdater::new(self.entities, self.updater)
//   // }
// }
// pub struct WorldEntityCreator<'a> {
//   world: &'a mut World,
//   builder: Option<EntityBuilder<'a>>
// }
// impl<'a> EntityConstructor<'a> for WorldEntityCreator<'a> {
//   fn add<C: Component + Send + Sync>(self, component: C) -> Self {
//     WorldEntityCreator {
//       world: self.world,
//       builder: Some(self.builder.unwrap().with(component))
//     }
//   }
//   fn build(&mut self) -> Entity {
//     let mut ret: Option<Entity> = None;
//     self.builder.map(|builder| {
//       ret = Some(builder.build());
//     });
//     self.builder = None;
//     ret.unwrap()
//     // let builder = self.builder;
//     // self.builder = None;
//     // builder.as_ref().build()

//     // let newEnt = self.builder.clone()
//     // self.builder.unwrap().build()
//     // self.builder.as_ref().unwrap().build();
//     // self.builder.unwrap().build();
//   }
//   fn new_entity(self) -> Self {
//     WorldEntityCreator::new(self.world)
//     // let myBuilder = self.world.create_entity() as EntityBuilder<'a>;
//     // self.builder = Some();
//   }
// }
// impl<'a> WorldEntityCreator<'a> {
//   pub fn new(world: &'a mut World) -> Self {
//     let builder: EntityBuilder<'a> = world.create_entity();
//     Self {
//       world,
//       builder: Some(builder),
//     }
//   }
// }

// pub struct EntityConstructorFactory;

// impl EntityConstructorFactory {
//   pub fn from_builder<'a>(world: &'a mut World) -> WorldEntityCreator<'a> {
//     WorldEntityCreator::new(world)
//   }

//   pub fn from_updater<'a, 'b>(entities: &'b Entities<'a>, updater: &'b LazyUpdate) -> LazyUpdater<'a, 'b> {
//     LazyUpdater::new(entities, updater)
//   }
// }

pub enum MyBuilder<'a, 'b> {
  Builder(EntityBuilder<'a>),
  LazyBuilder(LazyBuilder<'b>),
}

impl <'a, 'b> MyBuilder<'a, 'b> {
  pub fn with<C: Component + Send + Sync>(self, component: C) -> Self {
    match self {
      MyBuilder::Builder(b) => MyBuilder::Builder(b.with(component)),
      MyBuilder::LazyBuilder(lb) => MyBuilder::LazyBuilder(lb.with(component))
    }
  }

  pub fn build(self) -> Entity {
    match self {
      MyBuilder::Builder(b) => b.build(),
      MyBuilder::LazyBuilder(lb) => lb.build()
    }
  }
}

// /////////////////////////////////////////////
// // Entity Delegate for doing crud things on entities
// /////////////////////////////////////////////

pub trait EntityDelegate<'a> {
  type State: Sync + Send + Clone + std::fmt::Debug + Default + 'static;
  type EntityResources: SystemData<'a>;

  fn create<'b, F: Fn() -> MyBuilder<'a, 'b>>(&self, state: Self::State, resources: &mut Self::EntityResources, constructor: F) -> Vec<Entity>;

  fn update_entity(&self, entity: Entity, state: Self::State);
}

/////////////////////////////////////////////
// Entity manager system definition
/////////////////////////////////////////////

struct EntityManager<'a, Delegate: EntityDelegate<'a>> {
  delegate: Delegate,
  event_receiver_id: ReceiverID,
  marker: std::marker::PhantomData<&'a Delegate>,
}

impl<'a, Delegate: EntityDelegate<'a>> EntityManager<'a, Delegate> {
  fn new(delegate: Delegate) -> Self {
    Self {
      delegate,
      event_receiver_id: usize::MAX,
      marker: std::marker::PhantomData::default(),
    }
  }
}

impl<'a, Delegate: EntityDelegate<'a>> System<'a> for EntityManager<'a, Delegate> {
  type SystemData = (
    Delegate::EntityResources,
    Entities<'a>,
    Read<'a, LazyUpdate>,
    Write<'a, EventWithPayload<EntityCrudEvent, Delegate::State>>,
  );

  fn run(&mut self, (mut resource_storage, entities, updater, mut crud_events): Self::SystemData) {
    crud_events
    .read(&self.event_receiver_id)
    .for_each(|(event, payload)| {
      match event {
        EntityCrudEvent::NOOP => {}
        EntityCrudEvent::Create => {
            // let constructor = EntityConstructorFactory::from_updater(&entities, &updater);
            self.delegate.create(payload.clone(), &mut resource_storage, || {
              MyBuilder::LazyBuilder(updater.create_entity(&entities))
            });
          }
          EntityCrudEvent::Delete(id) => {
            entities.delete(*id);
          }
          EntityCrudEvent::Update(id) => {
            self.delegate.update_entity(*id, payload.clone());
          }
        };
      });
  }

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    let mut channel = world.fetch_mut::<EventWithPayload<EntityCrudEvent, Delegate::State>>();
    self.event_receiver_id = channel.register_with_subs(&[EntityCrudEvent::Create]);
  }
}
