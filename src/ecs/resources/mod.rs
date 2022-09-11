use specs::prelude::*;
use std::collections::HashMap;

pub mod player;
pub use player::*;
pub mod spawn_room;
pub use spawn_room::*;
pub mod tick;
pub use tick::*;

use crate::ecs::components::*;
use crate::model::Direction;
use crate::model::Exit;
use crate::model::Exits;

#[named]
pub fn create_room(ecs: &mut World, name: String) -> Entity {
  trace_enter!();
  let entityBuilder = ecs
    .create_entity()
    .has_name(name)
    .has_description("This is just a nondescript room.".into())
    .is_a_room()
    .has_exits()
    .build();
  trace_exit!();
  entityBuilder
}

#[named]
pub fn insert_resources(ecs: &mut World) {
  trace_enter!();
  ecs.insert(Tick(0));
  let spawn_room = create_room(ecs, "Spawn Room".into());
  ecs.insert(SpawnRoom(spawn_room));
  let ne_room = create_room(ecs, "Northeast Room".into());
  ecs.create_exit(spawn_room, ne_room, &Direction::Northeast, true);
  let n_room = create_room(ecs, "North Room".into());
  ecs.create_exit(spawn_room, n_room, &Direction::North, true);
  let nw_room = create_room(ecs, "Northwest Room".into());
  ecs.create_exit(spawn_room, nw_room, &Direction::Northwest, true);
  let e_room = create_room(ecs, "East Room".into());
  ecs.create_exit(spawn_room, e_room, &Direction::East, true);
  let w_room = create_room(ecs, "West Room".into());
  ecs.create_exit(spawn_room, w_room, &Direction::West, true);
  let se_room = create_room(ecs, "Southeast Room".into());
  ecs.create_exit(spawn_room, se_room, &Direction::Southeast, true);
  let s_room = create_room(ecs, "South Room".into());
  ecs.create_exit(spawn_room, s_room, &Direction::South, true);
  let sw_room = create_room(ecs, "Southwest Room".into());
  ecs.create_exit(spawn_room, sw_room, &Direction::Southwest, true);
  let player = ecs
    .create_entity()
    .has_name("Player".into())
    .has_description("It's you, idiot.".into())
    .is_a_player()
    .with(IsInRoom(spawn_room))
    .build();
  ecs.insert(Player(player));
  trace_exit!();
}
