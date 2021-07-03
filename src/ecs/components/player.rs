
use specs::prelude::*;
use specs::{Component, VecStorage, NullStorage, DefaultVecStorage};




#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Player;
