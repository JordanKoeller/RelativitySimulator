use specs::prelude::*;
use specs::world::LazyBuilder;

use crate::events::{StatefulEventChannel, EventChannel};
use crate::debug::{Logger};
use crate::ecs::PrefabBuilder;

// Provides a common interface for accessing commonly used resources
// All fields inside this should only be specified as `Read` or `ReadStorage` access.
// If mutation is used, please use interior mutability unless
#[derive(SystemData)]
pub struct SystemUtilities<'a> {
    logger: Read<'a, Logger>,
    entities: Entities<'a>,
    lazy_update: Read<'a, LazyUpdate>,
}

impl<'a> SystemUtilities<'a> {

    pub fn log(&self) -> &Logger {
        &self.logger
    }

    pub fn entity_builder(&self) -> LazyBuilder<'_> {
        self.lazy_update.create_entity(&self.entities)
    }

    pub fn build_prefab<B: PrefabBuilder>(&self, state: B::PrefabState)  -> Entity {
        let builder = B::build(self.entity_builder(), state);
        builder.build()
    }

    pub fn delete_entity(&self, entity: Entity) -> bool {
        match self.entities.delete(entity) {
            Result::Ok(_) => true,
            Result::Err(_) => false
        }
    }

    // pub fn get_panel_values(&self, system_id: ReceiverID) -> &ImguiPanelValues {

    // }
    
}