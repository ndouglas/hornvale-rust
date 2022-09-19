use std::collections::HashMap;
use std::sync::Mutex;
use super::TokenType;
use TokenType::*;

lazy_static! {
  pub static ref TOKEN_KEYWORDS: Mutex<HashMap<&'static str, TokenType>> = Mutex::new(HashMap::from([
    ("and", And),
    ("class", Class),
    ("else", Else),
    ("false", False),
    ("fun", Function),
    ("for", For),
    ("if", If),
    ("nil", Nil),
    ("or", Or),
    ("print", Print),
    ("return", Return),
    ("super", Super),
    ("this", This),
    ("true", True),
    ("var", Var),
    ("while", While),
  ])); 
}

