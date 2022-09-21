use std::io::Error;

use crate::scripting::language::parser::expression::Expression;
use crate::scripting::language::value::Value;


#[derive(Clone, Debug)]
pub struct Interpreter {
}

impl Interpreter {

  pub fn new() -> Self {
    Self {
      
    }
  }

  pub fn interpret(&self, expression: Expression) -> Result<Value, Error> {
    expression.get_value()
  }

}
