use crate::scripting::language::token::Token;
use crate::scripting::language::value::Value;

#[derive(Clone, Debug)]
pub enum ScriptError {
  Error { token: Option<Token>, message: String },
  Return { token: Token, value: Option<Value> },
}
