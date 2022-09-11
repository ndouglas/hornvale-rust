use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::Direction;
use crate::queue::enqueue_action;
use crate::traits::Actionable;
use crate::traits::Commandable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookCommand {
  pub entity: Entity,
}

impl Commandable for LookCommand {
  #[named]
  fn execute(&self, _ecs: &mut World) {
    trace_enter!();
    let action = Action::Look(LookAction { entity: self.entity });
    enqueue_action(action);
    trace_exit!();
  }
}
