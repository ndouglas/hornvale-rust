use std::io::{Error, ErrorKind};

pub mod interpreter;
pub use interpreter::*;
pub mod parser;
pub use parser::*;
pub mod scanner;
pub use scanner::*;
pub mod token;
pub use token::*;
pub mod value;
pub use value::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct ScriptingLanguage {
  pub had_error: bool,
}

impl ScriptingLanguage {
  #[named]
  pub fn new() -> Self {
    Self { had_error: false }
  }

  #[named]
  pub fn run(&mut self, source: &str) -> Result<(), Error> {
    trace_enter!();
    let tokens: Vec<Token> = {
      let mut scanner: Scanner = Scanner::new(source, self);
      scanner.scan_tokens()
    };
    info!("Tokens: {:?}", tokens);
    let mut parser = Parser::new(tokens, self);
    let parse_response = parser.parse();
    match parse_response {
      Ok(ref expression) => {
        info!("Abstract Syntax Tree: {}", &expression.print_ast());
      },
      Err(ref error) => {
        error!("Parse Error: {:?}", &error);
        return Err(Error::new(ErrorKind::Other, format!("Error: {:?}", *error)));
      },
    }
    let expression = parse_response.unwrap();
    let mut interpreter = Interpreter::new();
    let answer = interpreter.interpret(expression);
    match answer {
      Ok(value) => {
        info!("Answer: {:?}", value);
      },
      Err(ref error) => {
        error!("Runtime Error: {:?}", &error);
        return Err(Error::new(ErrorKind::Other, format!("Error: {:?}", *error)));
      },
    }
    trace_exit!();
    Ok(())
  }

  #[named]
  pub fn interpret(&mut self, line: &str) -> Result<(), Error> {
    trace_enter!();
    let result = self.run(line);
    trace_exit!();
    result
  }

  #[named]
  pub fn report_error(&mut self, line_number: usize, location: Option<&str>, message: &str) {
    trace_enter!();
    let location_str = if let Some(location_str) = location {
      location_str
    } else {
      ""
    };
    error!("[line {}] Error{}: {}", line_number, location_str, message);
    self.had_error = true;
    trace_exit!();
  }
}
