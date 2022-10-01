use crate::event::OutputEvent;
use crate::scripting::language::callable::{Callable, CallableKind};
use crate::scripting::language::interpreter::Interpreter;
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
    interpreter: &mut Interpreter,
    data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<(), ScriptError> {
    use Statement::*;
    match self {
      If {
        condition,
        then,
        r#else,
      } => {
        if condition.evaluate(interpreter, data)?.is_truthy() {
          then.evaluate(interpreter, data)?;
        } else if let Some(else_statement) = r#else {
          else_statement.evaluate(interpreter, data)?;
        }
        Ok(())
      },
      While { condition, body } => {
        while condition.evaluate(interpreter, data)?.is_truthy() {
          body.evaluate(interpreter, data)?;
        }
        Ok(())
      },
      Block(statements) => {
        interpreter.push_env();
        for statement in statements {
          if let Err(error) = statement.evaluate(interpreter, data) {
            interpreter.pop_env();
            return Err(error);
          }
        }
        interpreter.pop_env();
        Ok(())
      },
      Expression(expression) => match expression.evaluate(interpreter, data) {
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
        };
        interpreter.environment.borrow_mut().define(name, Value::Callable(function));
        Ok(())
      },
      Print(expression) => match expression.evaluate(interpreter, data) {
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
        Some(expression) => match expression.evaluate(interpreter, data) {
          Ok(value) => Err(ScriptError::Return {
            token: token.clone(),
            value: Some(value),
          }),
          Err(error) => Err(error),
        },
      },
      Variable { name, initializer } => {
        let value = match initializer {
          Some(init_expression) => match init_expression.evaluate(interpreter, data) {
            Ok(expr_result) => expr_result,
            Err(error) => return Err(error),
          },
          None => Value::Nil,
        };
        interpreter.environment.borrow_mut().define(name, value.clone());
        Ok(())
      },
    }
  }
}
