use specs::prelude::*;

use events::{Event, EventChannel, KeyCode, WindowEvent};
use utils::Vec3F;

use ecs::components::*;

use app::entities::create_player;

pub struct Scene {
  world: World,
}

impl Scene {
  pub fn new(cam_loc: Vec3F, mut world: World) -> Scene {
    create_player(cam_loc, &mut world);
    Scene { world }
  }

}
