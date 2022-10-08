use specs::prelude::*;
use specs::{Component, NullStorage, VecStorage};

use crate::events::ReceiverId;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct EventReceiver(pub ReceiverId);
