use std::io::{Error, ErrorKind};

pub mod expression;
pub use expression::*;
pub mod statement;
pub use statement::*;

use crate::scripting::language::token::{Token, TokenLiteral, TokenType};
use crate::scripting::language::ScriptingLanguage;

use Expression::*;
use TokenType::*;

pub struct Parser<'a> {
  pub tokens: Vec<Token>,
  pub current: usize,
  pub owner: &'a mut ScriptingLanguage,
}

impl<'a> Parser<'a> {
  #[named]
  pub fn new(tokens: Vec<Token>, owner: &'a mut ScriptingLanguage) -> Self {
    Self {
      tokens,
      current: 0,
      owner,
    }
  }

  #[named]
  pub fn parse(&mut self) -> Result<Vec<Statement>, Error> {
    let mut statements = Vec::new();
    while !self.is_at_end() {
      match self.declaration() {
        Ok(declaration) => statements.push(declaration),
        Err(err) => {
          self.synchronize();
          return Err(err);
        },
      }
    }
    Ok(statements)
  }

  #[named]
  pub fn declaration(&mut self) -> Result<Statement, Error> {
    if self.r#match(vec![TokenType::Var]) {
      return self.var_declaration();
    }
    self.statement()
  }

  #[named]
  pub fn var_declaration(&mut self) -> Result<Statement, Error> {
    match self.consume(TokenType::Identifier, "Expected a variable name.") {
      Ok(name) => {
        let mut initializer = None;
        if self.r#match(vec![TokenType::Equal]) {
          match self.expression() {
            Ok(expression) => initializer = Some(expression),
            Err(error) => {
              return Err(error);
            }
          }
        }
        self.consume(TokenType::Semicolon, "Expected ';' after variable declaration.")?;
        Ok(Statement::Variable { name, initializer })
      },
      Err(error) => Err(error),
    }
  }

  #[named]
  pub fn statement(&mut self) -> Result<Statement, Error> {
    if self.r#match(vec![TokenType::Print]) {
      return self.print_statement();
    }
    self.expression_statement()
  }

  #[named]
  pub fn expression(&mut self) -> Result<Expression, Error> {
    self.assignment()
  }

  #[named]
  pub fn assignment(&mut self) -> Result<Expression, Error> {
    let result = self.equality()?;
    if self.r#match(vec![Equal]) {
      let _equals = self.previous();
      let value = self.assignment()?;
      match result {
        Variable { identifier } => Ok(Assignment { identifier, value: Box::new(value) }),
        _ => Err(Error::new(ErrorKind::Other, "Invalid assignment target.")),
      }
    } else {
      Ok(result)
    }
  }

  #[named]
  pub fn equality(&mut self) -> Result<Expression, Error> {
    let mut result = self.comparison()?;
    while self.r#match(vec![BangEqual, EqualEqual]) {
      let operator = self.previous();
      let right = self.comparison()?;
      result = Binary {
        left: Box::new(result),
        operator,
        right: Box::new(right),
      };
    }
    Ok(result)
  }

  #[named]
  pub fn comparison(&mut self) -> Result<Expression, Error> {
    let mut result = self.term()?;
    while self.r#match(vec![GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual]) {
      let operator = self.previous();
      let right = self.term()?;
      result = Binary {
        left: Box::new(result),
        operator,
        right: Box::new(right),
      };
    }
    Ok(result)
  }

  #[named]
  pub fn term(&mut self) -> Result<Expression, Error> {
    let mut result = self.factor()?;
    while self.r#match(vec![Minus, Plus]) {
      let operator = self.previous();
      let right = self.factor()?;
      result = Binary {
        left: Box::new(result),
        operator,
        right: Box::new(right),
      };
    }
    Ok(result)
  }

  #[named]
  pub fn factor(&mut self) -> Result<Expression, Error> {
    let mut result = self.unary()?;
    while self.r#match(vec![Slash, Star]) {
      let operator = self.previous();
      let right = self.unary()?;
      result = Binary {
        left: Box::new(result),
        operator,
        right: Box::new(right),
      };
    }
    Ok(result)
  }

  #[named]
  pub fn unary(&mut self) -> Result<Expression, Error> {
    while self.r#match(vec![Bang, Minus]) {
      let operator = self.previous();
      let right = self.unary()?;
      return Ok(Unary {
        operator,
        right: Box::new(right),
      });
    }
    self.primary()
  }

  #[named]
  pub fn primary(&mut self) -> Result<Expression, Error> {
    if self.r#match(vec![False]) {
      return Ok(Literal {
        value: Some(TokenLiteral::String("false".to_string())),
      });
    }
    if self.r#match(vec![True]) {
      return Ok(Literal {
        value: Some(TokenLiteral::String("true".to_string())),
      });
    }
    if self.r#match(vec![Nil]) {
      return Ok(Literal {
        value: Some(TokenLiteral::String("nil".to_string())),
      });
    }
    if self.r#match(vec![TokenType::Number, TokenType::String]) {
      return Ok(Literal {
        value: self.previous().literal,
      });
    }
    if self.r#match(vec![TokenType::Identifier]) {
      return Ok(Variable {
        identifier: self.previous(),
      });
    }
    if self.r#match(vec![LeftParenthesis]) {
      let expression = self.expression()?;
      self.consume(RightParenthesis, "Expect ')' after expression.")?;
      return Ok(Grouping {
        expression: Box::new(expression),
      });
    }
    Err(Error::new(ErrorKind::Other, "Expected expression!"))
  }

  #[named]
  pub fn print_statement(&mut self) -> Result<Statement, Error> {
    let expression = self.expression();
    match expression {
      Ok(value) => {
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Statement::Print(value))
      },
      Err(error) => Err(error),
    }
  }

  #[named]
  pub fn expression_statement(&mut self) -> Result<Statement, Error> {
    let expression = self.expression();
    match expression {
      Ok(value) => {
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Statement::Expression(value))
      },
      Err(error) => Err(error),
    }
  }

  #[named]
  pub fn consume(&mut self, r#type: TokenType, message: &'a str) -> Result<Token, Error> {
    if self.check(r#type) {
      return Ok(self.advance());
    }
    self.parse_error(self.peek(), Error::new(ErrorKind::Other, message))
  }

  #[named]
  pub fn parse_error(&mut self, token: Token, error: Error) -> Result<Token, Error> {
    self.owner.report_error(token.line_number, Some(&token.lexeme), &error.to_string());
    Err(error)
  }

  #[named]
  pub fn synchronize(&mut self) {
    self.advance();
    while !self.is_at_end() {
      if self.previous().r#type == Semicolon {
        return;
      }
      match self.peek().r#type {
        Class | For | Function | If | Print | Return | Var | While => return,
        _ => {},
      }
      self.advance();
    }
  }

  #[named]
  pub fn r#match(&mut self, types: Vec<TokenType>) -> bool {
    for r#type in types {
      if self.check(r#type) {
        self.advance();
        return true;
      }
    }
    false
  }

  #[named]
  pub fn check(&mut self, r#type: TokenType) -> bool {
    if self.is_at_end() {
      return false;
    }
    self.peek().r#type == r#type
  }

  #[named]
  pub fn advance(&mut self) -> Token {
    if !self.is_at_end() {
      self.current += 1;
    }
    self.previous()
  }

  #[named]
  pub fn is_at_end(&self) -> bool {
    self.peek().r#type == Eof
  }

  #[named]
  pub fn peek(&self) -> Token {
    self.tokens.get(self.current).unwrap().clone()
  }

  #[named]
  pub fn previous(&self) -> Token {
    self.tokens.get(self.current - 1).unwrap().clone()
  }
}
