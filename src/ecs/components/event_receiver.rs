
use specs::prelude::*;
use specs::{Component, VecStorage, NullStorage};

use events::ReceiverID;



#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct EventReceiver(pub ReceiverID);
