use std::fmt;

use crate::scripting::language::ScriptingLanguage;
use crate::scripting::language::token::{TokenLiteral, Token, TokenType};

#[derive(Debug, PartialEq)]
pub struct Scanner<'a> {
  pub source: String,
  pub source_bytes: Vec<u8>,
  pub tokens: Vec<Token>,
  pub owner: &'a mut ScriptingLanguage,
  pub start: usize,
  pub current: usize,
  pub line_number: usize,
}

impl<'a> Scanner<'a> {

  pub fn new(source: &str, owner: &'a mut ScriptingLanguage) -> Self {
    let tokens = vec![];
    let owned_string = source.to_string();
    let source_bytes = owned_string.as_bytes().to_vec();
    Self {
      source: owned_string,
      source_bytes,
      tokens,
      owner,
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
  pub fn scan_tokens(&mut self) -> Vec<Token> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token();
    }
    self.tokens.push(Token::new(TokenType::Eof, "", None, self.line_number));
    self.tokens.clone()
  }

  #[named]
  pub fn scan_token(&mut self) {
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
      '/' => match self.match_current('/') {
        true => {
          while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
          }
        },
        false => self.add_token(Slash, None),
      },
      ' ' | '\r' | '\t' => {},
      '\n' => self.line_number += 1,
      '"' => self.match_string(),
      _ => match self.is_digit(char) {
        true => self.match_number(),
        false => self.owner.report_error(self.line_number, None, &format!("Unexpected character: {}", char)),
      },
    }
  }

  #[named]
  pub fn add_token(&mut self, r#type: TokenType, literal: Option<TokenLiteral>) {
    let lexeme = &self.source[self.start..self.current];
    self.tokens.push(Token::new(r#type,
      &lexeme,
      literal,
      self.line_number,
    ));
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
    self.add_token(TokenType::Number, Some(TokenLiteral::Number(value.parse::<f64>().unwrap())));
  }

  #[named]
  pub fn match_string(&mut self) {
    while self.peek() != '"' && !self.is_at_end() {
      if self.peek() == '\n' {
        self.line_number += 1;
      }
      self.advance();
    }
    if self.is_at_end() {
      self.owner.report_error(self.line_number, None, "Unterminated string.");
      return;
    }
    self.advance();
    let value = &self.source[self.start + 1..self.current - 1];
    self.add_token(TokenType::String, Some(TokenLiteral::String(value.to_string())));
  }

  #[named]
  pub fn is_digit(&self, char: char) -> bool {
    char >= '0' && char <= '9'
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

impl fmt::Display for Scanner<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "Scanner")
  }
}

