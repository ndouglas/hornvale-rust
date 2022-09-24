use crate::command::Command;
use crate::event::OutputEvent;
use crate::scripting::language::ScriptingLanguage;

use super::*;

impl<'a> ProcessCommandSystem {
  #[named]
  pub fn process_eval(&mut self, command: &Command, data: &mut ProcessCommandSystemData<'a>) {
    trace_enter!();
    if let Command::Eval { string, .. } = command {
      let mut interpreter = ScriptingLanguage::new();
      let outcome = interpreter.interpret(string);
      for string in interpreter.messages.iter() {
        data.output_event_channel.single_write(OutputEvent {
          string: string.to_string(),
        });
      }
      if let Err(error) = outcome {
        data.output_event_channel.single_write(OutputEvent {
          string: format!("Eval Error: {:?}", error),
        });
      }
    }
    trace_exit!();
  }
}
