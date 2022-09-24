use std::io::{Error, ErrorKind};

use crate::scripting::language::interpreter::Interpreter;
use crate::scripting::language::token::{Token, TokenLiteral, TokenType};
use crate::scripting::language::value::Value;

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

impl Expression {
  #[named]
  pub fn print_ast(&self) -> String {
    use Expression::*;
    match self {
      Assignment { identifier, value } => self.parenthesize(&identifier.lexeme, &vec![(*value).clone()]),
      Binary { left, operator, right } => self.parenthesize(&operator.lexeme, &vec![(*left).clone(), (*right).clone()]),
      Grouping { expression } => self.parenthesize("group", &vec![(*expression).clone()]),
      Literal { value } => match value {
        Some(inner_value) => format!("{}", inner_value),
        None => "nil".to_string(),
      },
      Logical { left, operator, right } => self.parenthesize(&operator.lexeme, &vec![(*left).clone(), (*right).clone()]),
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
    _interpreter: &mut Interpreter,
    value_option: &Option<TokenLiteral>,
  ) -> Result<Value, Error> {
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
    interpreter: &mut Interpreter,
    operator: &Token,
    right: &Expression,
  ) -> Result<Value, Error> {
    let right_value = right.evaluate(interpreter);
    let result = match operator.r#type {
      TokenType::Minus => match right_value {
        Ok(Value::Number(value)) => Ok(Value::Number(-(value as f64))),
        Ok(other) => Err(Error::new(
          ErrorKind::Other,
          format!("Operand for `-` must be a number, not {:?}!", other),
        )),
        err => err,
      },
      TokenType::Bang => match right_value {
        Ok(inner) => Ok(Value::Boolean(!inner.is_truthy())),
        err => err,
      },
      _ => Err(Error::new(
        ErrorKind::Other,
        format!("Bad unary operator {:?} for value {:?}!", operator, right_value),
      )),
    };
    debug!("{:?} {:?} => {:?}", operator, right, result);
    result
  }

  #[named]
  pub fn evaluate_binary_math(
    &self,
    _interpreter: &mut Interpreter,
    operator: &Token,
    x: f64,
    y: f64,
  ) -> Result<Value, Error> {
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
      _ => Err(Error::new(
        ErrorKind::Other,
        format!("Bad operator {:?} for binary expression!", operator),
      )),
    };
    debug!("{:?} {:?} {:?} => {:?}", x, operator, y, result);
    result
  }

  #[named]
  pub fn evaluate_binary_string(
    &self,
    _interpreter: &mut Interpreter,
    operator: &Token,
    x: String,
    y: String,
  ) -> Result<Value, Error> {
    let result = match operator.r#type {
      TokenType::Plus => Ok(Value::String(format!("{}{}", x, y))),
      TokenType::GreaterThan => Ok(Value::Boolean(x.gt(&y))),
      TokenType::GreaterThanOrEqual => Ok(Value::Boolean(x.ge(&y))),
      TokenType::LessThan => Ok(Value::Boolean(x.lt(&y))),
      TokenType::LessThanOrEqual => Ok(Value::Boolean(x.le(&y))),
      TokenType::BangEqual => Ok(Value::Boolean(x.ne(&y))),
      TokenType::EqualEqual => Ok(Value::Boolean(x.eq(&y))),
      _ => Err(Error::new(
        ErrorKind::Other,
        format!("Bad operator {:?} for string operands {:?} and {:?}!", operator, x, y),
      )),
    };
    debug!("{:?} {:?} {:?} => {:?}", x, operator, y, result);
    result
  }

  #[named]
  pub fn evaluate_binary(
    &self,
    interpreter: &mut Interpreter,
    left: &Expression,
    operator: &Token,
    right: &Expression,
  ) -> Result<Value, Error> {
    let left_value = left.evaluate(interpreter);
    let right_value = right.evaluate(interpreter);
    let result = match (left_value, right_value) {
      (Ok(Value::Number(x)), Ok(Value::Number(y))) => self.evaluate_binary_math(interpreter, operator, x, y),
      (Ok(Value::String(x)), Ok(Value::String(y))) => self.evaluate_binary_string(interpreter, operator, x, y),
      _ => Err(Error::new(
        ErrorKind::Other,
        format!("Bad operands ({:?} and {:?}) for operator {:?}!", left, right, operator),
      )),
    };
    debug!("{:?} {:?} {:?} => {:?}", left, operator, right, result);
    result
  }

  #[named]
  pub fn evaluate_logical(
    &self,
    interpreter: &mut Interpreter,
    left: &Expression,
    operator: &Token,
    right: &Expression,
  ) -> Result<Value, Error> {
    let left_value = left.evaluate(interpreter)?;
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
      Ok(right.evaluate(interpreter)?)
    };
    debug!("{:?} {:?} {:?} => {:?}", left, operator, right, result);
    result
  }

  #[named]
  pub fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Value, Error> {
    use Expression::*;
    info!("Abstract Syntax Tree: {}", self.print_ast());
    let result = match self {
      Assignment { identifier, value } => {
        let final_value = &value.evaluate(interpreter)?;
        interpreter.environment.assign(identifier, final_value)
      },
      Literal { value: value_option } => self.evaluate_literal(interpreter, value_option),
      Logical { left, operator, right } => self.evaluate_logical(interpreter, left, operator, right),
      Grouping { expression } => expression.evaluate(interpreter),
      Unary { operator, right } => self.evaluate_unary(interpreter, operator, right),
      Binary { left, operator, right } => self.evaluate_binary(interpreter, left, operator, right),
      Variable { identifier } => interpreter.environment.get(identifier),
    };
    debug!("{:?} => {:?}", self, result);
    result
  }
}
