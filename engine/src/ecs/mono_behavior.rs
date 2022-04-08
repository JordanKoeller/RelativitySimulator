use specs::prelude::*;

use crate::ecs::SystemUtilities;

pub trait MonoBehavior<'a> {
    type SystemData: specs::SystemData<'a>;

    fn run(&mut self, api: SystemUtilities<'a>, resources: Self::SystemData);

    #[allow(unused_variables)]
    fn setup(&mut self, world: &mut World) {

    }
    #[allow(unused_variables)]
    fn destroy(&mut self, api: SystemUtilities<'a>, resources: Self::SystemData) {

    }
}

struct Sys<M> 
where for<'a> M: MonoBehavior<'a> {
    mono_behavior: M
}

impl<'a, M> System<'a> for Sys<M>
where for<'b> M: MonoBehavior<'b> {
    type SystemData = (
        <M as MonoBehavior<'a>>::SystemData,
        SystemUtilities<'a>,
    );

    fn run(&mut self, (delegate_resources, utilities): Self::SystemData) {
        self.mono_behavior.run(utilities, delegate_resources);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.mono_behavior.setup(world);
    }

}

impl<M> Default for Sys<M>
where for <'a> M: MonoBehavior<'a> + Default {
    fn default() -> Self {
        Self {
            mono_behavior: M::default()
        }
    }
}