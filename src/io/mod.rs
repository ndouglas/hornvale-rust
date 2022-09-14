use colored::*;
use rustyline::{Editor, ExternalPrinter};
use specs::prelude::*;
use std::sync::Mutex;
use std::thread::{sleep, spawn};
use std::time::Duration;

use crate::command::Command;

use crate::message::get_messages;
use crate::state::STATE;

pub struct State {
  pub ecs: World,
  pub editor: Editor<()>,
}

lazy_static! {
  pub static ref INPUT: Mutex<Editor::<()>> = Mutex::new(Editor::<()>::new().unwrap());
}

#[named]
pub fn start_output() {
  let mut printer = INPUT.lock().unwrap().create_external_printer().unwrap();
  let duration = Duration::from_millis(50);
  spawn(move || loop {
    let messages = get_messages();
    if messages.len() > 0 {
      for message in messages.iter() {
        printer.print(format!("{}", message)).expect("External print failure");
      }
      printer.print(format!("{}", " ")).expect("External print failure");
    }
    sleep(duration);
  });
}

#[named]
pub fn read_input() {
  let input = { INPUT.lock().unwrap().readline(format!("{} ", ">".blue()).as_str()) };
  let result = match input {
    Ok(line) => {
      let player_entity = get_player!();
      match Command::from_str(&line, player_entity) {
        Ok(command) => enq_command!(command),
        Err(_) => enq_message!(format!("{}", "What?".bright_red())),
      }
    },
    Err(_) => {},
  };
  crate::tick::manual_tick();
  result
}
