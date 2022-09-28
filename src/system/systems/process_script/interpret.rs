use std::io::{Error, ErrorKind};

use crate::scripting::language::interpreter::Interpreter;
use crate::scripting::language::parser::Parser;
use crate::scripting::language::scanner::Scanner;

use super::*;

impl<'a> ProcessScriptSystem {
  #[named]
  pub fn interpret(&self, script: &str, data: &mut ProcessScriptSystemData<'a>) {
    trace_enter!();
    let token_result = {
      let mut scanner: Scanner = Scanner::new(script);
      scanner.scan_tokens(data)
    };
    if let Err(_error) = token_result {
      return;
    }
    let tokens = token_result.unwrap();
    info!("Tokens: {:?}", tokens);
    let mut parser = Parser::new(tokens);
    let parse_response = parser.parse();
    match parse_response {
      Ok(_) => {},
      Err(ref error) => {
        error!("Parse Error: {:?}", &error);
        let new_error = Error::new(ErrorKind::Other, format!("Parse Error: {:?}", error));
        data.output_event_channel.single_write(OutputEvent {
          string: format!("{}", new_error.to_string().red()),
        });
        return;
      },
    }
    let expressions = parse_response.unwrap();
    let mut interpreter = Interpreter::new();
    match interpreter.interpret(expressions, data) {
      Ok(()) => {},
      Err(error) => {
        error!("Runtime Error: {:?}", &error);
        let new_error = Error::new(ErrorKind::Other, format!("Runtime Error: {:?}", error));
        data.output_event_channel.single_write(OutputEvent {
          string: format!("{}", new_error.to_string().red()),
        });
        return;
      },
    }
    trace_exit!();
  }
}
