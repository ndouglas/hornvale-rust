use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::Direction;
use crate::queue::enqueue_action;
use crate::queue::enqueue_message;
use crate::traits::Actionable;
use crate::traits::Commandable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct EchoCommand {
  pub entity: Entity,
  pub string: String,
}

impl Commandable for EchoCommand {
  #[named]
  fn execute(&self, _ecs: &mut World) {
    trace_enter!();
    enqueue_message(format!("{}", self.string));
    trace_exit!();
  }
}
