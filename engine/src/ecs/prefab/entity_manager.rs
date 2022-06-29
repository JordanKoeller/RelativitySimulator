
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
        Self {
            entities: bits
        }
    }
}