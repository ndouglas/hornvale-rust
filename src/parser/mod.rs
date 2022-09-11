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
  trace_enter!();
  let words: Vec<&str> = string.split_whitespace().collect();
  let first: String = words.get(0).unwrap_or(&"").to_string();
  trace_var!(first);
  let second: String = words.get(1).unwrap_or(&"").to_string();
  trace_var!(second);
  let result = match first.as_str() {
    "move" => match second.as_str() {
      "n" | "north" => Some(cmd_move_to!(entity, North)),
      "ne" | "northeast" => Some(cmd_move_to!(entity, Northeast)),
      "nw" | "northwest" => Some(cmd_move_to!(entity, Northwest)),
      "e" | "east" => Some(cmd_move_to!(entity, East)),
      "w" | "west" => Some(cmd_move_to!(entity, West)),
      "s" | "south" => Some(cmd_move_to!(entity, South)),
      "se" | "southeast" => Some(cmd_move_to!(entity, Southeast)),
      "sw" | "southwest" => Some(cmd_move_to!(entity, Southwest)),
      "up" => Some(cmd_move_to!(entity, Up)),
      "down" => Some(cmd_move_to!(entity, Down)),
      "in" => Some(cmd_move_to!(entity, In)),
      "out" => Some(cmd_move_to!(entity, Out)),
      &_ => {
        enqueue_message(format!("{}", "What?".bright_red()));
        None
      }
    },
    "n" | "north" => Some(cmd_move_to!(entity, North)),
    "ne" | "northeast" => Some(cmd_move_to!(entity, Northeast)),
    "nw" | "northwest" => Some(cmd_move_to!(entity, Northwest)),
    "e" | "east" => Some(cmd_move_to!(entity, East)),
    "w" | "west" => Some(cmd_move_to!(entity, West)),
    "s" | "south" => Some(cmd_move_to!(entity, South)),
    "se" | "southeast" => Some(cmd_move_to!(entity, Southeast)),
    "sw" | "southwest" => Some(cmd_move_to!(entity, Southwest)),
    "up" => Some(cmd_move_to!(entity, Up)),
    "down" => Some(cmd_move_to!(entity, Down)),
    "in" => Some(cmd_move_to!(entity, In)),
    "out" => Some(cmd_move_to!(entity, Out)),
    "l" | "look" => Some(cmd_look!(entity)),
    "echo" => Some(cmd_echo!(entity, words[1..].join(" "))),
    "quit" => {
      enqueue_message(format!("{}", "Quitting...".bright_blue()));
      state.quit();
      None
    }
    &_ => {
      enqueue_message(format!("{}", "What?".bright_red()));
      None
    }
  };
  trace_exit!();
  result
}

#[named]
pub fn parse(string: String, state: &mut State) {
  trace_enter!();
  let player_entity = state.ecs.get_player_entity();
  if let Some(command) = get_command(state, player_entity, string) {
    state.ecs.insert_command(player_entity, command);
  }
  trace_exit!();
}
