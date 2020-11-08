use specs::*;

// trait EntityConfig: Sized {

// }

pub struct EntityConstructor<'a>(EntityBuilder<'a>);

impl<'a> EntityConstructor<'a> {
  pub fn new(world: &'a mut World) -> EntityConstructor<'a> {
    EntityConstructor(world.create_entity())
  }

  pub fn build(self) {
    self.0.build();
  }

  pub fn add<C: Component + Send + Sync>(self, component: C) -> EntityConstructor<'a> {
    EntityConstructor(self.0.with(component))
  }

}
