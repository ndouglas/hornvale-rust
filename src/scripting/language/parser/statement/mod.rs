use std::io::Error;

use crate::scripting::language::environment::Environment;
use crate::scripting::language::parser::Expression;
use crate::scripting::language::ScriptingLanguage;
use crate::scripting::language::token::Token;
use crate::scripting::language::value::Value;

#[derive(Clone, Debug)]
pub enum Statement {
  Variable {
    name: Token,
    initializer: Option<Expression>,
  },
  Expression(Expression),
  Print(Expression),
}

impl Statement {
  #[named]
  pub fn evaluate(&self, owner: &mut ScriptingLanguage, environment: &mut Environment) -> Result<(), Error> {
    use Statement::*;
    match self {
      Expression(expression) => match expression.evaluate(environment) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
      },
      Print(expression) => match expression.evaluate(environment) {
        Ok(value) => owner.print_value(&value),
        Err(error) => Err(error),
      },
      Variable { name, initializer } => {
        let value = match initializer {
          Some(init_expression) => {
            match init_expression.evaluate(environment) {
              Ok(expr_result) => expr_result,
              Err(error) => return Err(error),
            }
          }
          None => Value::Nil,
        };
        environment.define(&name.lexeme, &value);
        Ok(())
      },
    } 
  }
}
