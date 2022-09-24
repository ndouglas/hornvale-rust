use std::io::Error;

use crate::scripting::language::interpreter::Interpreter;
use crate::scripting::language::parser::Expression;
use crate::scripting::language::token::Token;
use crate::scripting::language::value::Value;

#[derive(Clone, Debug)]
pub enum Statement {
  If {
    condition: Expression,
    then: Box<Statement>,
    r#else: Option<Box<Statement>>,
  },
  Block(Vec<Statement>),
  Variable {
    name: Token,
    initializer: Option<Expression>,
  },
  Expression(Expression),
  Print(Expression),
}

impl Statement {
  #[named]
  pub fn evaluate(&self, interpreter: &mut Interpreter) -> Result<(), Error> {
    use Statement::*;
    match self {
      If { condition, then, r#else } => {
        error!("{:?}", condition);
        if condition.evaluate(interpreter)?.is_truthy() {
          then.evaluate(interpreter)?;
        } else if let Some(else_statement) = r#else {
          else_statement.evaluate(interpreter)?;
        }
        Ok(())
      }
      Block(statements) => {
        interpreter.push_env();
        for statement in statements {
          statement.evaluate(interpreter)?;
        }
        interpreter.pop_env();
        Ok(())
      },
      Expression(expression) => match expression.evaluate(interpreter) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
      },
      Print(expression) => match expression.evaluate(interpreter) {
        Ok(value) => interpreter.owner.print_value(&value),
        Err(error) => Err(error),
      },
      Variable { name, initializer } => {
        let value = match initializer {
          Some(init_expression) => match init_expression.evaluate(interpreter) {
            Ok(expr_result) => expr_result,
            Err(error) => return Err(error),
          },
          None => Value::Nil,
        };
        interpreter.environment.define(&name.lexeme, &value);
        Ok(())
      },
    }
  }
}
