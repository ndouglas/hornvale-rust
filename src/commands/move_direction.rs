use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::Direction;
use crate::queue::enqueue_action;
use crate::traits::Actionable;
use crate::traits::Commandable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct MoveDirectionCommand {
  pub entity: Entity,
  pub direction: Direction,
}

impl Commandable for MoveDirectionCommand {
  #[named]
  fn execute(&self, _ecs: &mut World) {
    enq_action!(act_move_direction!(self.entity, self.direction.clone()));
  }
}
