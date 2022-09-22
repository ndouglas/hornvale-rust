use std::fmt;

pub mod literal;
pub use literal::*;
pub mod r#type;
pub use r#type::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
  pub r#type: TokenType,
  pub lexeme: String,
  pub literal: Option<TokenLiteral>,
  pub line_number: usize,
}

impl Token {
  pub fn new(r#type: TokenType, lexeme: &str, literal: Option<TokenLiteral>, line_number: usize) -> Self {
    Self {
      r#type,
      lexeme: lexeme.to_string(),
      literal,
      line_number,
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {} {:?}", self.r#type, self.lexeme, self.literal)
  }
}
