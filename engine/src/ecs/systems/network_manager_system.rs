use specs::prelude::*;

use crate::ecs::{MonoBehavior, SystemUtilities, WorldProxy};
use crate::events::{Event, EventChannel, KeyCode, ReceiverID, StatelessEventChannel, WindowEvent};
use crate::net::{NetActor, NetActorHandle};

pub struct NetworkManager {
    network_handle: Option<NetActorHandle>,
}

impl<'a> MonoBehavior<'a> for NetworkManager {
    type SystemData = ();

    fn run(&mut self, api: SystemUtilities<'a>, data: Self::SystemData) {}

    fn setup(&mut self, mut world: WorldProxy) {
        Self::SystemData::setup(&mut world);
        let (network, network_handle) = NetActor::create();
        network.execute();
        self.network_handle = Some(network_handle);
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self { network_handle: None }
    }
}
