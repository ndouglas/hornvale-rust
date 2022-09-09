use specs::prelude::*;

use crate::commands::*;
use crate::ecs::components::*;
use crate::ecs::resources::Player;
use crate::model::CompassDirection;
use crate::State;

#[named]
pub fn get_command(string: String) -> Option<Command> {
  trace_enter!();
  let words: Vec<&str> = string.split_whitespace().collect();
  let first: String = words.get(0).unwrap_or(&"").to_string();
  trace_var!(first);
  let second: String = words.get(1).unwrap_or(&"").to_string();
  trace_var!(second);
  let result = match first.as_str() {
    "n" | "north" => Some(cmd_move_to!(North)),
    "ne" | "northeast" => Some(cmd_move_to!(Northeast)),
    "nw" | "northwest" => Some(cmd_move_to!(Northwest)),
    "e" | "east" => Some(cmd_move_to!(East)),
    "w" | "west" => Some(cmd_move_to!(West)),
    "s" | "south" => Some(cmd_move_to!(South)),
    "se" | "southeast" => Some(cmd_move_to!(Southeast)),
    "sw" | "southwest" => Some(cmd_move_to!(Southwest)),
    "l" | "look" => Some(cmd_look!()),
    &_ => None,
  };
  trace_exit!();
  result
}

#[named]
pub fn parse(string: String, state: &mut State) {
  trace_enter!();
  let player_entity;
  {
    player_entity = state.ecs.fetch::<Player>().0;
  }
  if let Some(command) = get_command(string) {
    state.ecs
      .write_storage::<HasCommand>()
      .insert(player_entity, HasCommand { command })
      .expect(format!("Could not insert command {:?} for entity {:?}", command, player_entity).as_str());
  }
  trace_exit!();
}
