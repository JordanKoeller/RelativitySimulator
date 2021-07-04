
use specs::prelude::*;
use specs::{Component, VecStorage, NullStorage};




#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Player;
