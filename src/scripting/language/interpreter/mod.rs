use std::io::Error;

use crate::scripting::language::ScriptingLanguage;
use crate::scripting::language::parser::statement::Statement;

#[derive(Debug)]
pub struct Interpreter<'a> {
  pub owner: &'a mut ScriptingLanguage,
}

impl<'a> Interpreter<'a> {
  pub fn new(owner: &'a mut ScriptingLanguage) -> Self {
    Self {
      owner,
    }
  }

  pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), Error> {
    for statement in statements {
      if let Err(error) = statement.evaluate(&mut self.owner) {
        return Err(error);
      }
    }
    Ok(())
  }
}
