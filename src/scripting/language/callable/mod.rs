use std::cell::RefCell;
use std::rc::Rc;

use crate::scripting::language::environment::Environment;
use crate::scripting::language::interpreter::Interpreter;
use crate::scripting::language::parser::statement::Statement;
use crate::scripting::language::script_error::ScriptError;
use crate::scripting::language::value::Value;
use crate::system::systems::process_script::ProcessScriptSystemData;

pub mod native_function;
pub use native_function::*;

#[derive(Clone, Debug)]
pub struct Callable {
  pub name: String,
  pub arity: usize,
  pub kind: CallableKind,
  pub environment: Rc<RefCell<Environment>>,
}

#[derive(Clone, Debug)]
pub enum CallableKind {
  NativeFunction(NativeFunction),
  DeclaredFunction(Statement),
}

impl Callable {
  #[named]
  pub fn call<'a>(
    &self,
    interpreter: &Interpreter,
    data: &mut ProcessScriptSystemData<'a>,
    arguments: &Vec<Value>,
  ) -> Result<Value, ScriptError> {
    use CallableKind::*;
    let result = match &self.kind {
      NativeFunction(function) => (function.0)(&self.environment, data, arguments),
      DeclaredFunction(declaration) => {
        if let Statement::Function { parameters, body, .. } = declaration {
          let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.environment)))));
          for (index, parameter) in parameters.iter().enumerate() {
            if let Some(value) = arguments.get(index) {
              environment.borrow_mut().define(&parameter, value);
            }
          }
          let result = body.evaluate(interpreter, &environment, data);
          result?;
          Ok(Value::Nil)
        } else {
          Err(ScriptError::Error {
            token: None,
            message: "Function was not executable.".to_string(),
          })
        }
      },
    };
    result
  }
}
