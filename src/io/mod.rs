use colored::*;
use rustyline::{ Editor, ExternalPrinter};
use specs::prelude::*;
use std::sync::Mutex;
use std::thread::{sleep, spawn};
use std::time::Duration;

use crate::commands::Command;
use crate::ecs::components::*;
use crate::ecs::resources::*;
use crate::queue::get_messages;

pub struct State {
  pub ecs: World,
  pub editor: Editor<()>,
}

lazy_static! {
  pub static ref INPUT: Mutex<Editor::<()>> = Mutex::new(Editor::<()>::new().unwrap());
}

pub fn start_output() {
  let mut printer = INPUT.lock().unwrap().create_external_printer().unwrap();
  let duration = Duration::from_millis(50);
  spawn(move || loop {
    let messages = get_messages();
    if messages.len() > 0 {
      for message in messages.iter() {
        printer.print(format!("{}", message)).expect("External print failure");
      }
      printer.print(format!("{}", " ")).expect("External print failure");
    }
    sleep(duration);
  });
}
