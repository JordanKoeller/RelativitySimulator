use specs::prelude::*;
use specs::storage::BTreeStorage;
use specs::Component;

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct EntityManager {
    entities: specs::hibitset::BitSet,
}

impl EntityManager {
    pub fn from(bits: specs::hibitset::BitSet) -> Self {
        Self { entities: bits }
    }
}


impl std::ops::Deref for EntityManager {
    type Target = BitSet;

    fn deref(&self) -> &Self::Target {
        &self.entities
    }
}

impl std::ops::DerefMut for EntityManager {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entities
    }
}

impl EntityManager {
    pub fn add_entity(&mut self, ett: &Entity) {
        self.entities.add(ett.id());
    }

    pub fn remove_entity(&mut self, ett: &Entity) {
        self.entities.remove(ett.id());
    }

    pub fn contains(&self, ett: &Entity) -> bool {
        self.entities.contains(ett.id())
    }
}