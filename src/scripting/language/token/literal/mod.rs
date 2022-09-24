use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenLiteral {
  Boolean(bool),
  String(String),
  Number(f64),
  Nil,
}

impl fmt::Display for TokenLiteral {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
