use cgmath::prelude::*;
use specs::prelude::*;

use app::minecraft::{ChunkComponent, ChunkGrid, FlyingPlayerState, PlayerStateMachine, WalkingPlayerState, BlockHighlightState};
use ecs::components::{Camera, EntityTargetComponent, EventReceiver, MeshComponent, Player};
use ecs::{SystemDelegate, EntityCrudEvent,};
use events::{
    Event, EventChannel, EventPayload, KeyCode, MouseButton, StatefulEventChannel, StatelessEventChannel, WindowEvent
};
use physics::{CanCollide, Gravity, RigidBody, TransformComponent};

// #[derive(SystemData)]
// pub struct BlockInteractionSystemData<'a> {
//     player: ReadStorage<'a, Player>,
//     event_receiver: ReadStorage<'a, EventReceiver>,
//     event_channel: Write<'a, StatelessEventChannel<WindowEvent>>,
//     chunk_grid: Write<'a, ChunkGrid>,
//     chunk_storage: ReadStorage<'a, ChunkComponent>,
//     transform: ReadStorage<'a, TransformComponent>,
//     camera: ReadStorage<'a, Camera>,
//     highlight_spawner: Write<'a, StatefulEventChannel<EntityCrudEvent, BlockHighlightState>>,
// }

#[derive(Default)]
pub struct BlockInterractionSystem {
}

impl<'a> SystemDelegate<'a> for BlockInterractionSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, EventReceiver>,
        Write<'a, StatelessEventChannel<WindowEvent>>,
        Write<'a, ChunkGrid>,
        ReadStorage<'a, ChunkComponent>,
        ReadStorage<'a, TransformComponent>,
        Write<'a, StatefulEventChannel<EntityCrudEvent, BlockHighlightState>>
    );

    fn run(&mut self,(
        player,
        event_receiver,
        events,
        chunk_grid,
        chunk_storage,
        transform_storage,
        mut highlight_spawner
    ): Self::SystemData) {
        for (_p, transform, evt_id) in (&player, &transform_storage, &event_receiver).join() {
            events.for_each(&evt_id.0, |evt| match &evt.code {
                Event::KeyPressed(KeyCode::T) => {
                    self.processs_block_click(transform, &chunk_grid, &chunk_storage, &mut highlight_spawner);
                }
                _ => {}
            })
        }
    }
}

impl BlockInterractionSystem {
    fn processs_block_click<'a>(
        &self,
        transform: &TransformComponent,
        chunk_grid: &Write<'a, ChunkGrid>,
        chunks: &ReadStorage<'a, ChunkComponent>,
        highlight_spawner: &mut Write<'a, StatefulEventChannel<EntityCrudEvent, BlockHighlightState>>
    ) {
        println!("Trying from here!");
        let looking_vec = 5f32 * transform.front();
        let line_start = transform.translation;
        let line_end = line_start + looking_vec;
        if let Some((chunk_index, block_index, block_face)) =
            chunk_grid.get_colliding_along_line(&line_start, &line_end, chunks)
        {
            println!("I'm seeing a block! {:?}", chunk_grid.get_position(&chunk_index, &block_index));
            highlight_spawner.publish((EntityCrudEvent::Create, BlockHighlightState::new(chunk_index, block_index)));

        }
    }
}
