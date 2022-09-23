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
  pub fn set(&mut self, name: &str, value: &Value) {
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
