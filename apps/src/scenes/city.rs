use app::entities::{
  create_floor, create_player, DistrictState
};
use ecs::{EntityCrudEvent};
use events::{EventChannel, StatefulEventChannel};
use specs::prelude::*;

use utils::*;

const LAYOUT: &str = "\
########
#......#
#.##.#.#
#......#
##.#.#.#
##.#.#.#
##.#.#.#
#......#
########\
";

pub fn build_city(world: &mut World) {
  let player_pos = Vec3F::new(30f64, 6f64, 65f64);
  create_player(player_pos, world);
  {
    let mut evt_mgr = world.fetch_mut::<StatefulEventChannel<EntityCrudEvent, DistrictState>>();
    evt_mgr.publish((
      EntityCrudEvent::Create,
      DistrictState::new(Vec3F::new(10f64, 0f64, 10f64), Vec2F::new(90f64, 90f64), LAYOUT.to_string()),
    ));
  }
}
