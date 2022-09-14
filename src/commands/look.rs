use specs::prelude::*;



use super::Commandable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookCommand {
  pub entity: Entity,
}

impl Commandable for LookCommand {
  #[named]
  fn execute(&self) {
    enq_action!(act_look!(self.entity));
  }
}
