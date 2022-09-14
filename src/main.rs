#![allow(unused_macros)]
#![allow(unused_imports)]

#[macro_use]
extern crate function_name;

use clap::Parser;
use pretty_env_logger::init as pretty_env_logger_init;

use io::start_output;
use message::start_message_spammer;
use state::State;
use tick::start_tick;

use hornvale::*;

#[named]
fn main() {
  pretty_env_logger_init();
  State::new();
  start_output();
  let _args = cli::Arguments::parse();
  start_message_spammer();
  start_tick();
  // Main game loop.
  loop {
    io::read_input();
    if !run_mode::RUN_MODE.lock().unwrap().should_continue() {
      break;
    }
  }
}
