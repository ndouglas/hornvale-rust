use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TokenType {
  LeftParenthesis,
  RightParenthesis,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  GreaterThan,
  GreaterThanOrEqual,
  LessThan,
  LessThanOrEqual,
  Identifier,
  String,
  Number,
  And,
  Class,
  Else,
  False,
  Function,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,
  Eof,
}

impl TokenType {
  #[named]
  pub fn get_all() -> Vec<TokenType> {
    use TokenType::*;
    return vec![
      LeftParenthesis,
      RightParenthesis,
      LeftBrace,
      RightBrace,
      Comma,
      Dot,
      Minus,
      Plus,
      Semicolon,
      Slash,
      Star,
      Bang,
      BangEqual,
      Equal,
      EqualEqual,
      GreaterThan,
      GreaterThanOrEqual,
      LessThan,
      LessThanOrEqual,
      Identifier,
      String,
      Number,
      And,
      Class,
      Else,
      False,
      Function,
      For,
      If,
      Nil,
      Or,
      Print,
      Return,
      Super,
      This,
      True,
      Var,
      While,
      Eof,
    ];
  }
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl FromStr for TokenType {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    use TokenType::*;
    match string {
      "and" | "&&" => Ok(And),
      "class" => Ok(Class),
      "else" => Ok(Else),
      "false" => Ok(False),
      "fun" | "func" | "fn" | "function" => Ok(Function),
      "for" => Ok(For),
      "if" => Ok(If),
      "nil" => Ok(Nil),
      "or" | "||" => Ok(Or),
      "print" => Ok(Print),
      "return" => Ok(Return),
      "super" => Ok(Super),
      "this" => Ok(This),
      "true" => Ok(True),
      "var" | "let" => Ok(Var),
      "while" => Ok(While),
      unknown => Err(Error::new(ErrorKind::Other, format!("Unknown keyword {}!", unknown))),
    }
  }
}
