use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use specs::prelude::*;
use specs::world::LazyBuilder;

use crate::debug::Logger;
use crate::ecs::PrefabBuilder;
use crate::events::{EventChannel, StatefulEventChannel};
use crate::graphics::AssetLibrary;
use crate::gui::{ControlPanel, ControlPanels};

// Provides a common interface for accessing commonly used resources
// All fields inside this should only be specified as `Read` or `ReadStorage` access.
// If mutation is used, please use interior mutability unless
#[derive(SystemData)]
pub struct SystemUtilities<'a> {
    logger: Read<'a, Logger>,
    entities: Entities<'a>,
    lazy_update: Read<'a, LazyUpdate>,
    asset_library: Read<'a, AssetLibrary>,
    control_panels: Read<'a, ControlPanels>,
}

impl<'a> SystemUtilities<'a> {
    pub fn log(&self) -> &Logger {
        &self.logger
    }

    pub fn entity_builder(&self) -> LazyBuilder<'_> {
        self.lazy_update.create_entity(&self.entities)
    }

    pub fn add_component<T: Component + Sync + Send>(&self, entity: &Entity, component: T) {
        self.lazy_update.insert(*entity, component);
    }

    pub fn remove_component<T: Component + Sync + Send>(&self, entity: &Entity) {
        self.lazy_update.remove::<T>(*entity);
    }

    pub fn delete_entity(&self, entity: Entity) -> bool {
        match self.entities.delete(entity) {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub fn assets(&self) -> &AssetLibrary {
        &self.asset_library
    }

    pub fn control_panel(&self, id: TypeId) -> Option<&RwLock<ControlPanel>> {
        self.control_panels.get(&id)
    }
}

impl<'a> std::ops::Deref for SystemUtilities<'a> {
    type Target = AssetLibrary;

    fn deref(&self) -> &AssetLibrary {
        &self.asset_library
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::{TestingEcs, TestingEcsBuilder};

    use super::*;

    #[derive(Default)]
    struct MakePrefabTest {
        pub ett: Option<Entity>,
    }

    impl<'a> System<'a> for MakePrefabTest {
        type SystemData = SystemUtilities<'a>;

        fn run(&mut self, data: Self::SystemData) {
            if let Some(e) = self.ett {
                data.delete_entity(e);
            } else {
                self.ett = Some(data.entity_builder().build());
            }
        }
    }

    #[test]
    fn build_prefab() {
        let mut test_ecs = TestingEcsBuilder::new().with_system(MakePrefabTest::default()).build();
        test_ecs.run();
        assert_eq!(test_ecs.entities().len(), 1);
        test_ecs.run();
        assert_eq!(test_ecs.entities().len(), 0);
    }
}
