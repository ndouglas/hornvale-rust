use colored::*;

use super::super::Effectable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PrintErrorEffect {
  pub message: String,
}

impl Effectable for PrintErrorEffect {
  #[named]
  fn execute(&self) {
    enq_message!(format!("{}", self.message.red()))
  }
}
