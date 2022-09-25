use std::io::{Error, ErrorKind};

use crate::scripting::language::interpreter::Interpreter;
use crate::scripting::language::value::Value;
use crate::system::systems::process_script::ProcessScriptSystemData;

pub mod native_function;
pub use native_function::*;

#[derive(Clone, Debug)]
pub enum Callable {
  NativeFunction(NativeFunction),
}

impl Callable {

  #[named]
  pub fn call<'a>(&self, interpreter: &Interpreter, data: &mut ProcessScriptSystemData<'a>, arguments: &Vec<Value>) -> Result<Value, Error> {
    Ok(Value::Nil)
  }

  #[named]
  pub fn get_name(&self) -> String {
    "Steve".to_owned()
  }

}
