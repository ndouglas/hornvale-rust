#![allow(unused_macros)]

#[macro_use]
extern crate function_name;
pub use ::function_name::named;
// #[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[macro_use]
pub mod _macro;
pub mod action;
pub mod application;
pub mod cli;
pub mod command;
pub mod component;
pub mod event;
pub mod navigation;
pub mod resource;
pub mod scripting;
pub mod system;

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
