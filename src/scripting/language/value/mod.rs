#[derive(Clone, Debug, PartialEq)]
pub enum Value {
  Any(Box<Value>),
  Boolean(bool),
  Number(f64),
  String(String),
  Nil,
}

impl Value {
  #[named]
  pub fn get_truthiness(&self) -> bool {
    use Value::*;
    match self {
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