use std::io::Error;

use crate::scripting::language::parser::Expression;
use crate::scripting::language::ScriptingLanguage;

#[derive(Clone, Debug)]
pub enum Statement {
  Expression(Expression),
  Print(Expression),
}

impl Statement {
  #[named]
  pub fn evaluate(&self, owner: &mut ScriptingLanguage) -> Result<(), Error> {
    use Statement::*;
    match self {
      Expression(expression) => match expression.evaluate() {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
      },
      Print(expression) => match expression.evaluate() {
        Ok(value) => owner.print_value(&value),
        Err(error) => Err(error),
      },
    } 
  }
}
