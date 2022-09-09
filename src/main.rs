#![allow(unused_macros)]
#![allow(unused_imports)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate function_name;
pub use ::function_name::named;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate specs;
extern crate uuid;

pub use hornvale::*;

use clap::Parser;

#[named]
fn main() {
  pretty_env_logger::init();
  trace_enter!();

  let _args = cli::Arguments::parse();
  let mut state = state::State::new();

  loop {
    state.tick();
    crate::io::read_input(&mut state);
  }
}
