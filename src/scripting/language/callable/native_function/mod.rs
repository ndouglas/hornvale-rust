use std::fmt;

use crate::scripting::language::interpreter::Interpreter;
use crate::scripting::language::script_error::ScriptError;
use crate::scripting::language::value::Value;
use crate::system::systems::process_script::ProcessScriptSystemData;

#[derive(Clone)]
pub struct NativeFunction(
  pub fn(&Interpreter, &mut ProcessScriptSystemData, &Vec<Value>) -> Result<Value, ScriptError>,
);

impl fmt::Debug for NativeFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "<NativeFunction>")
  }
}
