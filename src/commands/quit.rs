use specs::prelude::*;

use crate::run_mode::{ RUN_MODE, RunMode };
use crate::traits::Commandable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct QuitCommand {
  pub entity: Entity,
}

impl Commandable for QuitCommand {
  #[named]
  fn execute(&self) {
    *RUN_MODE.lock().unwrap() = RunMode::Quit;
  }
}
