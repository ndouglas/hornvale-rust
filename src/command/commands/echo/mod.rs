use crate::entity::Entity;

use super::super::Commandable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct EchoCommand {
  pub entity: Entity,
  pub string: String,
}

impl Commandable for EchoCommand {
  #[named]
  fn execute(&self) {
    enq_message!(format!("{}", self.string));
  }
}
