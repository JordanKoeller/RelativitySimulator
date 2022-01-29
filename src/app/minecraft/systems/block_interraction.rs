use specs::prelude::*;
use cgmath::prelude::*;

use app::minecraft::{ChunkComponent, ChunkGrid, FlyingPlayerState, WalkingPlayerState, PlayerStateMachine};
use ecs::components::{Camera, EntityTargetComponent, EventReceiver, MeshComponent, Player};
use ecs::SystemDelegate;
use events::{Event, EventChannel, EventPayload, KeyCode, StatefulEventChannel, StatelessEventChannel, WindowEvent, MouseButton};
use physics::{Gravity, RigidBody, TransformComponent, CanCollide};


#[derive(SystemData)]
pub struct BlockInteractionSystemData<'a> {
    player: ReadStorage<'a, Player>,
    event_receiver: ReadStorage<'a, EventReceiver>,
    event_channel: Write<'a, StatelessEventChannel<WindowEvent>>,
    chunk_grid: Write<'a, ChunkGrid>,
    chunk_storage: ReadStorage<'a, ChunkComponent>,
    transform: ReadStorage<'a, TransformComponent>,
    camera: ReadStorage<'a, Camera>,
}

pub struct BlockInterractionSystem {

}

impl<'a> SystemDelegate<'a> for BlockInterractionSystem {
    type SystemData = BlockInteractionSystemData<'a>;

    fn run(&mut self, mut s: Self::SystemData) {
        for (_p, cam, transform, events) in (&s.player, &s.camera, &s.transform, &s.event_receiver).join() {
            s.event_channel.for_each(&events.0, |evt| {
                match &evt.code {
                    Event::MouseDown(button) => {
                        self.processs_block_click(button, transform, cam, &s.chunk_grid, &s.chunk_storage);
                    }
                    _ => {}
                }
            })
        }
    }

}

impl BlockInterractionSystem {
    fn processs_block_click<'a>(&self, button: &MouseButton, transform: &TransformComponent, camera: &Camera, chunk_grid: &Write<'a, ChunkGrid>, chunks: &ReadStorage<'a, ChunkComponent>) {
        
    }
}
