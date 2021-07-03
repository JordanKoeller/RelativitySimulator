
use specs::prelude::*;
use specs::{Component, VecStorage, NullStorage, DefaultVecStorage};

use events::ReceiverID;



#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct EventReceiver(pub ReceiverID);
