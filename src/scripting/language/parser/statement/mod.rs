use std::cell::RefCell;
use std::rc::Rc;

use crate::event::OutputEvent;
use crate::scripting::language::callable::{Callable, CallableKind};
use crate::scripting::language::environment::Environment;
use crate::scripting::language::interpreter::Interpreter;
use crate::scripting::language::parser::Expression;
use crate::scripting::language::resolver::FunctionType;
use crate::scripting::language::resolver::Resolver;
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
  pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<(), ScriptError> {
    use Statement::*;
    match self {
      If {
        condition,
        then,
        r#else,
      } => {
        condition.resolve(resolver)?;
        then.resolve(resolver)?;
        if let Some(else_expression) = r#else {
          else_expression.resolve(resolver)?;
        }
        Ok(())
      },
      While { condition, body } => {
        condition.resolve(resolver)?;
        body.resolve(resolver)?;
        Ok(())
      },
      Block(statements) => {
        resolver.begin_scope();
        for statement in statements {
          if let Err(error) = statement.resolve(resolver) {
            resolver.end_scope();
            return Err(error);
          }
        }
        resolver.end_scope();
        Ok(())
      },
      Expression(expression) => expression.resolve(resolver),
      Function { name, parameters, body } => {
        let enclosing_type = resolver.in_function;
        resolver.in_function = FunctionType::Function;
        resolver.declare(name)?;
        resolver.define(name);
        resolver.begin_scope();
        for param in parameters {
          resolver.declare(param)?;
          resolver.define(param);
        }
        body.resolve(resolver)?;
        resolver.end_scope();
        resolver.in_function = enclosing_type;
        Ok(())
      },
      Print(expression) => expression.resolve(resolver),
      Return { expression, token } => {
        if resolver.in_function == FunctionType::None {
          return Err(ScriptError::Error {
            token: Some(token.clone()),
            message: "Cannot return from top-level scope!".to_string(),
          });
        }
        match expression {
          Some(expression_value) => expression_value.resolve(resolver),
          None => Ok(()),
        }
      },
      Variable { name, initializer } => {
        resolver.declare(name)?;
        if let Some(actual_initializer) = initializer {
          if let Err(error) = actual_initializer.resolve(resolver) {
            return Err(error);
          }
        }
        resolver.define(name);
        Ok(())
      },
    }
  }

  #[named]
  pub fn evaluate<'a>(
    &self,
    interpreter: &Interpreter,
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
        if condition.evaluate(interpreter, environment, data)?.is_truthy() {
          then.evaluate(interpreter, environment, data)?;
        } else if let Some(else_statement) = r#else {
          else_statement.evaluate(interpreter, environment, data)?;
        }
        Ok(())
      },
      While { condition, body } => {
        while condition.evaluate(interpreter, environment, data)?.is_truthy() {
          body.evaluate(interpreter, environment, data)?;
        }
        Ok(())
      },
      Block(statements) => {
        let inner_environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&environment)))));
        for statement in statements {
          if let Err(error) = statement.evaluate(interpreter, &inner_environment, data) {
            return Err(error);
          }
        }
        Ok(())
      },
      Expression(expression) => match expression.evaluate(interpreter, environment, data) {
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
        environment.borrow_mut().define(name, &Value::Callable(function));
        Ok(())
      },
      Print(expression) => match expression.evaluate(interpreter, environment, data) {
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
        Some(expression) => match expression.evaluate(interpreter, environment, data) {
          Ok(value) => Err(ScriptError::Return {
            token: token.clone(),
            value: Some(value),
          }),
          Err(error) => Err(error),
        },
      },
      Variable { name, initializer } => {
        let value = match initializer {
          Some(init_expression) => match init_expression.evaluate(interpreter, environment, data) {
            Ok(expr_result) => expr_result,
            Err(error) => return Err(error),
          },
          None => Value::Nil,
        };
        environment.borrow_mut().define(name, &value);
        Ok(())
      },
    }
  }
}
