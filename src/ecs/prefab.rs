use specs::prelude::SystemData;
use specs::prelude::*;
use specs::world::LazyBuilder;
use specs::prelude::Read;
use specs::shred::Resource;

use std::ops::Deref;

use events::{EventChannel, ReceiverID, StatefulEventChannel};

use renderer::modeling::Drawable;

/////////////////////////////////////////////
// CRUD EVENTS
/////////////////////////////////////////////

// #[derive(Clone, Eq, Debug)]
// pub enum EntityCrudEvent {
//     Create,
//     Update(Entity),
//     NOOP,
// }

// impl EntityCrudEvent {
//     fn index(&self) -> u32 {
//         match self {
//             EntityCrudEvent::Create => 0,
//             EntityCrudEvent::NOOP => 1,
//             EntityCrudEvent::Update(_) => 2,
//         }
//     }
// }

// impl std::hash::Hash for EntityCrudEvent {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         match self {
//             EntityCrudEvent::Create => 0.hash(state),
//             EntityCrudEvent::NOOP => 1.hash(state),
//             EntityCrudEvent::Update(e) => {
//                 2.hash(state);
//                 e.hash(state);
//             }
//         }
//         // self.index().hash(state)
//     }
// }

// impl PartialEq for EntityCrudEvent {
//     fn eq(&self, other: &Self) -> bool {
//         if self.index() != 2 {
//             self.index() == other.index()
//         } else {
//             if let EntityCrudEvent::Update(e1) = self {
//                 if let EntityCrudEvent::Update(e2) = other {
//                     e1 == e2
//                 } else {
//                     false
//                 }
//             } else {
//                 false
//             }
//         }
//     }
// }

// impl Default for EntityCrudEvent {
//     fn default() -> Self {
//         Self::NOOP
//     }
// }

/////////////////////////////////////////////
// Entity Constructor
/////////////////////////////////////////////

pub enum EntitySpawner<'a, 'b> {
    Builder(EntityBuilder<'a>),
    LazyBuilder(LazyBuilder<'b>),
}

impl<'a, 'b> EntitySpawner<'a, 'b> {

    pub fn new(world: &'a mut World) -> Self {
        Self::Builder(world.create_entity())
    }

    pub fn with<C: Component + Send + Sync>(self, component: C) -> Self {
        match self {
            EntitySpawner::Builder(b) => EntitySpawner::Builder(b.with(component)),
            EntitySpawner::LazyBuilder(lb) => EntitySpawner::LazyBuilder(lb.with(component)),
        }
    }

    pub fn with_drawable<D: Drawable>(self, d: &D) -> Self {
        self.with(d.material()).with(d.mesh_component())
    }

    pub fn build(self) -> Entity {
        match self {
            EntitySpawner::Builder(b) => b.build(),
            EntitySpawner::LazyBuilder(lb) => lb.build(),
        }
    }
}

// /////////////////////////////////////////////
// // Entity Delegate for doing crud things on entities
// /////////////////////////////////////////////

pub trait PrefabBuilder<'a> {
    type State: Sync + Send + Clone + std::fmt::Debug + Default + 'static;
    type EntityResources: SystemData<'a>;

    fn create<'b, F: Fn() -> EntitySpawner<'a, 'b>>(
        &self,
        state: &Self::State,
        resources: &mut Self::EntityResources,
        constructor: F,
    ) -> Vec<Entity>;

    fn update(&self, _state: &Self::State, _resources: &mut Self::EntityResources, _entity_id: &Entity) {
        panic!(
            "PrefabBuilder::create not implemented for {}",
            std::any::type_name::<Self>()
        );
    }

    fn setup_delegate(&mut self, _world: &mut World) {}
}

/////////////////////////////////////////////
// Entity manager system definition
/////////////////////////////////////////////
///
///
#[derive(SystemData)]
pub struct PrefabManager<'a, T: 'a>
where
    T: PrefabBuilder<'a> + Resource,
{
    builder: Write<'a, T>,
    updater: Read<'a, LazyUpdate>,
    entities: Entities<'a>,
    extra: T::EntityResources,
}

impl<'a, T: 'a> PrefabManager<'a, T> where T: PrefabBuilder<'a> + Resource {
    pub fn create(&mut self, state: T::State) -> Entity {
        let ents = &self.entities;
        let updater = &self.updater;
        let ret_vec = self.builder.create(&state, &mut self.extra, || {
            EntitySpawner::LazyBuilder(updater.create_entity(ents))
        });
        ret_vec[0]
    }
}
// pub struct EntityManager<Delegate>
// where
//   for<'a> Delegate: PrefabBuilder<'a> + Default,
// {
//   delegate: Delegate,
//   event_receiver_id: ReceiverID,
// }

// impl<Delegate> EntityManager<Delegate>
// where
//   for<'a> Delegate: PrefabBuilder<'a> + Default,
// {
//   fn new(delegate: Delegate) -> Self {
//     Self {
//       delegate,
//       event_receiver_id: usize::MAX,
//     }
//   }
// }

// impl<Delegate> Default for EntityManager<Delegate>
// where
//   for<'a> Delegate: PrefabBuilder<'a> + Default,
// {
//   fn default() -> Self {
//     Self {
//       delegate: Delegate::default(),
//       event_receiver_id: usize::MAX,
//     }
//   }
// }

// impl<'a, Delegate> System<'a> for EntityManager<Delegate>
// where
//     for<'b> Delegate: PrefabBuilder<'b> + Default,
// {
//     type SystemData = (
//         <Delegate as PrefabBuilder<'a>>::EntityResources,
//         Entities<'a>,
//         Read<'a, LazyUpdate>,
//         Write<'a, StatefulEventChannel<EntityCrudEvent, <Delegate as PrefabBuilder<'a>>::State>>,
//     );

//     fn run(&mut self, (mut resource_storage, entities, updater, mut crud_events): Self::SystemData) {
//         crud_events.for_each(&self.event_receiver_id, |ep| {
//             match ep.0 {
//                 EntityCrudEvent::NOOP => {}
//                 EntityCrudEvent::Create => {
//                     self.delegate.create(&ep.1, &mut resource_storage, || {
//                         EntitySpawner::LazyBuilder(updater.create_entity(&entities))
//                     });
//                 }
//                 EntityCrudEvent::Update(updating_entity) => {
//                     self.delegate.update(&ep.1, &mut resource_storage, &updating_entity);
//                 }
//             };
//         });
//         crud_events.clear_events();
//     }

//     fn setup(&mut self, world: &mut World) {
//         Self::SystemData::setup(world);
//         self.delegate.setup_delegate(world);
//         let mut channel = world.fetch_mut::<StatefulEventChannel<EntityCrudEvent, Delegate::State>>();
//         self.event_receiver_id = channel.register_with_subs(&[EntityCrudEvent::Create]);
//     }
// }
