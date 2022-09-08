#![allow(unused_macros)]
#![allow(unused_imports)]

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

pub mod cli;
pub use cli::*;
pub mod ecs;
pub use ecs::*;
pub mod io;
pub use io::*;
pub mod state;
pub use state::*;

pub mod test {
  use super::*;

  #[named]
  pub fn init() {
    let _ = pretty_env_logger::env_logger::builder().is_test(true).try_init();
    std::env::set_var("RUST_BACKTRACE", "1");
  }
}
