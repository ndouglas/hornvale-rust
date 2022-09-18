use specs::shrev::EventChannel;

use crate::command::Command;
use crate::event::OutputEvent;

use super::*;

impl<'a> ProcessCommandSystem {
  #[named]
  pub fn process_echo(&mut self, command: &Command, output_event_channel: &mut Write<'a, EventChannel<OutputEvent>>) {
    trace_enter!();
    if let Command::Echo { string, .. } = command {
      output_event_channel.single_write(OutputEvent {
        string: string.to_string(),
      });
    }
    trace_exit!();
  }
}
