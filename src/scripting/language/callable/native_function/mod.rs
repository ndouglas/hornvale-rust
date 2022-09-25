use std::fmt;
use std::io::{Error, ErrorKind};

use crate::scripting::language::interpreter::Interpreter;
use crate::scripting::language::value::Value;
use crate::system::systems::process_script::ProcessScriptSystemData;

pub struct NativeFunction {
  pub name: String,
  pub arity: usize,
  pub callable: fn(&Interpreter, &mut ProcessScriptSystemData, &Vec<Value>) -> Result<Value, Error>,
}

impl Clone for NativeFunction {
  fn clone(&self) -> Self {
    NativeFunction {
      name: (&self.name).to_string(),
      arity: self.arity,
      callable: self.callable,
    }
  }
}

impl fmt::Debug for NativeFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Callable {}", self.name)
  }
}
