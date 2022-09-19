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
      if let Err(()) = interpreter.interpret(string) {
        data.output_event_channel.single_write(OutputEvent {
          string: format!("Encountered error parsing script... check logs for details."),
        });
      }
    }
    trace_exit!();
  }
}
