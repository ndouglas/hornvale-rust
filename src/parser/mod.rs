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
pub fn parse(string: String, state: &mut State) {
  let player_entity = get_player!(state.ecs);
  match Command::from_str(&string, player_entity) {
    Ok(command) => state.ecs.insert_command(player_entity, command),
    Err(_) => enq_message!(format!("{}", "What?".bright_red())),
  }
}
