use specs::prelude::*;

use crate::command::Command;
use crate::resource::ShouldContinueResource;

use super::*;

impl<'a> ProcessCommandSystem {
  pub fn process_echo(&mut self, command: &Command) {
    if let Command::Echo { string, .. } = command {
      println!("{}", string);
    }
  }
}
