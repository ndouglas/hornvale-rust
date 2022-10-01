use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::scripting::language::callable::native_function::NativeFunction;
use crate::scripting::language::callable::{Callable, CallableKind};
use crate::scripting::language::environment::Environment;
use crate::scripting::language::parser::statement::Statement;
use crate::scripting::language::script_error::ScriptError;
use crate::scripting::language::token::{Token, TokenType};
use crate::scripting::language::value::Value;
use crate::system::systems::process_script::ProcessScriptSystemData;

#[derive(Debug)]
pub struct Interpreter {
  pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
  #[named]
  pub fn new() -> Self {
    let environment = Rc::new(RefCell::new(Environment::new(None)));
    Self { environment }
  }

  #[named]
  pub fn push_env(&mut self) {
    self.environment = Rc::new(RefCell::new(Environment {
      values: HashMap::new(),
      parent: Some(Rc::new(RefCell::new(std::mem::replace(
        &mut self.environment.take(),
        Environment {
          values: HashMap::new(),
          parent: None,
        },
      )))),
    }));
  }

  #[named]
  pub fn pop_env(&mut self) {
    let current = self.environment.take().parent.take().unwrap();
    self.environment = current;
  }

  #[named]
  pub fn define_globals(&mut self) {
    self.environment.borrow_mut().define_global(
      &Token::new(TokenType::Identifier, "clock", None, 0),
      Value::Callable(Callable {
        name: "clock".into(),
        arity: 0,
        kind: CallableKind::NativeFunction(NativeFunction(|_, _, _| Ok(Value::Number(3.0)))),
      }),
    );
  }

  #[named]
  pub fn interpret<'a>(
    &mut self,
    statements: Vec<Statement>,
    data: &mut ProcessScriptSystemData<'a>,
  ) -> Result<(), ScriptError> {
    self.define_globals();
    for statement in statements {
      let evaluation = statement.evaluate(self, data);
      if let Err(_error) = evaluation {
        return Ok(());
      }
    }
    Ok(())
  }
}
