use crate::scripting::language::token::{Token, TokenLiteral};

#[derive(Clone, Debug)]
pub enum Expression {
  Unary {
    operator: Token,
    right: Box<Expression>,
  },
  Binary { 
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
  },
  Grouping {
    expression: Box<Expression>,
  },
  Literal {
    value: Option<TokenLiteral>,
  },
}

impl Expression {

  pub fn print_ast(&self) -> String {
    use Expression::*;
    match self {
      Unary { operator, right } => self.parenthesize(&operator.lexeme, &vec![(*right).clone()]),
      Binary { left, operator, right } => self.parenthesize(&operator.lexeme, &vec![(*left).clone(), (*right).clone()]),
      Grouping { expression } => self.parenthesize("group", &vec![(*expression).clone()]),
      Literal { value } => match value {
        Some(inner_value) => format!("{}", inner_value),
        None => "nil".to_string(),
      },
    }
  }

  pub fn parenthesize(&self, first: &str, rest: &Vec<Box<Expression>>) -> String {
    let mut result = String::default();
    result.push_str(&format!("({}", first));
    for expression in rest.iter() {
      result.push_str(&format!(" {}", expression.print_ast()));
    }
    result.push_str(")");
    result
  }

}