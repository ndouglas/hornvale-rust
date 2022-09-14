use specs::prelude::*;


use crate::model::Direction;

use super::Commandable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct MoveDirectionCommand {
  pub entity: Entity,
  pub direction: Direction,
}

impl Commandable for MoveDirectionCommand {
  #[named]
  fn execute(&self) {
    enq_action!(act_move_direction!(self.entity, self.direction.clone()));
  }
}
