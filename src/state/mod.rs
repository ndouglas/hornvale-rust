use specs::prelude::*;
use std::sync::Mutex;

use crate::components::*;
use crate::model::Direction;
use crate::player::PLAYER;

pub struct State {
  pub ecs: World,
  pub tick: u64,
}

lazy_static! {
  pub static ref STATE: Mutex<State> = Mutex::new(State::new());
}


#[named]
pub fn register_components(ecs: &mut World) {
  trace_enter!();
  ecs.register::<HasDescription>();
  ecs.register::<HasName>();
  ecs.register::<HasExits>();
  ecs.register::<IsAPlayer>();
  ecs.register::<IsARoom>();
  ecs.register::<IsInRoom>();
  trace_exit!();
}

#[named]
pub fn insert_resources(ecs: &mut World) {
  let spawn_room = create_room!(ecs, "Spawn Room", "This is the Spawn Room");
  let player = create_player!(ecs, "Player", "It's you, you idiot.", spawn_room);
  PLAYER.lock().unwrap().0 = Some(player);
  let ne_room = create_room!(ecs, "Northeast Room", "This is the Northeastern Room.");
  let n_room = create_room!(ecs, "North Room", "This is the Northern Room.");
  let nw_room = create_room!(ecs, "Northwest Room", "This is the Northwestern Room.");
  let e_room = create_room!(ecs, "East Room", "This is the Eastern Room.");
  let w_room = create_room!(ecs, "West Room", "This is the Western Room.");
  let se_room = create_room!(ecs, "Southeast Room", "This is the Southeastern Room.");
  let s_room = create_room!(ecs, "South Room", "This is the Southern Room.");
  let sw_room = create_room!(ecs, "Southwest Room", "This is the Southwestern Room.");
  ecs.create_exit(spawn_room, ne_room, &Direction::Northeast, true);
  ecs.create_exit(spawn_room, n_room, &Direction::North, true);
  ecs.create_exit(n_room, ne_room, &Direction::East, true);
  ecs.create_exit(spawn_room, nw_room, &Direction::Northwest, true);
  ecs.create_exit(n_room, nw_room, &Direction::West, true);
  ecs.create_exit(spawn_room, e_room, &Direction::East, true);
  ecs.create_exit(spawn_room, w_room, &Direction::West, true);
  ecs.create_exit(spawn_room, se_room, &Direction::Southeast, true);
  ecs.create_exit(spawn_room, s_room, &Direction::South, true);
  ecs.create_exit(spawn_room, sw_room, &Direction::Southwest, true);
}

impl State {
  #[named]
  pub fn new() -> Self {
    let mut ecs = World::new();
    register_components(&mut ecs);
    insert_resources(&mut ecs);
    State { ecs, tick: 0 }
  }
}
