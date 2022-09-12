use specs::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;

use crate::commands::Command;
use crate::traits::Commandable;

lazy_static! {
  pub static ref COMMAND_QUEUE: Mutex<VecDeque<Command>> = Mutex::new(VecDeque::new());
}

#[named]
pub fn enqueue_command(command: Command) {
  COMMAND_QUEUE.lock().unwrap().push_back(command);
}

#[named]
pub fn run_command_queue(ecs: &mut World) {
  loop {
    let command_option: Option<Command> = COMMAND_QUEUE.lock().unwrap().pop_front();
    if let Some(command) = command_option {
      command.execute(ecs);
    } else {
      break;
    }
  }
}
