use app::entities::{
  create_floor, create_player, DistrictState
};
use ecs::{EntityCrudEvent};
use events::EventChannelWithPayload;
use specs::prelude::*;

use utils::*;

const LAYOUT: &str = "\
########
#......#
#.##.#.#
#......#
##.#.#.#
##.#.#.#
#......#
########\
";

pub fn build_city(world: &mut World) {
  let player_pos = Vec3F::new(30f32, 2f32, 70f32);
  create_player(player_pos, world);
  {
    let mut evt_mgr = world.fetch_mut::<EventChannelWithPayload<EntityCrudEvent, DistrictState>>();
    evt_mgr.publish_with_payload(
      EntityCrudEvent::Create(1),
      DistrictState::new(Vec3F::new(10f32, 0f32, 10f32), Vec2F::new(90f32, 90f32), LAYOUT.to_string()),
    );
  }
}
