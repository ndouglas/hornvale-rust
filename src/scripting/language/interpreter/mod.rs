use std::collections::HashMap;
use std::io::Error;

use crate::scripting::language::environment::Environment;
use crate::scripting::language::parser::statement::Statement;
use crate::scripting::language::ScriptingLanguage;

#[derive(Debug)]
pub struct Interpreter<'a> {
  pub owner: &'a mut ScriptingLanguage,
  pub environment: Environment,
}

impl<'a> Interpreter<'a> {
  pub fn new(owner: &'a mut ScriptingLanguage) -> Self {
    let environment = Environment::new(None);
    Self { owner, environment }
  }

  pub fn push_env(&mut self) {
    self.environment = Environment {
      values: HashMap::new(),
      parent: Some(Box::new(std::mem::replace(
        &mut self.environment,
        Environment {
          values: HashMap::new(),
          parent: None,
        },
      ))),
    };
  }

  pub fn pop_env(&mut self) {
    let current = self.environment.parent.take().unwrap();
    self.environment = *current;
  }

  pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), Error> {
    for statement in statements {
      if let Err(error) = statement.evaluate(self) {
        return Err(error);
      }
    }
    Ok(())
  }
}
