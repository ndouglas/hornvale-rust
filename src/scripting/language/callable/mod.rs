use std::io::{Error, ErrorKind};

use crate::scripting::language::interpreter::Interpreter;
use crate::scripting::language::parser::statement::Statement;
use crate::scripting::language::value::Value;
use crate::system::systems::process_script::ProcessScriptSystemData;

pub mod native_function;
pub use native_function::*;

#[derive(Clone, Debug)]
pub struct Callable {
  pub name: String,
  pub arity: usize,
  pub kind: CallableKind,
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
    interpreter: &mut Interpreter,
    data: &mut ProcessScriptSystemData<'a>,
    arguments: &Vec<Value>,
  ) -> Result<Value, Error> {
    use CallableKind::*;
    interpreter.push_env();
    let result = match &self.kind {
      NativeFunction(function) => (function.0)(interpreter, data, arguments),
      DeclaredFunction(declaration) => {
        if let Statement::Function { parameters, body, .. } = declaration {
          for (index, parameter) in parameters.iter().enumerate() {
            if let Some(value) = arguments.get(index) {
              interpreter.environment.define(&parameter.lexeme, value.clone());
            }
          }  
          body.evaluate(interpreter, data);
          Ok(Value::Nil)
        } else {
          Err(Error::new(ErrorKind::Other, "Function was not executable."))
        }
      },
    };
    interpreter.pop_env();
    result
  }
}
