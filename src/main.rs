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

// TEMPORARY

use rustyline::{Editor, ExternalPrinter, Result};
use std::thread;
use std::time::Duration;

// END TEMPORARY

#[named]
fn main() {
  pretty_env_logger::init();
  trace_enter!();

  let mut editor = Editor::<()>::new().unwrap();
  let mut printer = editor.create_external_printer().unwrap();

  let _args = cli::Arguments::parse();
  let mut state = state::State::new(editor);
  
  thread::spawn(move || {
    loop {
      let messages = queue::get_messages();
      if messages.len() > 0 {
        for message in messages.iter() {
          printer
            .print(format!("{}", message))
            .expect("External print failure");
        }  
        printer
          .print(format!("{}", " "))
          .expect("External print failure");
      }
      thread::sleep(Duration::from_millis(50));
    }
  });

  loop {
    state.tick();
    state.read_input();
    if !state.should_continue() {
      break;
    }
  }
}
