use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use crate::scripting::language::value::Value;
use crate::scripting::language::token::Token;

#[derive(Clone, Debug)]
pub struct Environment {
  pub values: HashMap<String, Value>,
}

impl Environment {
  #[named]
  pub fn new() -> Self {
    Self {
      values: HashMap::new(),
    }
  }

  #[named]
  pub fn assign(&mut self, name: &Token, value: &Value) -> Result<Value, Error> {
    let actual_name = &name.lexeme.to_string();
    if !self.values.contains_key(actual_name) {
      Err(Error::new(ErrorKind::Other, format!("Undefined variable '{}'!", name.lexeme)))
    } else {
      self.define(actual_name, value);
      Ok(value.clone())
    }
  }

  #[named]
  pub fn define(&mut self, name: &str, value: &Value) {
    self.values.insert(name.to_string(), value.clone());
  }

  #[named]
  pub fn get(&self, name: &Token) -> Result<Value, Error> {
    match self.values.get(&name.lexeme.to_string()) {
      Some(value) => Ok(value.clone()),
      None => Err(Error::new(ErrorKind::Other, format!("Undefined variable '{}'!", name.lexeme))),
    }
  }

}
