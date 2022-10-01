use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::scripting::language::script_error::ScriptError;
use crate::scripting::language::token::Token;
use crate::scripting::language::value::Value;

#[derive(Debug, Default)]
pub struct Environment {
  pub values: HashMap<String, Value>,
  pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
  #[named]
  pub fn new(parent: Option<Rc<RefCell<Environment>>>) -> Self {
    Self {
      values: HashMap::new(),
      parent,
    }
  }

  #[named]
  pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), ScriptError> {
    let actual_name = &name.lexeme.to_string();
    if !self.values.contains_key(actual_name) {
      match self.parent {
        Some(ref mut parent) => parent.borrow_mut().assign(name, value),
        None => Err(ScriptError::Error {
          token: Some(name.clone()),
          message: format!("Undefined variable '{}'!", name.lexeme),
        }),
      }
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
    match self.parent {
      Some(ref mut parent) => parent.borrow_mut().define_global(name, value),
      None => self.define(name, value),
    }
  }

  #[named]
  pub fn get(&self, name: &Token) -> Result<Value, ScriptError> {
    match self.values.get(&name.lexeme.to_string()) {
      Some(value) => Ok(value.clone()),
      None => match self.parent {
        Some(ref parent) => parent.borrow().get(name),
        None => Err(ScriptError::Error {
          token: Some(name.clone()),
          message: format!("Undefined variable '{}'!", name.lexeme),
        }),
      },
    }
  }
}
