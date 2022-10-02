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
  pub fn assign(&mut self, name: &Token, value: &Value) -> Result<(), ScriptError> {
    if self.values.contains_key(&name.lexeme.to_string()) {
      self.define(name, value);
      Ok(())
    } else {
      Err(ScriptError::Error {
        token: Some(name.clone()),
        message: format!("Undefined variable '{}'!", name.lexeme),
      })
    }
  }

  #[named]
  pub fn define(&mut self, name: &Token, value: &Value) {
    self.values.insert(name.lexeme.to_string(), value.clone());
  }

  #[named]
  pub fn get(&self, name: &Token) -> Result<Value, ScriptError> {
    match self.values.get(&name.lexeme.to_string()) {
      Some(value) => Ok(value.clone()),
      None => Err(ScriptError::Error {
        token: Some(name.clone()),
        message: format!("Undefined variable '{}'!", name.lexeme),
      }),
    }
  }

  #[named]
  pub fn get_ancestor(&self, distance: usize) -> Option<Rc<RefCell<Environment>>> {
    let mut result = match self.parent {
      Some(ref environment) => environment.clone(),
      None => return None,
    };
    for _ in 1..distance {
      let found = match result.borrow().parent {
        Some(ref environment) => environment.clone(),
        None => return None,
      };
      result = found;
    }
    Some(result)
  }

  #[named]
  pub fn get_at(&self, distance: usize, name: &Token) -> Result<Value, ScriptError> {
    if distance == 0 {
      return self.get(name);
    }
    let ancestor = self.get_ancestor(distance);
    match ancestor {
      Some(environment) => environment.borrow().get(name),
      None => Err(ScriptError::Error {
        token: Some(name.clone()),
        message: format!("Undefined variable '{}'!", name.lexeme),
      }),
    }
  }

  #[named]
  pub fn assign_at(&mut self, distance: usize, name: &Token, value: &Value) -> Result<(), ScriptError> {
    let ancestor = self.get_ancestor(distance);
    match ancestor {
      Some(environment) => environment.borrow_mut().assign(name, value)?,
      None => self.assign(name, value)?,
    };
    Ok(())
  }
}
