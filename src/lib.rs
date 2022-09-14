#![allow(unused_macros)]
#![allow(unused_imports)]

#[macro_use]
extern crate function_name;
pub use ::function_name::named;
#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod _macro;
pub mod action;
pub mod cli;
pub mod command;
pub mod component;
pub mod effect;
pub mod event;
pub mod io;
pub mod message;
pub mod model;
pub mod player;
pub mod run_mode;
pub mod state;
pub mod tick;

pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  use super::*;

  #[named]
  pub fn init() {
    let _ = builder().is_test(true).try_init();
    set_var("RUST_BACKTRACE", "1");
  }
}
