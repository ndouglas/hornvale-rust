use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::Direction;
use crate::queue::enqueue_action;
use crate::traits::Actionable;
use crate::traits::Commandable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveDirectionCommand {
  pub entity: Entity,
  pub compass_direction: Direction,
}

impl Commandable for MoveDirectionCommand {
  #[named]
  fn execute(&self, ecs: &mut World) {
    trace_enter!();
    let action = Action::MoveDirection(MoveDirectionAction {
      entity: self.entity,
      compass_direction: self.compass_direction,
    });
    enqueue_action(action);
    trace_exit!();
  }
}
