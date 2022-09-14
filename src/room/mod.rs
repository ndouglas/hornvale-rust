pub mod storage;
pub use storage::*;

use std::sync::Mutex;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct Room(u64);

lazy_static! {
  pub static ref ROOMS: Mutex<RoomStorage> = Mutex::new(RoomStorage::new());
}

#[named]
pub fn generate_map() {
  let spawn_room = create_room!("Spawn Room", "This is the Spawn Room");
  let _player = create_player!("Player", "It's you, you idiot.", spawn_room);
  let ne_room = create_room!("Northeast Room", "This is the Northeastern Room.");
  let n_room = create_room!("North Room", "This is the Northern Room.");
  let nw_room = create_room!("Northwest Room", "This is the Northwestern Room.");
  let e_room = create_room!("East Room", "This is the Eastern Room.");
  let w_room = create_room!("West Room", "This is the Western Room.");
  let se_room = create_room!("Southeast Room", "This is the Southeastern Room.");
  let s_room = create_room!("South Room", "This is the Southern Room.");
  let sw_room = create_room!("Southwest Room", "This is the Southwestern Room.");
  create_exit!(spawn_room, ne_room, &Direction::Northeast, true);
  create_exit!(spawn_room, n_room, &Direction::North, true);
  create_exit!(n_room, ne_room, &Direction::East, true);
  create_exit!(spawn_room, nw_room, &Direction::Northwest, true);
  create_exit!(n_room, nw_room, &Direction::West, true);
  create_exit!(spawn_room, e_room, &Direction::East, true);
  create_exit!(spawn_room, w_room, &Direction::West, true);
  create_exit!(spawn_room, se_room, &Direction::Southeast, true);
  create_exit!(spawn_room, s_room, &Direction::South, true);
  create_exit!(spawn_room, sw_room, &Direction::Southwest, true);
}
