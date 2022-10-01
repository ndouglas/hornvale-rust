use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::scripting::language::environment::Environment;
use crate::scripting::language::script_error::ScriptError;
use crate::scripting::language::value::Value;
use crate::system::systems::process_script::ProcessScriptSystemData;

#[derive(Clone)]
pub struct NativeFunction(
  pub fn(&Rc<RefCell<Environment>>, &mut ProcessScriptSystemData, &Vec<Value>) -> Result<Value, ScriptError>,
);

impl fmt::Debug for NativeFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "<NativeFunction>")
  }
}
