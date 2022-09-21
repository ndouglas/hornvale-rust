use std::io::{Error, ErrorKind};

use crate::scripting::language::token::{Token, TokenLiteral, TokenType};
use crate::scripting::language::value::Value;

#[derive(Clone, Debug)]
pub enum Expression {
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
  Unary {
    operator: Token,
    right: Box<Expression>,
  },
}

impl Expression {

  pub fn print_ast(&self) -> String {
    use Expression::*;
    match self {
      Binary { left, operator, right } => self.parenthesize(&operator.lexeme, &vec![(*left).clone(), (*right).clone()]),
      Grouping { expression } => self.parenthesize("group", &vec![(*expression).clone()]),
      Literal { value } => match value {
        Some(inner_value) => format!("{}", inner_value),
        None => "nil".to_string(),
      },
      Unary { operator, right } => self.parenthesize(&operator.lexeme, &vec![(*right).clone()]),
    }
  }

  pub fn parenthesize(&self, first: &str, rest: &Vec<Box<Expression>>) -> String {
    let mut result = String::default();
    result.push_str(&format!("({}", first));
    for expression in rest.iter() {
      result.push_str(&format!(" {}", expression.print_ast()));
    }
    result.push_str(")");
    result
  }

  pub fn evaluate_literal(&self, value_option: &Option<TokenLiteral>) -> Result<Value, Error> {
    match value_option {
      Some(TokenLiteral::Number(number)) => Ok(Value::Number(*number)),
      Some(TokenLiteral::String(string)) => Ok(Value::String(string.to_string())),
      None => Ok(Value::Nil),
    }
  }

  pub fn evaluate_unary(&self, operator: &Token, right: &Expression) -> Result<Value, Error> {
    let right_value = right.get_value();
    let result = match operator.r#type {
      TokenType::Minus => match right_value {
        Ok(Value::Number(value)) => Ok(Value::Number(-(value as f64))),
        Ok(other) => Err(Error::new(ErrorKind::Other, format!("Operand for `-` must be a number, not {:?}!", other))),
        err => err,
      },
      TokenType::Bang => match right_value {
        Ok(inner) => Ok(Value::Boolean(!inner.get_truthiness())),
        err => err,
      },
      _ => Err(Error::new(ErrorKind::Other, format!("Bad unary operator {:?} for value {:?}!", operator, right_value))),
    };
    debug!("{:?} {:?} => {:?}", operator, right, result);
    result
  }

  pub fn evaluate_binary_math(&self, operator: &Token, x: f64, y: f64) -> Result<Value, Error> {
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
      _ => Err(Error::new(ErrorKind::Other, format!("Bad operator {:?} for binary expression!", operator))),
    };
    debug!("{:?} {:?} {:?} => {:?}", x, operator, y, result);
    result
  }

  pub fn evaluate_binary_string(&self, operator: &Token, x: String, y: String) -> Result<Value, Error> {
    let result = match operator.r#type {
      TokenType::Plus => Ok(Value::String(format!("{}{}", x, y))),
      TokenType::GreaterThan => Ok(Value::Boolean(x.gt(&y))),
      TokenType::GreaterThanOrEqual => Ok(Value::Boolean(x.ge(&y))),
      TokenType::LessThan => Ok(Value::Boolean(x.lt(&y))),
      TokenType::LessThanOrEqual => Ok(Value::Boolean(x.le(&y))),
      TokenType::BangEqual => Ok(Value::Boolean(x.ne(&y))),
      TokenType::EqualEqual => Ok(Value::Boolean(x.eq(&y))),
      _ => Err(Error::new(ErrorKind::Other, format!("Bad operator {:?} for string operands {:?} and {:?}!", operator, x, y))),
    };
    debug!("{:?} {:?} {:?} => {:?}", x, operator, y, result);
    result
  }
  pub fn evaluate_binary(&self, left: &Expression, operator: &Token, right: &Expression) -> Result<Value, Error> {
    let left_value = left.get_value();
    let right_value = right.get_value();
    let result = match (left_value, right_value) {
      (Ok(Value::Number(x)), Ok(Value::Number(y))) => self.evaluate_binary_math(operator, x, y),
      (Ok(Value::String(x)), Ok(Value::String(y))) => self.evaluate_binary_string(operator, x, y),
      _ => Err(Error::new(ErrorKind::Other, format!("Bad operands ({:?} and {:?}) for operator {:?}!", left, right, operator))),
    };
    debug!("{:?} {:?} {:?} => {:?}", left, operator, right, result);
    result
  }
    
  pub fn get_value(&self) -> Result<Value, Error>  {
    use Expression::*;
    let result = match self {
      Literal { value: value_option } => self.evaluate_literal(value_option),
      Grouping { expression } => expression.get_value(),
      Unary { operator, right } => self.evaluate_unary(operator, right),
      Binary { left, operator, right } => self.evaluate_binary(left, operator, right),
    };
    debug!("{:?} => {:?}", self, result);
    result
  }

}