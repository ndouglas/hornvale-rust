use std::collections::HashMap;

use crate::scripting::language::interpreter::Interpreter;
use crate::scripting::language::parser::statement::Statement;
use crate::scripting::language::script_error::ScriptError;
use crate::scripting::language::token::Token;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum FunctionType {
  None,
  Function,
}

#[derive(Debug)]
pub struct Resolver<'a> {
  pub interpreter: &'a mut Interpreter,
  pub scopes: Vec<HashMap<String, bool>>,
  pub in_function: FunctionType,
}

impl<'a> Resolver<'a> {
  #[named]
  pub fn new(interpreter: &'a mut Interpreter) -> Self {
    let scopes = Vec::new();
    let in_function = FunctionType::None;
    Self {
      interpreter,
      scopes,
      in_function,
    }
  }

  #[named]
  pub fn begin_scope(&mut self) {
    self.scopes.push(HashMap::new());
  }

  #[named]
  pub fn end_scope(&mut self) {
    self.scopes.pop();
  }

  #[named]
  pub fn is_only_declared(&self, name: &Token) -> bool {
    if let Some(last) = self.scopes.last() {
      if let Some(value) = last.get(&name.lexeme.to_string()) {
        return *value == false;
      }
    }
    false
  }

  #[named]
  pub fn declare(&mut self, name: &Token) -> Result<(), ScriptError> {
    if let Some(last) = self.scopes.last_mut() {
      if last.contains_key(&name.lexeme.to_string()) {
        return Err(ScriptError::Error {
          token: Some(name.clone()),
          message: "This scope already contains a variable with this name!".to_string(),
        });
      }
      last.insert(name.lexeme.to_string(), false);
    }
    Ok(())
  }

  #[named]
  pub fn define(&mut self, name: &Token) {
    if let Some(last) = self.scopes.last_mut() {
      last.insert(name.lexeme.to_string(), true);
    }
  }

  #[named]
  pub fn resolve(&mut self, statements: &mut Vec<Statement>) -> Result<(), ScriptError> {
    for statement in statements {
      if let Err(error) = statement.resolve(self) {
        return Err(error);
      }
    }
    Ok(())
  }

  #[named]
  pub fn resolve_local(&mut self, name: &Token) -> Option<usize> {
    for (index, scope) in self.scopes.iter().enumerate().rev() {
      if scope.contains_key(&name.lexeme) {
        return Some(self.scopes.len() - 1 - index);
      }
    }
    None
  }
}
