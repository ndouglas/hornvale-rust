use specs::prelude::*;
use std::collections::HashMap;

pub mod player;
pub use player::*;
pub mod run_mode;
pub use run_mode::*;
pub mod spawn_room;
pub use spawn_room::*;
pub mod tick;
pub use tick::*;

use crate::ecs::components::*;
use crate::model::Direction;
use crate::model::Exit;
use crate::model::Exits;

#[named]
pub fn insert_resources(ecs: &mut World) {
  ecs.insert(RunMode::MainGame);
  ecs.insert(Tick(0));
  let spawn_room = create_room!(ecs, "Spawn Room", "This is the Spawn Room");
  ecs.insert(SpawnRoom(spawn_room));
  let ne_room = create_room!(ecs, "Northeast Room", "This is the Northeastern Room.");
  ecs.create_exit(spawn_room, ne_room, &Direction::Northeast, true);
  let n_room = create_room!(ecs, "North Room", "This is the Northern Room.");
  ecs.create_exit(spawn_room, n_room, &Direction::North, true);
  ecs.create_exit(n_room, ne_room, &Direction::East, true);
  let nw_room = create_room!(ecs, "Northwest Room", "This is the Northwestern Room.");
  ecs.create_exit(spawn_room, nw_room, &Direction::Northwest, true);
  ecs.create_exit(n_room, nw_room, &Direction::West, true);
  let e_room = create_room!(ecs, "East Room", "This is the Eastern Room.");
  ecs.create_exit(spawn_room, e_room, &Direction::East, true);
  let w_room = create_room!(ecs, "West Room", "This is the Western Room.");
  ecs.create_exit(spawn_room, w_room, &Direction::West, true);
  let se_room = create_room!(ecs, "Southeast Room", "This is the Southeastern Room.");
  ecs.create_exit(spawn_room, se_room, &Direction::Southeast, true);
  let s_room = create_room!(ecs, "South Room", "This is the Southern Room.");
  ecs.create_exit(spawn_room, s_room, &Direction::South, true);
  let sw_room = create_room!(ecs, "Southwest Room", "This is the Southwestern Room.");
  ecs.create_exit(spawn_room, sw_room, &Direction::Southwest, true);
  let _player = create_player!(ecs, "Player", "It's you, you idiot.", spawn_room);
}
