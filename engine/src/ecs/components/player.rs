use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Player;
