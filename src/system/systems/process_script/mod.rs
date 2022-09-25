use colored::*;
use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::event::{ErrorEvent, OutputEvent, ScriptEvent};

pub struct ProcessScriptSystem {
  pub reader_id: ReaderId<ScriptEvent>,
}

impl ProcessScriptSystem {}

#[derive(SystemData)]
pub struct ProcessScriptSystemData<'a> {
  pub entities: Entities<'a>,
  pub error_event_channel: Write<'a, EventChannel<ErrorEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
  pub script_event_channel: Read<'a, EventChannel<ScriptEvent>>,
}

impl<'a> System<'a> for ProcessScriptSystem {
  type SystemData = ProcessScriptSystemData<'a>;
  #[named]
  fn run(&mut self, mut data: Self::SystemData) {
    trace_enter!();
    let script_events = data
      .script_event_channel
      .read(&mut self.reader_id)
      .map(|event| event.clone())
      .collect::<Vec<ScriptEvent>>();
    let event_count = script_events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} script event(s)...", event_count);
    for event in script_events.iter() {
      debug!("Processing next script event {:?}", event);
      let ScriptEvent { script } = event;
      self.interpret(script, &mut data);
    }
    trace_exit!();
  }

}

mod interpret;
