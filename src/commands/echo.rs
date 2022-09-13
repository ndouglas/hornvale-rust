use specs::prelude::*;

use crate::traits::Commandable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct EchoCommand {
  pub entity: Entity,
  pub string: String,
}

impl Commandable for EchoCommand {
  #[named]
  fn execute(&self, _ecs: &mut World) {
    enq_message!(format!("{}", self.string));
  }
}
