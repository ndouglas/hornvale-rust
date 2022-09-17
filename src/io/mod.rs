use colored::*;
use rustyline::{Editor, ExternalPrinter};
use std::sync::Mutex;
use std::thread::{sleep, spawn};
use std::time::Duration;

use crate::command::Command;

use crate::message::get_messages;

lazy_static! {
  pub static ref INPUT: Mutex<Editor::<()>> = Mutex::new(Editor::<()>::new().unwrap());
}

#[named]
pub fn read_input() {
  let input = INPUT.lock().unwrap().readline(format!("{} ", ">".blue()).as_str());
  let result = match input {
    Ok(line) => {
      use crate::player::PLAYER;
      if let Some(player_entity) = PLAYER.lock().unwrap().0 {
        match Command::from_str(&line, player_entity) {
          Ok(command) => enq_command!(command),
          Err(_) => enq_message!(format!("{}", "What?".bright_red())),
        }
      }
    },
    Err(_) => {},
  };
  // crate::message::show_spinner(20);
  crate::tick::manual_tick();
  result
}
