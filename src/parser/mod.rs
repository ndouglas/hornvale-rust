use crate::commands::move_compass_direction;
use crate::ecs::resources::Player;
use crate::model::CompassDirection;
use crate::State;

#[named]
pub fn parse(string: String, state: &mut State) {
  trace_enter!();
  let player_entity;
  {
    player_entity = state.ecs.fetch::<Player>().0;
  }
  let words: Vec<&str> = string.split_whitespace().collect();
  let first: String = words.get(0).unwrap_or(&"").to_string();
  trace_var!(first);
  let second: String = words.get(1).unwrap_or(&"").to_string();
  trace_var!(second);
  match first.as_str() {
    "n" | "north" => {
      trace!("Going north");
      move_compass_direction(&mut state.ecs, player_entity, CompassDirection::North);
    }
    "ne" | "northeast" => {
      trace!("Going northeast");
      move_compass_direction(&mut state.ecs, player_entity, CompassDirection::Northeast);
    }
    "nw" | "northwest" => {
      trace!("Going northwest");
      move_compass_direction(&mut state.ecs, player_entity, CompassDirection::Northwest);
    }
    "e" | "east" => {
      trace!("Going east");
      move_compass_direction(&mut state.ecs, player_entity, CompassDirection::East);
    }
    "w" | "west" => {
      trace!("Going west");
      move_compass_direction(&mut state.ecs, player_entity, CompassDirection::West);
    }
    "s" | "south" => {
      trace!("Going south");
      move_compass_direction(&mut state.ecs, player_entity, CompassDirection::South);
    }
    "se" | "southeast" => {
      trace!("Going southeast");
      move_compass_direction(&mut state.ecs, player_entity, CompassDirection::Southeast);
    }
    "sw" | "southwest" => {
      trace!("Going southwest");
      move_compass_direction(&mut state.ecs, player_entity, CompassDirection::Southwest);
    }
    "l" | "look" => {}
    &_ => {
      print!("Your command is unrecognized and therefore does diddly.\n");
    }
  }
  trace_exit!();
}
