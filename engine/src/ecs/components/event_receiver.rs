use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

use crate::events::ReceiverID;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct EventReceiver(pub ReceiverID);
