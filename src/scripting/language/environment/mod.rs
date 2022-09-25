use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use crate::scripting::language::token::Token;
use crate::scripting::language::value::Value;

#[derive(Debug)]
pub struct Environment {
  pub values: HashMap<String, Value>,
  pub parent: Option<Box<Environment>>,
}

impl Environment {
  #[named]
  pub fn new(parent: Option<Box<Environment>>) -> Self {
    Self {
      values: HashMap::new(),
      parent,
    }
  }

  #[named]
  pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), Error> {
    let actual_name = &name.lexeme.to_string();
    if !self.values.contains_key(actual_name) {
      if let Some(parent_box) = self.parent.as_mut() {
        let parent = &mut *parent_box;
        return parent.assign(name, value);
      }
      Err(Error::new(
        ErrorKind::Other,
        format!("Undefined variable '{}'!", name.lexeme),
      ))
    } else {
      self.define(actual_name, value);
      Ok(())
    }
  }

  #[named]
  pub fn define(&mut self, name: &str, value: Value) {
    self.values.insert(name.to_string(), value.clone());
  }

  #[named]
  pub fn get(&self, name: &Token) -> Result<Value, Error> {
    match self.values.get(&name.lexeme.to_string()) {
      Some(value) => Ok(value.clone()),
      None => match &self.parent {
        Some(parent) => parent.get(name),
        None => Err(Error::new(
          ErrorKind::Other,
          format!("Undefined variable '{}'!", name.lexeme),
        )),
      },
    }
  }
}
