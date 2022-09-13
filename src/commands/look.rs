use specs::prelude::*;

use crate::actions::*;

use crate::traits::Commandable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookCommand {
  pub entity: Entity,
}

impl Commandable for LookCommand {
  #[named]
  fn execute(&self, _ecs: &mut World) {
    enq_action!(act_look!(self.entity));
  }
}
