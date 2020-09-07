use renderer;
use stateful::Scene;
use utils;

use initializers::AssetManager;

pub struct CubeScene {}

impl CubeScene {
    pub fn get_scene(asset_manager: &mut AssetManager) -> Scene {
        let cube = renderer::modeling::ColoredCube::new(
            utils::Vec3F::new(0.0, 0.0, -10.0),
            utils::Color::new(1.0, 0.0, 1.0),
            asset_manager,
        );
        let crate_block = renderer::modeling::TexturedBlock::new(
            utils::Vec3F::new(6.0, 0.0, -10.0),
            "resources/textures/crate.png",
            asset_manager,
        );
        let skybox = renderer::modeling::Skybox::new("resources/textures/skybox", asset_manager);
        let player_pos = utils::Vec3F::new(3.0, 3.0, 0.0);
        let player_facing = utils::Vec3F::unit_z();
        Scene::new(
            vec![
                Box::new(cube),
                Box::new(skybox),
                Box::new(crate_block),
            ],
            player_pos,
            player_facing,
        )
    }
}
