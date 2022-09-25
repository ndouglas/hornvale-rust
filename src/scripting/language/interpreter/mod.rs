use std::collections::HashMap;
use std::io::Error;

use crate::scripting::language::environment::Environment;
use crate::scripting::language::parser::statement::Statement;
use crate::system::systems::process_script::ProcessScriptSystemData;

#[derive(Debug)]
pub struct Interpreter {
  pub environment: Environment,
}

impl<'a> Interpreter {
  pub fn new() -> Self {
    let environment = Environment::new(None);
    Self { environment }
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

  pub fn interpret(&mut self, statements: Vec<Statement>, data: &mut ProcessScriptSystemData<'a>) -> Result<(), Error> {
    for statement in statements {
      if let Err(error) = statement.evaluate(self, data) {
        return Err(error);
      }
    }
    Ok(())
  }
}
