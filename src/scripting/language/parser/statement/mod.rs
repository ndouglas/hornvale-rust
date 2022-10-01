use std::cell::RefCell;
use std::rc::Rc;

use crate::event::OutputEvent;
use crate::scripting::language::callable::{Callable, CallableKind};
use crate::scripting::language::environment::Environment;
use crate::scripting::language::parser::Expression;
use crate::scripting::language::script_error::ScriptError;
use crate::scripting::language::token::Token;
use crate::scripting::language::value::Value;
use crate::system::systems::process_script::ProcessScriptSystemData;

#[derive(Clone, Debug)]
pub enum Statement {
  If {
    condition: Expression,
    then: Box<Statement>,
    r#else: Option<Box<Statement>>,
  },
  While {
    condition: Expression,
    body: Box<Statement>,
  },
  Block(Vec<Statement>),
  Function {
    name: Token,
    parameters: Vec<Token>,
    body: Box<Statement>,
  },
  Variable {
    name: Token,
    initializer: Option<Expression>,
  },
  Expression(Expression),
  Print(Expression),
  Return {
    token: Token,
    expression: Option<Expression>,
  },
}

impl Statement {
  #[named]
  pub fn evaluate<'a>(
    &self,
    environment: &Rc<RefCell<Environment>>,
    data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<(), ScriptError> {
    use Statement::*;
    match self {
      If {
        condition,
        then,
        r#else,
      } => {
        if condition.evaluate(environment, data)?.is_truthy() {
          then.evaluate(environment, data)?;
        } else if let Some(else_statement) = r#else {
          else_statement.evaluate(environment, data)?;
        }
        Ok(())
      },
      While { condition, body } => {
        while condition.evaluate(environment, data)?.is_truthy() {
          body.evaluate(environment, data)?;
        }
        Ok(())
      },
      Block(statements) => {
        let inner_environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&environment)))));
        for statement in statements {
          if let Err(error) = statement.evaluate(&inner_environment, data) {
            return Err(error);
          }
        }
        Ok(())
      },
      Expression(expression) => match expression.evaluate(environment, data) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
      },
      Function {
        name,
        parameters,
        body: _,
      } => {
        let function = Callable {
          name: name.lexeme.to_string(),
          arity: parameters.len(),
          kind: CallableKind::DeclaredFunction(self.clone()),
          environment: Rc::clone(&environment),
        };
        environment.borrow_mut().define(name, Value::Callable(function));
        Ok(())
      },
      Print(expression) => match expression.evaluate(environment, data) {
        Ok(value) => {
          data.output_event_channel.single_write(OutputEvent {
            string: format!("{}", value),
          });
          Ok(())
        },
        Err(error) => Err(error),
      },
      Return { token, expression } => match expression {
        None => Err(ScriptError::Return {
          token: (*token).clone(),
          value: None,
        }),
        Some(expression) => match expression.evaluate(environment, data) {
          Ok(value) => Err(ScriptError::Return {
            token: token.clone(),
            value: Some(value),
          }),
          Err(error) => Err(error),
        },
      },
      Variable { name, initializer } => {
        let value = match initializer {
          Some(init_expression) => match init_expression.evaluate(environment, data) {
            Ok(expr_result) => expr_result,
            Err(error) => return Err(error),
          },
          None => Value::Nil,
        };
        environment.borrow_mut().define(name, value.clone());
        Ok(())
      },
    }
  }
}
