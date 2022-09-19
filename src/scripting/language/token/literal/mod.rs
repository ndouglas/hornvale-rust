use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenLiteral {
  String(String),
  Number(f64),
}

impl fmt::Display for TokenLiteral {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}
