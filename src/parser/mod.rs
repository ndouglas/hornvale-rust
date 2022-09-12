use colored::*;
use specs::prelude::*;

use crate::commands::*;
use crate::ecs::components::*;
use crate::ecs::resources::Player;
use crate::model::Direction;
use crate::queue::enqueue_message;
use crate::state::State;
use crate::traits::WorldUsable;

#[named]
pub fn get_command(state: &mut State, entity: Entity, string: String) -> Option<Command> {
  let words: Vec<&str> = string.split_whitespace().collect();
  let first: String = words.get(0).unwrap_or(&"").to_string();
  let second: String = words.get(1).unwrap_or(&"").to_string();
  match first.as_str() {
    "move" => match second.as_str() {
      "n" | "north" => Some(cmd_move_to!(entity, Direction::North)),
      "ne" | "northeast" => Some(cmd_move_to!(entity, Direction::Northeast)),
      "nw" | "northwest" => Some(cmd_move_to!(entity, Direction::Northwest)),
      "e" | "east" => Some(cmd_move_to!(entity, Direction::East)),
      "w" | "west" => Some(cmd_move_to!(entity, Direction::West)),
      "s" | "south" => Some(cmd_move_to!(entity, Direction::South)),
      "se" | "southeast" => Some(cmd_move_to!(entity, Direction::Southeast)),
      "sw" | "southwest" => Some(cmd_move_to!(entity, Direction::Southwest)),
      "up" => Some(cmd_move_to!(entity, Direction::Up)),
      "down" => Some(cmd_move_to!(entity, Direction::Down)),
      "in" => Some(cmd_move_to!(entity, Direction::In)),
      "out" => Some(cmd_move_to!(entity, Direction::Out)),
      &_ => {
        enq_message!(format!("{}", "What?".bright_red()));
        None
      }
    },
    "n" | "north" => Some(cmd_move_to!(entity, Direction::North)),
    "ne" | "northeast" => Some(cmd_move_to!(entity, Direction::Northeast)),
    "nw" | "northwest" => Some(cmd_move_to!(entity, Direction::Northwest)),
    "e" | "east" => Some(cmd_move_to!(entity, Direction::East)),
    "w" | "west" => Some(cmd_move_to!(entity, Direction::West)),
    "s" | "south" => Some(cmd_move_to!(entity, Direction::South)),
    "se" | "southeast" => Some(cmd_move_to!(entity, Direction::Southeast)),
    "sw" | "southwest" => Some(cmd_move_to!(entity, Direction::Southwest)),
    "up" => Some(cmd_move_to!(entity, Direction::Up)),
    "down" => Some(cmd_move_to!(entity, Direction::Down)),
    "in" => Some(cmd_move_to!(entity, Direction::In)),
    "out" => Some(cmd_move_to!(entity, Direction::Out)),
    "l" | "look" => Some(cmd_look!(entity)),
    "echo" => Some(cmd_echo!(entity, words[1..].join(" "))),
    "quit" => {
      enq_message!(format!("{}", "Quitting...".bright_blue()));
      state.quit();
      None
    }
    &_ => {
      enq_message!(format!("{}", "What?".bright_red()));
      None
    }
  }
}

#[named]
pub fn parse(string: String, state: &mut State) {
  let player_entity = get_player!(state.ecs);
  if let Some(command) = get_command(state, player_entity, string) {
    state.ecs.insert_command(player_entity, command);
  }
}
