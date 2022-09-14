use colored::*;
use rustyline::Editor;
use specs::prelude::*;
use std::sync::Mutex;

use crate::actions::run_action_queue;
use crate::commands::{ Command, enqueue_command, run_command_queue };
use crate::ecs::components::*;
use crate::ecs::resources::*;
use crate::io::INPUT;
use crate::queue::*;
use crate::run_mode::RUN_MODE;
use crate::tick::TICK;

pub struct State {
  pub ecs: World,
}

lazy_static! {
  pub static ref STATE: Mutex<State> = Mutex::new(State::new());
}

impl State {
  #[named]
  pub fn new() -> Self {
    let mut ecs = World::new();
    register_components(&mut ecs);
    insert_resources(&mut ecs);
    State { ecs }
  }

  #[named]
  pub fn tick(&mut self) {
    run_command_queue();
    run_action_queue();
    run_effect_queue(&mut self.ecs);
    run_event_queue(&mut self.ecs);
  }

  #[named]
  pub fn should_continue(&self) -> bool {
    RUN_MODE.lock().unwrap().should_continue()
  }

  #[named]
  pub fn read_input(&self) {
    let input = INPUT.lock().unwrap().readline(format!("{} ", ">".blue()).as_str());
    match input {
      Ok(line) => {
        let player_entity = get_player!(self.ecs);
        match Command::from_str(&line, player_entity) {
          Ok(command) => enq_command!(command),
          Err(_) => enq_message!(format!("{}", "What?".bright_red())),
        }
      },
      Err(_) => {},
    }
  }
}
