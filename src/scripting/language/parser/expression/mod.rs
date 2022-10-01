use std::cell::RefCell;
use std::rc::Rc;

use crate::scripting::language::environment::Environment;
use crate::scripting::language::script_error::ScriptError;
use crate::scripting::language::token::{Token, TokenLiteral, TokenType};
use crate::scripting::language::value::Value;
use crate::system::systems::process_script::ProcessScriptSystemData;

#[derive(Clone, Debug)]
pub enum Expression {
  Assignment {
    identifier: Token,
    value: Box<Expression>,
  },
  Binary {
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
  },
  Call {
    callee: Box<Expression>,
    closing_parenthesis: Token,
    arguments: Vec<Expression>,
  },
  Grouping {
    expression: Box<Expression>,
  },
  Literal {
    value: Option<TokenLiteral>,
  },
  Logical {
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
  },
  Unary {
    operator: Token,
    right: Box<Expression>,
  },
  Variable {
    identifier: Token,
  },
}

impl<'a> Expression {
  #[named]
  pub fn print_ast(&self) -> String {
    use Expression::*;
    match self {
      Assignment { identifier, value } => self.parenthesize(&identifier.lexeme, &vec![(*value).clone()]),
      Binary { left, operator, right } => self.parenthesize(&operator.lexeme, &vec![(*left).clone(), (*right).clone()]),
      Call {
        callee: _, arguments, ..
      } => self.parenthesize(
        "function",
        &arguments.iter().map(|arg| Box::new((*arg).clone())).collect(),
      ),
      Grouping { expression } => self.parenthesize("group", &vec![(*expression).clone()]),
      Literal { value } => match value {
        Some(inner_value) => format!("{}", inner_value),
        None => "nil".to_string(),
      },
      Logical { left, operator, right } => {
        self.parenthesize(&operator.lexeme, &vec![(*left).clone(), (*right).clone()])
      },
      Unary { operator, right } => self.parenthesize(&operator.lexeme, &vec![(*right).clone()]),
      Variable { identifier } => identifier.lexeme.to_string(),
    }
  }

  #[named]
  pub fn parenthesize(&self, first: &str, rest: &Vec<Box<Expression>>) -> String {
    let mut result = String::default();
    result.push_str(&format!("({}", first));
    for expression in rest.iter() {
      result.push_str(&format!(" {}", expression.print_ast()));
    }
    result.push_str(")");
    result
  }

  #[named]
  pub fn evaluate_literal(
    &self,
    _environment: &Rc<RefCell<Environment>>,
    value_option: &Option<TokenLiteral>,
    _data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<Value, ScriptError> {
    match value_option {
      Some(TokenLiteral::Boolean(boolean)) => Ok(Value::Boolean(*boolean)),
      Some(TokenLiteral::Nil) => Ok(Value::Nil),
      Some(TokenLiteral::Number(number)) => Ok(Value::Number(*number)),
      Some(TokenLiteral::String(string)) => Ok(Value::String(string.to_string())),
      None => Ok(Value::Nil),
    }
  }

  #[named]
  pub fn evaluate_unary(
    &self,
    environment: &Rc<RefCell<Environment>>,
    operator: &Token,
    right: &Expression,
    data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<Value, ScriptError> {
    let right_value = right.evaluate(environment, data);
    let result = match operator.r#type {
      TokenType::Minus => match right_value {
        Ok(Value::Number(value)) => Ok(Value::Number(-(value as f64))),
        Ok(other) => Err(ScriptError::Error {
          token: Some((*operator).clone()),
          message: format!("Operand for `-` must be a number, not {:?}!", other),
        }),
        err => err,
      },
      TokenType::Bang => match right_value {
        Ok(inner) => Ok(Value::Boolean(!inner.is_truthy())),
        err => err,
      },
      _ => Err(ScriptError::Error {
        token: Some((*operator).clone()),
        message: format!("Bad unary operator {:?} for value {:?}!", operator, right_value),
      }),
    };
    debug!("{:?} {:?} => {:?}", operator, right, result);
    result
  }

  #[named]
  pub fn evaluate_binary_math(
    &self,
    _environment: &Rc<RefCell<Environment>>,
    operator: &Token,
    x: f64,
    y: f64,
    _data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<Value, ScriptError> {
    let result = match operator.r#type {
      TokenType::Minus => Ok(Value::Number(x - y)),
      TokenType::Slash => Ok(Value::Number(x / y)),
      TokenType::Star => Ok(Value::Number(x * y)),
      TokenType::Plus => Ok(Value::Number(x + y)),
      TokenType::GreaterThan => Ok(Value::Boolean(x > y)),
      TokenType::GreaterThanOrEqual => Ok(Value::Boolean(x >= y)),
      TokenType::LessThan => Ok(Value::Boolean(x < y)),
      TokenType::LessThanOrEqual => Ok(Value::Boolean(x <= y)),
      TokenType::BangEqual => Ok(Value::Boolean(x != y)),
      TokenType::EqualEqual => Ok(Value::Boolean(x == y)),
      _ => Err(ScriptError::Error {
        token: Some((*operator).clone()),
        message: format!("Bad operator {:?} for binary expression!", operator),
      }),
    };
    debug!("{:?} {:?} {:?} => {:?}", x, operator, y, result);
    result
  }

  #[named]
  pub fn evaluate_binary_string(
    &self,
    _environment: &Rc<RefCell<Environment>>,
    operator: &Token,
    x: String,
    y: String,
    _data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<Value, ScriptError> {
    let result = match operator.r#type {
      TokenType::Plus => Ok(Value::String(format!("{}{}", x, y))),
      TokenType::GreaterThan => Ok(Value::Boolean(x.gt(&y))),
      TokenType::GreaterThanOrEqual => Ok(Value::Boolean(x.ge(&y))),
      TokenType::LessThan => Ok(Value::Boolean(x.lt(&y))),
      TokenType::LessThanOrEqual => Ok(Value::Boolean(x.le(&y))),
      TokenType::BangEqual => Ok(Value::Boolean(x.ne(&y))),
      TokenType::EqualEqual => Ok(Value::Boolean(x.eq(&y))),
      _ => Err(ScriptError::Error {
        token: Some((*operator).clone()),
        message: format!("Bad operator {:?} for string operands {:?} and {:?}!", operator, x, y),
      }),
    };
    debug!("{:?} {:?} {:?} => {:?}", x, operator, y, result);
    result
  }

  #[named]
  pub fn evaluate_binary(
    &self,
    environment: &Rc<RefCell<Environment>>,
    left: &Expression,
    operator: &Token,
    right: &Expression,
    data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<Value, ScriptError> {
    let left_value = left.evaluate(environment, data);
    let right_value = right.evaluate(environment, data);
    let result = match (left_value, right_value) {
      (Ok(Value::Number(x)), Ok(Value::Number(y))) => self.evaluate_binary_math(environment, operator, x, y, data),
      (Ok(Value::String(x)), Ok(Value::String(y))) => self.evaluate_binary_string(environment, operator, x, y, data),
      (Ok(Value::String(x)), Ok(y)) => self.evaluate_binary_string(environment, operator, x, format!("{}", y), data),
      (Ok(x), Ok(Value::String(y))) => self.evaluate_binary_string(environment, operator, format!("{}", x), y, data),
      _ => Err(ScriptError::Error {
        token: Some((*operator).clone()),
        message: format!("Bad operands ({:?} and {:?}) for operator {:?}!", left, right, operator),
      }),
    };
    debug!("{:?} {:?} {:?} => {:?}", left, operator, right, result);
    result
  }

  #[named]
  pub fn evaluate_logical(
    &self,
    environment: &Rc<RefCell<Environment>>,
    left: &Expression,
    operator: &Token,
    right: &Expression,
    data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<Value, ScriptError> {
    let left_value = left.evaluate(environment, data)?;
    let result = {
      if operator.r#type == TokenType::Or {
        if left_value.is_truthy() {
          return Ok(left_value);
        }
      } else {
        if !left_value.is_truthy() {
          return Ok(left_value);
        }
      }
      Ok(right.evaluate(environment, data)?)
    };
    debug!("{:?} {:?} {:?} => {:?}", left, operator, right, result);
    result
  }

  #[named]
  pub fn evaluate_call(
    &self,
    environment: &Rc<RefCell<Environment>>,
    callee: &Expression,
    arguments: &[Expression],
    closing_parenthesis: &Token,
    data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<Value, ScriptError> {
    let callee_value = callee.evaluate(environment, data)?;
    let result = {
      let mut argument_values = Vec::new();
      for argument in arguments.iter() {
        argument_values.push(argument.evaluate(environment, data)?);
      }
      if let Value::Callable(callable) = callee_value.clone() {
        if argument_values.len() != callable.arity {
          return Err(ScriptError::Error {
            token: Some((*closing_parenthesis).clone()),
            message: format!(
              "Expected {} arguments but got {}.",
              callable.arity,
              argument_values.len()
            ),
          });
        }
        let response = callable.call(data, &argument_values);
        match response {
          Err(ScriptError::Return {
            value: value_option, ..
          }) => match value_option {
            Some(value) => value,
            None => Value::Nil,
          },
          Err(error) => return Err(error),
          Ok(value) => value,
        }
      } else {
        return Err(ScriptError::Error {
          token: Some((*closing_parenthesis).clone()),
          message: "Can only call functions and classes.".to_string(),
        });
      }
    };
    debug!("{:?}({:?}) => {:?}", callee_value, arguments, result);
    Ok(result)
  }

  #[named]
  pub fn evaluate(
    &self,
    environment: &Rc<RefCell<Environment>>,
    data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<Value, ScriptError> {
    use Expression::*;
    info!("Abstract Syntax Tree: {}", self.print_ast());
    let result = match self {
      Assignment { identifier, value } => {
        let final_value = &value.evaluate(environment, data)?;
        environment.borrow_mut().assign(identifier, (*final_value).clone())?;
        Ok(Value::Nil)
      },
      Literal { value: value_option } => self.evaluate_literal(environment, value_option, data),
      Logical { left, operator, right } => self.evaluate_logical(environment, left, operator, right, data),
      Grouping { expression } => expression.evaluate(environment, data),
      Unary { operator, right } => self.evaluate_unary(environment, operator, right, data),
      Binary { left, operator, right } => self.evaluate_binary(environment, left, operator, right, data),
      Call {
        callee,
        arguments,
        closing_parenthesis,
      } => self.evaluate_call(environment, callee, arguments, closing_parenthesis, data),
      Variable { identifier } => environment.borrow().get(identifier),
    };
    debug!("{:?} => {:?}", self, result);
    result
  }
}
