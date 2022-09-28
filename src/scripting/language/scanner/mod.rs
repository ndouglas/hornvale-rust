use std::fmt;
use std::str::FromStr;

use crate::scripting::language::script_error::ScriptError;
use crate::scripting::language::token::{Token, TokenLiteral, TokenType};
use crate::system::systems::process_script::ProcessScriptSystemData;

pub struct Scanner {
  pub source: String,
  pub source_bytes: Vec<u8>,
  pub tokens: Vec<Token>,
  pub start: usize,
  pub current: usize,
  pub line_number: usize,
}

impl Scanner {
  pub fn new(source: &str) -> Self {
    let tokens = vec![];
    let owned_string = source.to_string();
    let source_bytes = owned_string.as_bytes().to_vec();
    Self {
      source: owned_string,
      source_bytes,
      tokens,
      start: 0,
      current: 0,
      line_number: 0,
    }
  }

  #[named]
  pub fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  #[named]
  pub fn scan_tokens<'a>(&mut self, _data: &mut ProcessScriptSystemData<'a>) -> Result<Vec<Token>, ScriptError> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token()?;
    }
    self.tokens.push(Token::new(TokenType::Eof, "", None, self.line_number));
    Ok(self.tokens.clone())
  }

  #[named]
  pub fn scan_token(&mut self) -> Result<(), ScriptError> {
    let char = self.advance();
    use TokenType::*;
    match char {
      '(' => self.add_token(LeftParenthesis, None),
      ')' => self.add_token(RightParenthesis, None),
      '{' => self.add_token(LeftBrace, None),
      '}' => self.add_token(RightBrace, None),
      ',' => self.add_token(Comma, None),
      '.' => self.add_token(Dot, None),
      '-' => self.add_token(Minus, None),
      '+' => self.add_token(Plus, None),
      ';' => self.add_token(Semicolon, None),
      '*' => self.add_token(Star, None),
      '!' => match self.match_current('=') {
        true => self.add_token(BangEqual, None),
        false => self.add_token(Bang, None),
      },
      '=' => match self.match_current('=') {
        true => self.add_token(EqualEqual, None),
        false => self.add_token(Equal, None),
      },
      '>' => match self.match_current('=') {
        true => self.add_token(GreaterThanOrEqual, None),
        false => self.add_token(GreaterThan, None),
      },
      '<' => match self.match_current('=') {
        true => self.add_token(LessThanOrEqual, None),
        false => self.add_token(LessThan, None),
      },
      '/' => match self.peek() {
        '/' => self.match_line_comment(),
        '*' => self.match_multiline_comment(),
        _ => self.add_token(Slash, None),
      },
      ' ' | '\r' | '\t' => {},
      '\n' => self.line_number += 1,
      '"' => self.match_string()?,
      char if self.is_digit(char) => self.match_number(),
      char if self.is_alpha(char) => self.match_identifier(),
      _ => {
        return Err(ScriptError::Error {
          token: None,
          message: format!("Unexpected character: {}", char),
        })
      },
    }
    Ok(())
  }

  #[named]
  pub fn add_token(&mut self, r#type: TokenType, literal: Option<TokenLiteral>) {
    let lexeme = &self.source[self.start..self.current];
    self.tokens.push(Token::new(r#type, &lexeme, literal, self.line_number));
  }

  #[named]
  pub fn advance(&mut self) -> char {
    let position = self.current;
    self.current += 1;
    self.source_bytes[position] as char
  }

  #[named]
  pub fn match_current(&mut self, char: char) -> bool {
    if self.is_at_end() {
      return false;
    }
    if self.source_bytes[self.current] as char != char {
      return false;
    }
    self.current += 1;
    return true;
  }

  #[named]
  pub fn match_line_comment(&mut self) {
    while self.peek() != '\n' && !self.is_at_end() {
      self.advance();
    }
  }

  #[named]
  pub fn match_multiline_comment(&mut self) {
    while self.peek_next() != '*' && self.peek_at_offset(2) != '/' && !self.is_at_end() {
      self.advance();
    }
    self.advance();
    self.advance();
    self.advance();
  }

  #[named]
  pub fn match_number(&mut self) {
    while self.is_digit(self.peek()) {
      self.advance();
    }
    if self.peek() == '.' && self.is_digit(self.peek_next()) {
      self.advance();
      while self.is_digit(self.peek()) {
        self.advance();
      }
    }
    let value = &self.source[self.start..self.current];
    self.add_token(
      TokenType::Number,
      Some(TokenLiteral::Number(value.parse::<f64>().unwrap())),
    );
  }

  #[named]
  pub fn match_string(&mut self) -> Result<(), ScriptError> {
    while self.peek() != '"' && !self.is_at_end() {
      if self.peek() == '\n' {
        self.line_number += 1;
      }
      self.advance();
    }
    if self.is_at_end() {
      return Err(ScriptError::Error {
        token: None,
        message: format!("Unterminated string."),
      });
    }
    self.advance();
    let value = &self.source[self.start + 1..self.current - 1];
    self.add_token(TokenType::String, Some(TokenLiteral::String(value.to_string())));
    Ok(())
  }

  #[named]
  pub fn match_identifier(&mut self) {
    while self.is_alpha_numeric(self.peek()) {
      self.advance();
    }
    let value = &self.source[self.start..self.current];
    let value_type = match TokenType::from_str(value) {
      Ok(token_type) => token_type,
      Err(_) => TokenType::Identifier,
    };
    self.add_token(value_type, None);
  }

  #[named]
  pub fn is_digit(&self, char: char) -> bool {
    char >= '0' && char <= '9'
  }

  #[named]
  pub fn is_alpha(&self, char: char) -> bool {
    (char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z') || char == '_'
  }

  #[named]
  pub fn is_alpha_numeric(&self, char: char) -> bool {
    self.is_digit(char) || self.is_alpha(char)
  }

  #[named]
  pub fn peek(&self) -> char {
    self.peek_at_offset(0)
  }

  #[named]
  pub fn peek_next(&self) -> char {
    self.peek_at_offset(1)
  }

  #[named]
  pub fn peek_at_offset(&self, offset: usize) -> char {
    match self.current + offset >= self.source.len() {
      true => '\0',
      false => self.source_bytes[self.current + offset] as char,
    }
  }
}

impl fmt::Display for Scanner {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Scanner")
  }
}
