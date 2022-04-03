use cgmath::prelude::Zero;
use physics::TransformComponent;
use shapes::{Block, Sprite};
use specs::prelude::*;
use utils::{QuatF, Vec3F};

use renderer::{Drawable, Mesh, Renderer};

use app::minecraft::components::ChunkComponent;

#[derive(Default)]
pub struct ChunkManager;

impl<'a> System<'a> for ChunkManager {
    type SystemData = (WriteStorage<'a, ChunkComponent>,);

    fn run(&mut self, _data: Self::SystemData) {}

    fn setup(&mut self, world: &mut World) {
        world.register::<ChunkComponent>();
        let template_block = Block::new("resources/minecraft/grass_block.png");
        let template_mtl = template_block.material();
        let d_id = {
            let mut renderer = world.write_resource::<Renderer>();
            renderer.submit_model(template_block.mesh())
        };
        for y in 0..1 {
            for x in 0..32 {
                for z in 0..32 {
                    let transform = TransformComponent::new(
                        Vec3F::new(x as f32, 1f32 - (y as f32), z as f32),
                        Vec3F::new(1f32, 1f32, 1f32),
                        QuatF::zero(),
                    );
                    world
                        .create_entity()
                        .with(template_mtl.clone())
                        .with(d_id.clone())
                        .with(transform)
                        .build();
                }
            }
        }
    }
}

impl ChunkManager {
    fn minify(chunk: &mut ChunkComponent) {}
}
