use colored::*;
use specs::prelude::*;

use crate::effects::Effect;
use crate::traits::Effectable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PrintErrorEffect {
  pub message: String,
}

impl Effectable for PrintErrorEffect {
  #[named]
  fn execute(&self, _ecs: &mut World) {
    enq_message!(format!("{}", self.message.red()))
  }
}
