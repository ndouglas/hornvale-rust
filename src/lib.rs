#![allow(unused_macros)]
#![allow(unused_imports)]

extern crate colored;
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

#[macro_use]
pub mod macros;
pub use macros::*;

pub mod action;
pub mod cli;
pub mod commands;
pub mod components;
pub mod effects;
pub mod events;
pub mod io;
pub mod model;
pub mod player;
pub mod queue;
pub mod run_mode;
pub mod state;
pub mod tick;

pub mod test {
  use super::*;

  #[named]
  pub fn init() {
    let _ = pretty_env_logger::env_logger::builder().is_test(true).try_init();
    std::env::set_var("RUST_BACKTRACE", "1");
  }
}
