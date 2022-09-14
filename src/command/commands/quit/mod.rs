use specs::prelude::*;

use crate::run_mode::{RunMode, RUN_MODE};

use super::super::Commandable;

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
