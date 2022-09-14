#![allow(unused_macros)]
#![allow(unused_imports)]

#[macro_use]
extern crate function_name;

use clap::Parser;
use pretty_env_logger::init as pretty_env_logger_init;

use hornvale::*;

#[named]
fn main() {
  pretty_env_logger_init();
  state::State::new();
  io::start_output();
  let _args = cli::Arguments::parse();
  message::start_message_spammer();
  tick::start_tick();
  // Main game loop.
  loop {
    io::read_input();
    if !run_mode::RUN_MODE.lock().unwrap().should_continue() {
      break;
    }
  }
}
