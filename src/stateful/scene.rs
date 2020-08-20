use utils::Vec3F;

use mechanics::Player;
use renderer::IRenderable;

pub struct Scene {
    world: Vec<Box<dyn IRenderable>>,
    player: Player,
}

impl Scene {
    pub fn get_renderables(&self) -> impl Iterator<Item = &Box<dyn IRenderable>> {
        self.world.iter()
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn new(world: Vec<Box<dyn IRenderable>>, player_pos: Vec3F, player_facing: Vec3F) -> Scene {
        let player = Player::new(player_pos, player_facing);
        Scene { world, player }
    }
}
