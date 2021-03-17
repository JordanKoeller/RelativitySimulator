use specs::prelude::*;
use specs::world::LazyBuilder;

use events::{EventChannelWithPayload, ReceiverID};

/////////////////////////////////////////////
// CRUD EVENTS
/////////////////////////////////////////////

#[derive(Clone, Eq, Debug)]
pub enum EntityCrudEvent {
  Create(usize),
  NOOP,
}

impl EntityCrudEvent {
  fn index(&self) -> u32 {
    match self {
      EntityCrudEvent::Create(i) => *i as u32,
      EntityCrudEvent::NOOP => u32::MAX,
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

}

/////////////////////////////////////////////
// Entity manager system definition
/////////////////////////////////////////////

pub struct EntityManager<Delegate> 
where for<'a> Delegate: EntityDelegate<'a> + Default {
  delegate: Delegate,
  event_receiver_id: ReceiverID,
}

impl<Delegate> EntityManager<Delegate>
where for <'a> Delegate: EntityDelegate<'a> + Default {
  fn new(delegate: Delegate) -> Self {
    Self {
      delegate,
      event_receiver_id: usize::MAX,
    }
  }
}

impl<Delegate> Default for EntityManager<Delegate> 
where for <'a> Delegate: EntityDelegate<'a> + Default {
  fn default() -> Self {
    Self {
      delegate: Delegate::default(),
      event_receiver_id: usize::MAX,
    }
  }
}

impl<'a, Delegate> System<'a> for EntityManager<Delegate> 
where for <'b> Delegate: EntityDelegate<'b> + Default {
  type SystemData = (
    <Delegate as EntityDelegate<'a>>::EntityResources,
    Entities<'a>,
    Read<'a, LazyUpdate>,
    Write<'a, EventChannelWithPayload<EntityCrudEvent, <Delegate as EntityDelegate<'a>>::State>>,
  );

  fn run(&mut self, (mut resource_storage, entities, updater, mut crud_events): Self::SystemData) {
    crud_events
    .read(&self.event_receiver_id)
    .for_each(|(event, payload)| {
      match event {
        EntityCrudEvent::NOOP => {}
        EntityCrudEvent::Create(_) => {
            match payload {
              Some(pld) => {
                self.delegate.create(pld.clone(), &mut resource_storage, || {
                  MyBuilder::LazyBuilder(updater.create_entity(&entities))
                });
              }
              None => {}
            }
          }
        };
      });
      crud_events.clear_events();
  }

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    let mut channel = world.fetch_mut::<EventChannelWithPayload<EntityCrudEvent, Delegate::State>>();
    self.event_receiver_id = channel.register_with_subs(&[
      EntityCrudEvent::Create(0),
      EntityCrudEvent::Create(1),
      EntityCrudEvent::Create(2),
      EntityCrudEvent::Create(3),
      EntityCrudEvent::Create(4),
      EntityCrudEvent::Create(5),
      EntityCrudEvent::Create(6),
      EntityCrudEvent::Create(7),
      EntityCrudEvent::Create(8),
      EntityCrudEvent::Create(9),
      EntityCrudEvent::Create(10),
      EntityCrudEvent::Create(11),
      EntityCrudEvent::Create(12),
      EntityCrudEvent::Create(13),
      EntityCrudEvent::Create(14),
      EntityCrudEvent::Create(15),
      EntityCrudEvent::Create(16),
      EntityCrudEvent::Create(17),
      EntityCrudEvent::Create(18),
      EntityCrudEvent::Create(19),
      EntityCrudEvent::Create(20),
      EntityCrudEvent::Create(21),
      EntityCrudEvent::Create(22),
      EntityCrudEvent::Create(23),
      EntityCrudEvent::Create(24),
      EntityCrudEvent::Create(25),
      EntityCrudEvent::Create(26),
      EntityCrudEvent::Create(27),
      EntityCrudEvent::Create(28),
      EntityCrudEvent::Create(29),
      EntityCrudEvent::Create(30),
      EntityCrudEvent::Create(31),
      EntityCrudEvent::Create(32),
      EntityCrudEvent::Create(33),
      EntityCrudEvent::Create(34),
      EntityCrudEvent::Create(35),
      EntityCrudEvent::Create(36),
      EntityCrudEvent::Create(37),
      EntityCrudEvent::Create(38),
      EntityCrudEvent::Create(39),
      EntityCrudEvent::Create(40),
      EntityCrudEvent::Create(41),
      EntityCrudEvent::Create(42),
      EntityCrudEvent::Create(43),
      EntityCrudEvent::Create(44),
      EntityCrudEvent::Create(45),
      EntityCrudEvent::Create(46),
      EntityCrudEvent::Create(47),
      EntityCrudEvent::Create(48),
      EntityCrudEvent::Create(49),
      EntityCrudEvent::Create(50),
      EntityCrudEvent::Create(51),
      EntityCrudEvent::Create(52),
      EntityCrudEvent::Create(53),
      EntityCrudEvent::Create(54),
      EntityCrudEvent::Create(55),
      EntityCrudEvent::Create(56),
      EntityCrudEvent::Create(57),
      EntityCrudEvent::Create(58),
      EntityCrudEvent::Create(59),
      EntityCrudEvent::Create(60),
      EntityCrudEvent::Create(61),
      EntityCrudEvent::Create(62),
      EntityCrudEvent::Create(63),
      EntityCrudEvent::Create(64),
      EntityCrudEvent::Create(65),
      EntityCrudEvent::Create(66),
      EntityCrudEvent::Create(67),
      EntityCrudEvent::Create(68),
      EntityCrudEvent::Create(69),
      EntityCrudEvent::Create(70),
      EntityCrudEvent::Create(71),
      EntityCrudEvent::Create(72),
      EntityCrudEvent::Create(73),
      EntityCrudEvent::Create(74),
      EntityCrudEvent::Create(75),
      EntityCrudEvent::Create(76),
      EntityCrudEvent::Create(77),
      EntityCrudEvent::Create(78),
      EntityCrudEvent::Create(79),
      EntityCrudEvent::Create(80),
      EntityCrudEvent::Create(81),
      EntityCrudEvent::Create(82),
      EntityCrudEvent::Create(83),
      EntityCrudEvent::Create(84),
      EntityCrudEvent::Create(85),
      EntityCrudEvent::Create(86),
      EntityCrudEvent::Create(87),
      EntityCrudEvent::Create(88),
      EntityCrudEvent::Create(89),
      EntityCrudEvent::Create(90),
      EntityCrudEvent::Create(91),
      EntityCrudEvent::Create(92),
      EntityCrudEvent::Create(93),
      EntityCrudEvent::Create(94),
      EntityCrudEvent::Create(95),
      EntityCrudEvent::Create(96),
      EntityCrudEvent::Create(97),
      EntityCrudEvent::Create(98),
      EntityCrudEvent::Create(99),
      EntityCrudEvent::Create(100),
      EntityCrudEvent::Create(101),
      EntityCrudEvent::Create(102),
      EntityCrudEvent::Create(103),
      EntityCrudEvent::Create(104),
      EntityCrudEvent::Create(105),
      EntityCrudEvent::Create(106),
      EntityCrudEvent::Create(107),
      EntityCrudEvent::Create(108),
      EntityCrudEvent::Create(109),
      ]);
  }
}
