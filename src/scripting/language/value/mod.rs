use std::fmt;

use crate::scripting::language::callable::Callable;

#[derive(Clone, Debug)]
pub enum Value {
  Any(Box<Value>),
  Boolean(bool),
  Number(f64),
  String(String),
  Callable(Callable),
  Nil,
}

impl Value {
  #[named]
  pub fn is_truthy(&self) -> bool {
    use Value::*;
    match self {
      Any(boxed_value) => boxed_value.is_truthy(),
      &Boolean(boolean) => boolean,
      Nil => false,
      _ => true,
    }
  }

  #[named]
  pub fn is_equal(&self, other: &Value) -> bool {
    use Value::*;
    match (self, other) {
      (a, Any(b)) => a.is_equal(b),
      (Any(a), b) => a.is_equal(b),
      (Nil, Nil) => true,
      (Nil, _) => false,
      (Boolean(a), Boolean(b)) => a == b,
      (Number(a), Number(b)) => a == b,
      (String(a), String(b)) => a == b,
      _ => false,
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    use Value::*;
    match self {
      Any(value) => write!(formatter, "{}", *value),
      Boolean(value) => write!(formatter, "{}", if *value { "true" } else { "false" }),
      Callable(value) => write!(formatter, "{}", value.get_name()),
      Number(value) => write!(formatter, "{}", value),
      String(value) => write!(formatter, "{}", value),
      Nil => write!(formatter, "{}", "nil"),
    }
  }
}
