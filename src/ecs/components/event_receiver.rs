use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

use events::ReceiverID;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct EventReceiver(pub ReceiverID);
