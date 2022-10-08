use specs::{Component, NullStorage};

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct NPC;
