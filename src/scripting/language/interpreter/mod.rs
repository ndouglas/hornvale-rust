use std::collections::HashMap;
use std::io::Error;

use crate::scripting::language::callable::native_function::NativeFunction;
use crate::scripting::language::callable::{Callable, CallableKind};
use crate::scripting::language::environment::Environment;
use crate::scripting::language::parser::statement::Statement;
use crate::scripting::language::value::Value;
use crate::system::systems::process_script::ProcessScriptSystemData;

#[derive(Debug)]
pub struct Interpreter {
  pub environment: Environment,
  pub globals: Environment,
}

impl Interpreter {
  #[named]
  pub fn new() -> Self {
    let globals = Environment::new(None);
    let environment = Environment::new(None);
    Self { environment, globals }
  }

  #[named]
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

  #[named]
  pub fn pop_env(&mut self) {
    let current = self.environment.parent.take().unwrap();
    self.environment = *current;
  }

  #[named]
  pub fn define_globals(&mut self) {
    self.globals.define(
      "clock",
      Value::Callable(Callable {
        name: "clock".into(),
        arity: 0,
        kind: CallableKind::NativeFunction(NativeFunction(|_, _, _| { Ok(Value::Number(3.0)) })),
      }),
    );
  }

  #[named]
  pub fn interpret<'a>(
    &mut self,
    statements: Vec<Statement>,
    data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<(), Error> {
    self.define_globals();
    for statement in statements {
      let evaluation = statement.evaluate(self, data);
      if let Err(error) = evaluation {
        return Ok(());
      }
    }
    Ok(())
  }
}
