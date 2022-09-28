use std::collections::HashMap;

use crate::scripting::language::script_error::ScriptError;
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
  pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), ScriptError> {
    let actual_name = &name.lexeme.to_string();
    if !self.values.contains_key(actual_name) {
      if let Some(parent_box) = self.parent.as_mut() {
        let parent = &mut *parent_box;
        return parent.assign(name, value);
      }
      Err(ScriptError::Error {
        token: Some(name.clone()),
        message: format!("Undefined variable '{}'!", name.lexeme),
      })
    } else {
      self.define(name, value);
      Ok(())
    }
  }

  #[named]
  pub fn define(&mut self, name: &Token, value: Value) {
    self.values.insert(name.lexeme.to_string(), value.clone());
  }

  #[named]
  pub fn define_global(&mut self, name: &Token, value: Value) {
    if let Some(parent_box) = self.parent.as_mut() {
      let parent = &mut *parent_box;
      parent.define_global(name, value)
    } else {
      self.define(name, value)
    }
  }

  #[named]
  pub fn get(&self, name: &Token) -> Result<Value, ScriptError> {
    match self.values.get(&name.lexeme.to_string()) {
      Some(value) => Ok(value.clone()),
      None => match &self.parent {
        Some(parent) => parent.get(name),
        None => Err(ScriptError::Error {
          token: Some(name.clone()),
          message: format!("Undefined variable '{}'!", name.lexeme),
        }),
      },
    }
  }
}
