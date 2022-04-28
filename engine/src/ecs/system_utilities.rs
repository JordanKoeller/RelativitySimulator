use specs::prelude::*;
use specs::world::LazyBuilder;

use crate::events::{StatefulEventChannel, EventChannel};
use crate::debug::{Logger};
use crate::ecs::PrefabBuilder;
use crate::graphics::AssetLibrary;

// Provides a common interface for accessing commonly used resources
// All fields inside this should only be specified as `Read` or `ReadStorage` access.
// If mutation is used, please use interior mutability unless
#[derive(SystemData)]
pub struct SystemUtilities<'a> {
    logger: Read<'a, Logger>,
    entities: Entities<'a>,
    lazy_update: Read<'a, LazyUpdate>,
    asset_library: Read<'a, AssetLibrary>,
}

impl<'a> SystemUtilities<'a> {

    pub fn log(&self) -> &Logger {
        &self.logger
    }

    pub fn entity_builder(&self) -> LazyBuilder<'_> {
        self.lazy_update.create_entity(&self.entities)
    }

    pub fn delete_entity(&self, entity: Entity) -> bool {
        match self.entities.delete(entity) {
            Result::Ok(_) => true,
            Result::Err(_) => false
        }
    }

    pub fn assets(&self) -> &AssetLibrary {
        &self.asset_library
    }
    
}


#[cfg(test)]
mod tests {
    use crate::testing::{TestingEcsBuilder, TestingEcs};

    use super::*;

    #[derive(Default)]
    struct MakePrefabTest {
        pub ett: Option<Entity>
    }

    impl <'a> System<'a> for MakePrefabTest {
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
        let mut test_ecs = TestingEcsBuilder::new()
            .with_system(MakePrefabTest::default())
            .build();
        test_ecs.run();
        assert_eq!(test_ecs.entities().len(), 1);
        test_ecs.run();
        assert_eq!(test_ecs.entities().len(), 0);
    }
}