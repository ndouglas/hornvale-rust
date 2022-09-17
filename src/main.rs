#![allow(unused_macros)]
#![allow(unused_imports)]

#[macro_use]
extern crate function_name;

use clap::Parser;
use pretty_env_logger::init as pretty_env_logger_init;
use specs::prelude::*;

use state::State;

use hornvale::*;

#[named]
fn main() {
  pretty_env_logger_init();
  let _args = cli::Arguments::parse();
  let mut state = State::new();
  state.run();
}
