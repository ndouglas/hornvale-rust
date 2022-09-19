pub mod scanner;
pub use scanner::*;
pub mod token;
pub use token::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct ScriptingLanguage {
  pub had_error: bool,
}

impl ScriptingLanguage {

  #[named]
  pub fn new() -> Self {
    Self {
      had_error: false,
    }
  }

  #[named]
  pub fn run(&mut self, source: &str) -> Result<(), ()> {
    trace_enter!();
    let tokens: Vec<Token> = {
      let mut scanner: Scanner = Scanner::new(source, self);
      scanner.scan_tokens()
    };
    let result = match self.had_error {
      true => Err(()),
      false => Ok(()), 
    };
    trace_exit!();
    result
  }

  #[named]
  pub fn interpret(&mut self, line: &str) -> Result<(), ()> {
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